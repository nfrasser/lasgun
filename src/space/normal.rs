use cgmath::prelude::*;
use cgmath::{ BaseNum, BaseFloat, Vector3 };

/**
Sormal vector representation.

For situations where we want the vector to be treated as a normal
e.g., when doing transformations to maintain normal behaviour
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Normal3<S>(pub Vector3<S>);

impl<S: BaseNum> Normal3<S> {

    /// Create a new normal from the given vector
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Normal3<S> { Normal3(Vector3::new(x, y, z)) }

    /// Get a reference to the underlying vector
    #[inline]
    pub fn as_vec(&self) -> &Vector3<S> { &self.0 }

    /// Get the value of the underlying vector
    #[inline]
    pub fn to_vec(&self) -> Vector3<S> { self.0 }

    /// Conver to the underlyinv vector
    #[inline]
    pub fn into_vec(self) -> Vector3<S> { self.0 }
}

impl<S: BaseFloat> Normal3<S> {
    /// Ensure the normal is facing toward the given d vector
    #[inline]
    pub fn face_forward(self, d: Vector3<S>) -> Normal3<S> {
        let zero = S::zero();
        Normal3(if self.0.dot(d) > zero { -self.0 } else { self.0 })
    }
}
