use crate::{ space::*, material::Material, interaction::RayIntersection };

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive {
    /// Object-level bounds for this primitive
    fn bound(&self) -> Bounds;

    /// Calculate intersection for a ray with this primitive.
    ///
    /// Implementors where an intersection occurs at a point _closer_ to the ray
    /// origin (i.e., `t < interaction.t`) should return a reference to
    /// themselves wrapped in `Some()`.
    ///
    /// Implementors should also update the following:
    ///
    /// - `interaction.t`: new interaction ray parameter
    /// - `interaction.(dpdu|dpdv)`: new differentials at the surface of intersection
    /// - `interaction.material`: a `MaterialRef` for the material at the
    ///    surface, if applicable.
    fn intersect(&self, ray: &Ray, isect: &mut RayIntersection) -> OptionalPrimitive;

    /// Get a dynamic reference to the material this primitive uses. If None,
    /// clients should use some pre-defined default material (such as the global
    /// material on the parent Triangle mesh).
    fn material(&self) -> Option<Material> { None }

    /// Whether an intersection with the given ray exists. Default
    /// implementation calls `intersect`. Available so that more efficient
    /// intersection tests can be computed for some primitives without having to
    /// update the interaction.
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray, &mut RayIntersection::default()).is_some()
    }
}

pub type OptionalPrimitive<'a> = Option<&'a dyn Primitive>;
