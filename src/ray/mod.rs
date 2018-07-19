use space::*;

pub struct Ray {
    /**
    Point at which ray originates
    */
    pub origin: Point,

    /**
    Unit vector representing ray directoin
    */
    pub d: Vector,

    /**
    Ray direction except each component is inverted
    Used for optimization
    */
    pub dinv: Vector
}

impl Ray {
    pub fn new(origin: Point, d: Vector) -> Ray {
        assert!(d.x != 0.0 || d.y != 0.0 || d.z != 0.0);
        let d = d.normalize();
        let dinv = inv(&d);
        Ray { origin, d, dinv }
    }
}

pub mod primary;
