use na::{ Real, Scalar, Vector3 };

/**
Normal vector representation.

For situations where we want the vector to be treated as a normal
e.g., when doing transformations to maintain normal behaviour
*/
#[derive(Debug, Copy, Clone)]
pub struct Normal3<N: Scalar>(pub Vector3<N>);

impl<N: Scalar> Normal3<N> {
    /**
    Create a new normal from the given vector
    */
    pub fn new(v: Vector3<N>) -> Normal3<N> { Normal3(v) }

    /**
    Get a reference to the underlying vector
    */
    pub fn as_ref(&self) -> &Vector3<N> { &self.0 }

    /**
    Get the value of the underlying vector
    */
    pub fn unwrap(&self) -> Vector3<N> { self.0 }

}

impl<N: Scalar + Real> Normal3<N> {
    /// Ensure the normal is facing toward the given d vector
    pub fn face_forward(&self, d: &Vector3<N>) -> Normal3<N> {
        Normal3(if self.0.dot(d).is_sign_negative() { -self.0 } else { self.0 })
    }
}
