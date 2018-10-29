use cgmath::{ BaseFloat, Point3, Vector3 };

/// The default ray is 3D uses double-precision units
pub type Ray = Ray3<f64>;

/// A generic three-dimensional ray
#[derive(Copy, Clone, Debug)]
pub struct Ray3<N: BaseFloat> {
    /**
    Point at which ray originates
    */
    pub origin: Point3<N>,

    /**
    Unit vector representing ray direction
    By convention, we guarantee that this is normalized
    */
    pub d: Vector3<N>,

    /**
    Ray direction except each component is inverted
    Used for optimizations
    */
    pub dinv: Vector3<N>,

    /// Level of recursion for reflective materials
    pub level: u32,

    /// Whether the origin is in a medium. False if in a vacuum
    /// TODO: Make this a proper medium trait/ref
    pub medium: bool
}

impl<N: BaseFloat> Ray3<N> {
    pub fn new(origin: Point3<N>, d: Vector3<N>, level: u32, medium: bool) -> Ray3<N> {
        let (zero, one) = (N::zero(), N::one());
        debug_assert!(d.x != zero || d.y != zero || d.z != zero);
        let dinv = Vector3::new(one/d.x, one/d.y, one/d.z);
        // TODO: Custom recursion level
        Ray3 { origin, d, dinv, level, medium }
    }

    /// Create a new ray refacted at the given origin in the given direction
    /// Returns None if the recursion level has reached zero
    pub fn reflect(ray: &Ray3<N>, origin: Point3<N>, d: Vector3<N>) -> Option<Ray3<N>> {
        if ray.level == 0 { None }
        else { Some(Ray3::new(origin, d, ray.level - 1, ray.medium)) }
    }

    /// Create a new ray refacted at the given origin in the given direction
    /// Returns None if the recursion level has reached zero
    pub fn refract(ray: &Ray3<N>, origin: Point3<N>, d: Vector3<N>) -> Option<Ray3<N>> {
        if ray.level == 0 { None }
        else { Some(Ray3::new(origin, d, ray.level - 1, !ray.medium)) }
    }
}

pub mod primary;
