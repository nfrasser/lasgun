use cgmath::prelude::*;
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
    pub dinv: Vector3<N>
}

impl<N: BaseFloat> Ray3<N> {
    pub fn new(origin: Point3<N>, d: Vector3<N>) -> Ray3<N> {
        let (zero, one) = (N::zero(), N::one());
        debug_assert!(d.x != zero || d.y != zero || d.z != zero);
        let dinv = Vector3::new(one/d.x, one/d.y, one/d.z);
        Ray3 { origin, d, dinv }
    }
}

pub mod primary;
