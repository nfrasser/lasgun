use cgmath::{prelude::*, Point3, Vector3, BaseFloat };
use crate::{space::normal::Normal3, ray::Ray3, scene::MaterialRef};

/// Surface interaction retrived by casting a specific ray through a scene. The
/// `t` parameter is specified to compare previous parametric ray intersection
/// distances and avoid extra computation in some cases.
#[derive(Copy, Clone)]
pub struct SurfaceInteraction<N: BaseFloat> {
    /// Parametric distance to point of interaction based on ray origin
    pub t: N,

    /// Parametric differential ∂p/∂u at point of interaction
    pub dpdu: Vector3<N>,

    /// Parametric differential ∂p/∂v at point of interaction
    pub dpdv: Vector3<N>,

    // Index-based reference to a material definition in the scene settings
    pub material: Option<MaterialRef>,

    /// Normal at interaction surface
    n: Normal3<N>,

    /// Point of interaction in world coordinates
    p: Point3<N>,

    /// A small vector used to offset floating-point error from the point of
    /// interaction. Used to avoid speckling during the lighting/integration
    /// step. Parallel to the normal vector n.
    p_err: Vector3<N>,

    /// Incident direction vector at point of interaction based on ray
    /// definition
    d: Vector3<N>,
}

impl<N: BaseFloat> SurfaceInteraction<N> {

    /// Initialize a basic new surface interaction. Note that this interaction
    /// is not valid until commit is called with a `Ray` instance (`p()` and
    /// `d()` methods return zero-values)
    pub fn new(t: N, dpdu: Vector3<N>, dpdv: Vector3<N>, material: Option<MaterialRef>) -> SurfaceInteraction<N> {
        SurfaceInteraction {
            t, dpdu, dpdv, material,
            n: Normal3::zero(),
            p: Point3::from_value(N::zero()),
            p_err: Vector3::zero(),
            d: Vector3::zero()
        }
    }

    /// A default surface interaction that doesn't exist, with just a direction
    /// vector and a default material
    pub fn default() -> SurfaceInteraction<N> {
        SurfaceInteraction {
            t: N::infinity(),
            dpdu: Vector3::zero(),
            dpdv: Vector3::zero(),
            material: None,
            n: Normal3::zero(),
            p: Point3::from_value(N::zero()),
            p_err: Vector3::zero(),
            d: Vector3::zero()
        }
    }

    /// Commit to the current interaction state as being the closest point of
    /// interaction for the given ray. Call this once after scene node traversal
    /// is complete. Returns the resulting interaction point in world space. The
    /// interaction is valid once this method is called. Also normalizes
    /// everything.
    pub fn commit(&mut self, ray: &Ray3<N>) {
        self.dpdu = self.dpdu.normalize();
        self.dpdv = self.dpdv.normalize();
        self.n = Normal3(self.dpdu.cross(self.dpdv).normalize()).face_forward(ray.d);

        // Add a small fraction of the normal to avoid speckling due to
        // floating point errors (the calculated point ends up inside the
        // geometric primitive).
        let err = N::epsilon() * (N::one() + N::one()).powi(16);
        self.p = ray.origin + ray.d*self.t;
        self.p_err = self.n.0 * err;

        self.d = ray.d.normalize();
    }

    /// Has in interaction been successfully found
    pub fn exists(&self) -> bool {
        return self.material.is_some()
    }

    /// Normal at point of intersection. Must be committed
    pub fn n(&self) -> Vector3<N> {
        debug_assert!(self.valid());
        self.n.0
    }

    /// Incident direction vector. self must be committed
    pub fn d(&self) -> Vector3<N> {
        debug_assert!(self.valid());
        self.d
    }

    /// Point of interaction in world coordinates. self must be committed
    pub fn p(&self) -> Point3<N> {
        debug_assert!(self.valid());
        self.p
    }

    /// Floating point error offset from the intersection point p, parallel to
    /// normal n.
    pub fn p_err(&self) -> Vector3<N> {
        debug_assert!(self.valid());
        self.p_err
    }

    /// Whether this is a valid surface interaction (i.e., has been committed
    /// with a ray)
    fn valid(&self) -> bool {
        self.n.0 != Vector3::zero()
    }
}
