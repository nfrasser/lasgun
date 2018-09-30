use cgmath::prelude::*;
use cgmath::{ BaseNum, BaseFloat, Vector3 };

/**
Sormal vector representation.

For situations where we want the vector to be treated as a normal
e.g., when doing transformations to maintain normal behaviour
*/
#[derive(Debug, Copy, Clone)]
pub struct Normal3<S>(pub Vector3<S>);

impl<S: BaseNum> Normal3<S> {

    /// Create a new normal from the given vector
    pub fn new(v: Vector3<S>) -> Normal3<S> { Normal3(v) }

    /// Get a reference to the underlying vector
    pub fn as_ref(&self) -> &Vector3<S> { &self.0 }

    /// Get the value of the underlying vector
    pub fn to_vec(&self) -> Vector3<S> { self.0 }

    /// Conver to the underlyinv vector
    pub fn into_vec(self) -> Vector3<S> { self.0 }
}

impl<S: BaseFloat> Normal3<S> {
/// Ensure the normal is facing toward the given d vector
    pub fn face_forward(&self, d: &Vector3<S>) -> Normal3<S> {
        Normal3(if self.0.dot(*d).is_sign_negative() { -self.0 } else { self.0 })
    }
}
