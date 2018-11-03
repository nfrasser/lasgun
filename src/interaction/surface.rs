use cgmath::{prelude::*, Point3, Vector3, BaseFloat };
use crate::{space::normal::Normal3, ray::Ray3, scene::MaterialRef};

/// Surface interaction retrived by casting a specific ray through a scene. The
/// `t` parameter is specified to compare previous parametric ray intersection
/// distances and avoid extra computation in some cases.
#[derive(Copy, Clone)]
pub struct SurfaceInteraction<N: BaseFloat> {
    /// Parametric distance to point of interaction based on ray origin
    pub t: N,

    /// Normal at interaction surface
    pub n: Normal3<N>,

    // Index-based reference to a material definition in the scene settings
    pub material: Option<MaterialRef>,

    /// Point of interaction in world coordinates
    p: Point3<N>,

    /// Incident direction vector at point of interaction based on ray
    /// definition
    d: Vector3<N>,

    /// Level of recursion for the ray that caused this interaction
    level: u32
}

impl<N: BaseFloat> SurfaceInteraction<N> {

    /// Initialize a basic new surface interaction. Note that this interaction
    /// is not valid until commit is called with a `Ray` instance (`p()` and
    /// `d()` methods return zero-values)
    pub fn new(t: N, n: Normal3<N>, material: Option<MaterialRef>) -> SurfaceInteraction<N> {
        SurfaceInteraction {
            t, n, material,
            p: Point3::from_value(N::zero()),
            d: Vector3::from_value(N::zero()),
            level: 0
        }
    }

    /// A default surface interaction that doesn't exist, with just a direction
    /// vector and a default material
    pub fn default() -> SurfaceInteraction<N> {
        SurfaceInteraction {
            t: N::infinity(),
            n: Normal3::new(N::zero(), N::zero(), N::zero()),
            material: None,
            d: Vector3::zero(),
            p: Point3::from_value(N::zero()),
            level: 0
        }
    }

    /// Commit to the current interaction state as being the closest point of
    /// interaction for the given ray. Call this once after scene node traversal
    /// is complete. Returns the resulting interaction point in world space. The
    /// interaction is valid once this method is called. Also normalizes
    /// everything.
    pub fn commit(&mut self, ray: &Ray3<N>) {
        self.n.normalize();
        self.p = ray.origin + ray.d*self.t;
        self.d = ray.d.normalize();
        self.level = ray.level;
    }

    /// Has in interaction been successfully found
    pub fn exists(&self) -> bool {
        if let Some(_) = self.material { true } else { false }
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

    /// Recursion level of ray causing this interaction. self must be committed
    pub fn level(&self) -> u32 {
        debug_assert!(self.valid());
        self.level
    }

    /// Whether this is a valid surface interaction (i.e., has been committed
    /// with a ray)
    fn valid(&self) -> bool {
        self.d != Vector3::zero()
    }
}
