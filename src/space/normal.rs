use cgmath::prelude::*;
use cgmath::{ BaseNum, BaseFloat, Vector3 };

/// Normal vector representation. Used for cases where we want the vector to be
/// treated as a normal e.g., when doing transformations to maintain normal
/// behaviour
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Normal3<S>(pub Vector3<S>);

impl<S: BaseNum> Normal3<S> {

    /// Create a new normal from the given vector
    #[inline]
    pub fn new(x: S, y: S, z: S) -> Normal3<S> { Normal3(Vector3::new(x, y, z)) }

    /// Create a new normal from the given vector
    #[inline]
    pub fn zero() -> Normal3<S> { Normal3(Vector3::new(S::zero(), S::zero(), S::zero())) }

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

    /// Normalize the inner vector
    #[inline]
    pub fn normalize(&mut self) {
        self.0 = self.0.normalize();
    }
}

impl<S: BaseNum> Into<Vector3<S>> for Normal3<S> {
    fn into(self) -> Vector3<S> { self.0 }
}

impl<'a, S: BaseNum> Into<&'a Vector3<S>> for &'a Normal3<S> {
    fn into(self) -> &'a Vector3<S> { &self.0 }
}
