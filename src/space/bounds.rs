use std::{ ops::Index };
use cgmath::prelude::*;
use cgmath::{ Vector3, Point3, BaseNum, BaseFloat, Bounded };

/// Bounding box
#[derive(Debug, Copy, Clone)]
pub struct Bounds3<S: BaseNum> {
    pub min: Point3<S>,
    pub max: Point3<S>
}

impl<S: BaseNum> Bounds3<S> {
    /// Get the ith corner of the bounding box
    #[inline]
    pub fn corner(&self, i: u32) -> Point3<S> {
        Point3::new(
            self[if i & 1 > 0 { 1 } else { 0 }].x,
            self[if i & 2 > 0 { 1 } else { 0 }].y,
            self[if i & 4 > 0 { 1 } else { 0 }].z,
        )
    }
}

/// Return one of the two corners
impl<S: BaseNum> Index<u8> for Bounds3<S> {
    type Output = Point3<S>;
    #[inline]
    fn index(&self, index: u8) -> &Point3<S> {
        debug_assert!(index < 2);
        if index == 0 { &self.min } else { &self.max}
    }
}

impl<S: BaseNum> Bounds3<S> {
    /// Create a new bounding box with the minimum of two points
    #[inline]
    pub fn new(p0: Point3<S>, p1: Point3<S>) -> Bounds3<S> {
        Bounds3 {
            min: zip_points!(p0, p1, min),
            max: zip_points!(p0, p1, max),
        }
    }

    /// Find the intersection between two bounding boxes
    #[inline]
    pub fn intersection(&self, with: &Self) -> Self {
        Bounds3 {
            min: zip_points!(self.min, with.min, max),
            max: zip_points!(self.max, with.max, min)
        }
    }

    /// Expand using another bounding box
    #[inline]
    pub fn union(&self, with: &Self) -> Self {
        Bounds3 {
            min: zip_points!(self.min, with.min, min),
            max: zip_points!(self.max, with.max, max)
        }
    }

    /// Expand using another point
    #[inline]
    pub fn point_union(&self, with: &Point3<S>) -> Self {
        Bounds3 {
            min: zip_points!(self.min, with, min),
            max: zip_points!(self.max, with, max)
        }
    }

    /// True if this instance overlaps with the given
    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        all_coords_match!(self.min, other.max, |min, max| min >= max) &&
        all_coords_match!(self.max, other.min, |max, min| max <= min)
    }

    /// Return true if the point is within the given bounds
    #[inline]
    pub fn contains(&self, p: Point3<S>) -> bool {
        all_coords_match!(p, self.min, |coord, min| coord >= min) &&
        all_coords_match!(p, self.max, |coord, max| coord >= max)
    }

    /// Return true if the point is within the bounds but not at the max edges
    #[inline]
    pub fn contains_exclusive(&self, p: Point3<S>) -> bool {
        all_coords_match!(p, self.min, |coord, min| coord >= min) &&
        all_coords_match!(p, self.max, |coord, max| coord < max)
    }

    /// Expand the bounds by a constant factor
    #[inline]
    pub fn expand(&self, delta: S) -> Self {
        let expansion = Vector3::from_value(delta);
        Bounds3 {
            min: self.min - expansion,
            max: self.max + expansion
        }
    }

    /// Get the vector from the min point to the max point
    #[inline]
    pub fn diagonal(&self) -> Vector3<S> {
        self.max - self.min
    }

    /// Get the surface area of the bounding box
    #[inline]
    pub fn surface_area(&self) -> S {
        let d = self.diagonal();
        let half = d.x * d.y + d.x * d.z + d.y * d.z;
        half + half
    }

    /// Get the volume of the bounding box
    #[inline]
    pub fn volume(&self) -> S {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    // Returns inde of which of three axes is longest
    #[inline]
    pub fn maximum_extent(&self) -> usize {
        let d = self.diagonal();
        if d.x > d.y && d.z > d.z { 0 }
        else if d.y > d.z { 1 }
        else { 2 }
    }

    #[inline]
    pub fn offset(&self, p: &Point3<S>) -> Vector3<S> {
        let mut o = p - self.min;
        if self.max.x > self.min.x { o.x /= self.max.x - self.min.x };
        if self.max.y > self.min.y { o.y /= self.max.y - self.min.y };
        if self.max.z > self.min.z { o.z /= self.max.z - self.min.z };
        o
    }
}

impl<S: BaseNum + Bounded> Bounds3<S> {
    #[inline]
    pub fn infinite() -> Bounds3<S> {
        Bounds3 {
            min: Point3::min_value(),
            max: Point3::max_value()
        }
    }

    #[inline]
    pub fn none() -> Bounds3<S> {
        Bounds3 {
            min: Point3::max_value(),
            max: Point3::min_value()
        }
    }
}

impl<S: BaseFloat> Bounds3<S> {
    #[inline]
    pub fn lerp(&self, t: &Point3<S>) -> Point3<S> {
        Point3::new(
            super::lerp(t.x, self.min.x, self.max.x),
            super::lerp(t.y, self.min.y, self.max.y),
            super::lerp(t.z, self.min.z, self.max.z))
    }
}

#[inline]
fn min<S: BaseNum>(a: S, b: S) -> S {
    if a < b { a } else { b }
}

#[inline]
fn max<S: BaseNum>(a: S, b: S) -> S {
    if a < b { b } else { a }
}
