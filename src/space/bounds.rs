use std::{ ops::Index};
use num::bounds::Bounded;
use na::{ Real, Scalar, Point3 };

macro_rules! zip_points {
    ($t:ty, $p0:expr, $p1:expr, $cb:expr) => {
        <$t>::from_coordinates($p0.coords.zip_map(&$p1.coords, $cb))
    }
}

/**
Bounding box
*/
#[derive(Debug, Copy, Clone)]
pub struct Bounds3<N: Scalar> {
    pub min: Point3<N>,
    pub max: Point3<N>
}

impl<N: Scalar> Bounds3<N> {
    /// Get the ith corner of the bounding box
    #[inline]
    pub fn corner(&self, i: u32) -> Point3<N> {
        Point3::new(
            self[if i & 1 > 0 { 1 } else { 0 }].x,
            self[if i & 2 > 0 { 1 } else { 0 }].y,
            self[if i & 4 > 0 { 1 } else { 0 }].z,
        )
    }
}

// Return one of the two corners
impl<N: Scalar> Index<u8> for Bounds3<N> {
    type Output = Point3<N>;
    #[inline]
    fn index(&self, index: u8) -> &Point3<N> {
        debug_assert!(index < 2);
        if index == 0 { &self.min } else { &self.max}
    }
}

impl<N: Scalar + Real> Bounds3<N> {
    /// Create a new bounding box with the minimum of two points
    #[inline]
    pub fn new(p0: Point3<N>, p1: Point3<N>) -> Bounds3<N> {
        Bounds3 {
            min: zip_points!(Point3<N>, p0, p1, |c0, c1| c0.min(c1)),
            max: zip_points!(Point3<N>, p0, p1, |c0, c1| c0.max(c1)),
        }
    }

    /// Find the intersection between two bounding boxes
    #[inline]
    pub fn intersection(&self, with: &Self) -> Self {
        Bounds3 {
            min: zip_points!(Point3<N>, self.min, with.min, |c0, c1| c0.max(c1)),
            max: zip_points!(Point3<N>, self.max, with.max, |c0, c1| c0.min(c1))
        }
    }

    /// Expand using another bounding box
    #[inline]
    pub fn union(&self, with: &Self) -> Self {
        Bounds3 {
            min: zip_points!(Point3<N>, self.min, with.min, |c0, c1| c0.min(c1)),
            max: zip_points!(Point3<N>, self.max, with.max, |c0, c1| c0.min(c1))
        }
    }

    /// Expand using another point
    #[inline]
    pub fn point_union(&self, with: &Point3<N>) -> Self {
        Bounds3 {
            min: zip_points!(Point3<N>, self.min, with, |c0, c1| c0.min(c1)),
            max: zip_points!(Point3<N>, self.max, with, |c0, c1| c0.max(c1))
        }
    }

    /// True if this instance overlaps with the given
    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.iter().zip(other.max.iter()).all(|(min, max)| min >= max) &&
        self.max.iter().zip(other.min.iter()).all(|(max, min)| max <= min)
    }

    /**
    Return true if the point is within the given bounds
    */
    #[inline]
    pub fn contains(&self, p: Point3<N>) -> bool {
        p.iter().zip(self.min.iter()).all(|(coord, min)| coord >= min) &&
        p.iter().zip(self.max.iter()).all(|(coord, max)| coord <= max)
    }

    /**
    Return true if the point is within the bounds but not at the max edges
    */
    #[inline]
    pub fn contains_exclusive(&self, p: Point3<N>) -> bool {
        p.iter().zip(self.min.iter()).all(|(coord, min)| coord >= min) &&
        p.iter().zip(self.max.iter()).all(|(coord, max)| coord < max)
    }

}

impl<N: Scalar + Bounded> Bounds3<N> {
    #[inline]
    pub fn infinite() -> Bounds3<N> {
        Bounds3 {
            min: Point3::min_value(),
            max: Point3::max_value()
        }
    }

    #[inline]
    pub fn none() -> Bounds3<N> {
        Bounds3 {
            min: Point3::max_value(),
            max: Point3::min_value()
        }
    }
}

/*
impl<N: Scalar + ClosedAdd + ClosedSub + ClosedMul> Bounds3<N> {

    /// Expand the bounds by a constant factor
    #[inline]
    pub fn expand(&self, delta: N) -> Self {
        let expansion = Vector3::repeat(delta);
        Bounds3 {
            min: self.min - expansion,
            max: self.max + expansion
        }
    }

    /// Get the vector from the min point to the max point
    #[inline]
    pub fn diagonal(&self) -> Vector3<N> {
        self.max - self.min
    }

    /// Get the surface area of the bounding box
    #[inline]
    pub fn surface_area(&self) -> N {
        let d = self.diagonal();
        let half = d.x * d.y + d.x * d.z + d.y * d.z;
        half + half
    }

    /// Get the volume of the bounding box
    #[inline]
    pub fn volume(&self) -> N {
        let d = self.diagonal();
        d.x * d.y * d.z
    }
}
*/
