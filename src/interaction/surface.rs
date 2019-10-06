use cgmath::{prelude::*, Point2, Point3, Vector3, BaseFloat };
use crate::{space::normal::Normal3, ray::Ray3, Material};

/// Collection of shading parameters, used for either geometry or surface
/// shading.
///
/// When used for surface shading (not geometric shading) it's important that
/// the normal created by taking the cross-product of dpdu and dpdv points
/// outside the associated bounding volume.
#[derive(Debug, Copy, Clone)]
pub struct Shading<N: BaseFloat> {
    /// Parametric differential ∂p/∂u at point of interaction. Combined with
    /// ∂p/∂v, represents change in the surface intersection point given a small
    /// change in the (u, v) parametric texture coordinates.
    pub dpdu: Vector3<N>,

    /// Parametric differential ∂p/∂v at point of interaction.
    pub dpdv: Vector3<N>,

    // TODO: Implement these
    // pub dndu: Vector3<N>,
    // pub dndv: Vector3<N>
}


/// Intermediate data structure retrived by casting a specific ray through a
/// scene. The `t` parameter is specified to compare previous parametric ray
/// intersection distances and avoid extra computation in some cases.
///
/// Transformed as the ray traverses the scene. Used to create normalized
/// `SurfaceInteraction` instances.
#[derive(Debug, Copy, Clone)]
pub struct RayIntersection<N: BaseFloat> {
    /// Ray equation parameter used to determine point of intersection
    pub t: N,

    /// Texture UV, each in range [0, 1] coordinates. TODO: Actually use this
    pub uv: Point2<N>,

    /// Base geometry shading
    pub geometry: Shading<N>,

    /// Special surface shading. Taking the cross-product of dpdu and dpdv
    /// should always yield
    pub surface: Shading<N>,

    /// Material at surface interaction. Use this when the shape doesn't provide
    /// a material on its own.
    pub material: Material,

    /// Optional authoritative shading normal, to be used instead of surface
    /// shading parameters for some shapes
    pub n: Option<Normal3<N>>,
}

impl<N: BaseFloat> RayIntersection<N> {
    pub fn new(t: N, uv: Point2<N>, dpdu: Vector3<N>, dpdv: Vector3<N>) -> Self {
        let geometry = Shading { dpdu, dpdv };
        let material = Material::default();
        // Surface shading is copied geometry
        RayIntersection { t, uv, geometry, surface: geometry, material, n: None }
    }

    /// Create a non-existent ray intersection that will be populated later
    pub fn default() -> Self {
        Self::new(
            N::infinity(),
            Point2::new(N::zero(), N::zero()),
            Vector3::zero(),
            Vector3::zero()
        )
    }

    /// Reassign the surface shading to something different.
    pub fn set_surface_shading(&mut self, dpdu: Vector3<N>, dpdv: Vector3<N>) {
        self.surface.dpdu = dpdu;
        self.surface.dpdv = dpdv;
    }

    /// Reset the default material, to use when the shape of intersection
    /// doesn't provide one.
    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    /// Flip orientation of the normals resulting from the cross-product of the
    /// shading coordinates.
    pub fn swap_backface(&mut self) {
        let (dpdu, dpdv) = (self.geometry.dpdu, self.geometry.dpdv);
        self.geometry.dpdu = dpdv;
        self.geometry.dpdv = dpdu;

        let (dpdu, dpdv) = (self.surface.dpdu, self.surface.dpdv);
        self.surface.dpdu = dpdv;
        self.surface.dpdv = dpdu;

        // Swap custom shading normal
        if let Some(n) = self.n { self.n = Some(-n) }
    }

    /// Whether an intersection exists
    pub fn exists(&self) -> bool {
        self.t > N::zero() && self.t != N::infinity()
    }

    #[inline]
    pub fn ng(&self) -> Vector3<N> {
        self.geometry.dpdu.cross(self.geometry.dpdv).normalize()
    }

    #[inline]
    pub fn ns(&self) -> Vector3<N> {
        if let Some(n) = self.n {
            n.0.normalize()
        } else {
            self.surface.dpdu.cross(self.surface.dpdv).normalize()
        }
    }
}

/// Describes light interaction at point p in the outgoing direction wo.
/// Generated from normalized `RayIntersection` data. Used to determine light
/// BSDF light scattering for the intersection point.
#[derive(Debug, Copy, Clone)]
pub struct SurfaceInteraction<N: BaseFloat> {
    /// Point of interaction in world coordinates
    pub p: Point3<N>,

    /// A small vector used to offset floating-point error from the point of
    /// interaction. Used to avoid speckling during the lighting/integration
    /// step. Parallel to the normal vector n.
    pub p_err: Vector3<N>,

    /// Outgoing direction vector at point of interaction based on ray
    /// definition. Points from `p` to the ray's origin; reversed `ray.d`.
    pub wo: Vector3<N>,

    /// Geometric shading normal. e.g., perpendicular to plane that a triangle
    /// lies in. Always points in the same hemisphere as ray origin.
    pub ng: Normal3<N>,

    /// Surface shading normal. e.g., from interpolating the mesh-provided
    /// normals at each vertex. Always points towards outside of bounding volume.
    pub ns: Normal3<N>,

    /// Normalized geometric shading parameters
    pub geometry: Shading<N>,

    /// Normalized surface shading parameters
    pub surface: Shading<N>,
}

impl<N: BaseFloat> SurfaceInteraction<N> {

    /// Initialize a basic new surface interaction. Note that this interaction
    /// is not valid until commit is called with a `Ray` instance (`p()` and
    /// `d()` methods return zero-values)
    pub fn from(ray: &Ray3<N>, isect: &RayIntersection<N>) -> Self {
        debug_assert!(isect.exists());

        let wo = -ray.d.normalize();
        let ng = Normal3(isect.ng()).face_forward(wo);
        let ns = Normal3(isect.ns());

        // Add a small fraction of the normal to avoid speckling due to
        // floating point errors (the calculated point ends up inside the
        // geometric primitive).
        let err = N::epsilon() * (N::one() + N::one()).powi(16);
        let p = ray.origin + ray.d*isect.t;
        let p_err = ng.0 * err;

        SurfaceInteraction {
            p, p_err, wo, ng, ns,
            geometry: Shading {
                dpdu: isect.geometry.dpdu.normalize(),
                dpdv: isect.geometry.dpdv.normalize(),
            },
            surface: Shading {
                dpdu: isect.surface.dpdu.normalize(),
                dpdv: isect.surface.dpdv.normalize(),
            }
        }
    }

    #[inline] pub fn ng(&self) -> Vector3<N> { self.ng.0 }
    #[inline] pub fn ns(&self) -> Vector3<N> { self.ns.0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let ray: Ray3<f64> = Ray3::new(Point3::new(0.0, 0.0, 1.0), -Vector3::unit_z());
        let isect = RayIntersection::new(1.0, Point2::new(0.0, 0.0), Vector3::unit_x(), Vector3::unit_y());
        let interaction = SurfaceInteraction::from(&ray, &isect);

        assert_eq!(interaction.ng(), Vector3::unit_z());
    }
}
