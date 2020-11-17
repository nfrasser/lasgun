use cgmath::{
    Matrix, Transform,
    Vector3, Point3, Matrix4, Vector4,
    BaseFloat, Deg,
    InnerSpace, num_traits::identities::Zero
};
use super::{normal::Normal3, ray::Ray3, bounds::Bounds3};
use crate::interaction::surface::RayIntersection;

/// Identity transformation
pub const ID: Transform3<f64> = Transform3 { m: ID_MATRIX, minv: ID_MATRIX };

/// Identity matrix
const ID_MATRIX: Matrix4<f64> = Matrix4 {
    x: Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
    y: Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    z: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
    w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
};

/// Generic lasgun-wise transformation
pub trait Trans<N: BaseFloat>: Transform<Point3<N>> {
    fn transform_normal(&self, normal: Normal3<N>) -> Normal3<N>;
    fn transform_ray(&self, ray: Ray3<N>) -> Ray3<N>;
    fn transform_bounds(&self, bounds: Bounds3<N>) -> Bounds3<N>;
    fn transform_ray_intersection(&self, isect: &RayIntersection<N>)
    -> RayIntersection<N>;

    // Default implementations
    fn inverse_transform_normal(&self, normal: Normal3<N>) -> Normal3<N> {
        self.inverse_transform().unwrap().transform_normal(normal)
    }
    fn inverse_transform_ray(&self, ray: Ray3<N>) -> Ray3<N> {
        self.inverse_transform().unwrap().transform_ray(ray)
    }

    fn inverse_transform_bounds(&self, bounds: Bounds3<N>) -> Bounds3<N> {
        self.inverse_transform().unwrap().transform_bounds(bounds)
    }

    fn inverse_transform_ray_intersection(&self, isect: &RayIntersection<N>)
    -> RayIntersection<N> {
        self.inverse_transform().unwrap().transform_ray_intersection(isect)
    }
}

/// A transformation for three-space constructs
#[derive(Debug)]
pub struct Transform3<N: BaseFloat> {
    m: Matrix4<N>,
    minv: Matrix4<N>
}

impl<N: BaseFloat> Transform3<N> {

    /// Create a new transformation with the given matrix and inverse
    pub fn new(m: Matrix4<N>, minv: Matrix4<N>) -> Self {
        Transform3 { m, minv }
    }

    /// Create a new transformation from the given matrix. Calculates inverse
    /// internally
    pub fn from_matrix(m: Matrix4<N>) -> Self {
        let minv = m.inverse_transform().unwrap();
        Transform3 { m, minv }
    }

    /// Create a new transform with the given matrix slice, arranged in
    /// column-major order (as per cgmath convention)
    pub fn from_slice(mat: &[[N; 4]; 4]) -> Self {
        let m = Matrix4::new(
            mat[0][0], mat[0][1], mat[0][2], mat[0][3],
            mat[1][0], mat[1][1], mat[1][2], mat[1][3],
            mat[2][0], mat[2][1], mat[2][2], mat[2][3],
            mat[3][0], mat[3][1], mat[3][2], mat[3][3]);

        let minv = m.inverse_transform().unwrap();
        Transform3 { m, minv }
    }

    pub fn inverse(t: &Self) -> Self {
        Transform3 { m: t.minv, minv: t.m }
    }

    pub fn transpose(t: &Self) -> Self {
        Transform3 { m: t.m.transpose(), minv: t.minv.transpose() }
    }

    pub fn identity() -> Self {
        let m = Matrix4::one();
        Transform3 { m, minv: m }
    }

    pub fn translate(delta: Vector3<N>) -> Self {
        let m = Matrix4::from_translation(delta);
        let minv = Matrix4::from_translation(-delta);

        Transform3 { m, minv }
    }

    pub fn scale(x: N, y: N, z: N) -> Self {
        let one = N::one();

        let m = Matrix4::from_nonuniform_scale(x, y, z);
        let minv = Matrix4::from_nonuniform_scale(one/x, one/y, one/z);

        Transform3 { m, minv }
    }

    // Does this transform have a scale transformation?
    pub fn has_scale(&self) -> bool {
        let a1_2 = self.m.transform_vector(Vector3::unit_x()).magnitude2();
        let b1_2 = self.m.transform_vector(Vector3::unit_y()).magnitude2();
        let c1_2 = self.m.transform_vector(Vector3::unit_z()).magnitude2();

        let eps = N::default_epsilon();
        let ulps = N::default_max_ulps(); // unit of least precision
        let one = N::one();

        one.ulps_ne(&a1_2, eps, ulps) ||
        one.ulps_ne(&b1_2, eps, ulps) ||
        one.ulps_ne(&c1_2, eps, ulps)
    }

    pub fn rotate_x(theta: Deg<N>) -> Self {
        let m = Matrix4::from_angle_x(theta);
        let minv = m.transpose();
        Transform3 { m, minv }
    }

    pub fn rotate_y(theta: Deg<N>) -> Self {
        let m = Matrix4::from_angle_y(theta);
        let minv = m.transpose();
        Transform3 { m, minv }
    }

    pub fn rotate_z(theta: Deg<N>) -> Self {
        let m = Matrix4::from_angle_z(theta);
        let minv = m.transpose();
        Transform3 { m, minv }
    }

    pub fn rotate(theta: Deg<N>, axis: Vector3<N>) -> Self {
        let m = Matrix4::from_axis_angle(axis, theta);
        let minv = m.transpose();
        Transform3 { m, minv }
    }
}

impl<N: BaseFloat> Transform<Point3<N>> for Transform3<N> {

    #[inline]
    fn one() -> Self {
        Transform3::identity()
    }

    fn look_at(eye: Point3<N>, look: Point3<N>, up: Vector3<N>) -> Self {
        let m = Matrix4::look_at(eye, look, up);
        let minv = m.inverse_transform().unwrap();
        Transform3 { m, minv }
    }

    #[inline]
    fn transform_vector(&self, vec: Vector3<N>) -> Vector3<N> {
        self.m.transform_vector(vec)
    }

    #[inline]
    fn transform_point(&self, point: Point3<N>) -> Point3<N> {
        self.m.transform_point(point)
    }

    #[inline]
    fn concat(&self, other: &Self) -> Self {
        Transform3 {
            m: other.m.concat(&self.m),
            minv: self.minv.concat(&other.minv)
        }
    }

    #[inline]
    fn inverse_transform(&self) -> Option<Self> {
        Some(Transform3::inverse(self))
    }

    #[inline]
    fn inverse_transform_vector(&self, vec: Vector3<N>) -> Option<Vector3<N>> {
        Some(self.minv.transform_vector(vec))
    }

    #[inline]
    fn concat_self(&mut self, other: &Self) {
        let m = other.m.concat(&self.m);
        let minv = self.minv.concat(&other.minv);
        self.m = m;
        self.minv = minv;
    }

}

impl<N: BaseFloat> Trans<N> for Transform3<N> {
    #[inline]
    fn transform_normal(&self, normal: Normal3<N>) -> Normal3<N> {
        let (x, y, z) = (normal.0.x, normal.0.y, normal.0.z);
        let minv = &self.minv;
        Normal3::new(
            minv[0][0]*x + minv[0][1]*y + minv[0][2]*z,
            minv[1][0]*x + minv[1][1]*y + minv[1][2]*z,
            minv[2][0]*x + minv[2][1]*y + minv[2][2]*z)
    }

    #[inline]
    fn transform_ray(&self, ray: Ray3<N>) -> Ray3<N> {
        let origin = self.m.transform_point(ray.origin);
        let d = self.m.transform_vector(ray.d);
        Ray3::new(origin, d)
    }

    #[inline]
    fn transform_bounds(&self, bounds: Bounds3<N>) -> Bounds3<N> {
        // Implementation from http://dev.theomader.com/transform-bounding-boxes/
        let xa = self.m.x * bounds.min.x;
        let xb = self.m.x * bounds.max.x;

        let ya = self.m.y * bounds.min.y;
        let yb = self.m.y * bounds.max.y;

        let za = self.m.z * bounds.min.z;
        let zb = self.m.z * bounds.max.z;

        // Get min and max, transformed
        let min = zip_vectors!(xa, xb, min) + zip_vectors!(ya, yb, min) + zip_vectors!(za, zb, min);
        let max = zip_vectors!(xa, xb, max) + zip_vectors!(ya, yb, max) + zip_vectors!(za, zb, max);

        // Apply translation
        let w = &self.m.w;
        let min = Point3 { x: min.x + w.x, y: min.y + w.y, z: min.z + w.z };
        let max = Point3 { x: max.x + w.x, y: max.y + w.y, z: max.z + w.z };

        Bounds3::new(min, max)
    }

    #[inline]
    fn transform_ray_intersection(&self, isect: &RayIntersection<N>)
    -> RayIntersection<N> {
        let dpdu = self.transform_vector(isect.geometry.dpdu);
        let dpdv = self.transform_vector(isect.geometry.dpdv);
        let mut isect_t = RayIntersection::new(isect.t, isect.uv, dpdu, dpdv);
        isect_t.set_material(isect.material);

        // Transform surface shading if required
        if isect.geometry.dpdu != isect.surface.dpdu
        || isect.geometry.dpdv != isect.surface.dpdv {
            let dpdu = self.transform_vector(isect.surface.dpdu);
            let dpdv = self.transform_vector(isect.surface.dpdv);
            isect_t.set_surface_shading(dpdu, dpdv);
        }

        // Transform normal if available
        if let Some(n) = isect.n {
            isect_t.n = Some(self.transform_normal(n));
        }

        isect_t
    }

    #[inline]
    fn inverse_transform_normal(&self, normal: Normal3<N>) -> Normal3<N> {
        let (x, y, z) = (normal.0.x, normal.0.y, normal.0.z);
        let m = &self.m;
        Normal3::new(
            m[0][0]*x + m[0][1]*y + m[0][2]*z,
            m[1][0]*x + m[1][1]*y + m[1][2]*z,
            m[2][0]*x + m[2][1]*y + m[2][2]*z)
    }


    /// Transform a ray from its world coordinates to model coordinates
    #[inline]
    fn inverse_transform_ray(&self, ray: Ray3<N>) -> Ray3<N> {
        let origin = self.minv.transform_point(ray.origin);
        let d = self.minv.transform_vector(ray.d);
        Ray3::new(origin, d)
    }

    #[inline]
    fn inverse_transform_ray_intersection(&self, isect: &RayIntersection<N>)
    -> RayIntersection<N> {
        let dpdu = self.inverse_transform_vector(isect.geometry.dpdu).unwrap_or(Vector3::zero());
        let dpdv = self.inverse_transform_vector(isect.geometry.dpdv).unwrap_or(Vector3::zero());
        let mut isect_inv = RayIntersection::new(isect.t, isect.uv, dpdu, dpdv);

        // Transform surface shading if required
        if isect.geometry.dpdu != isect.surface.dpdu
        || isect.geometry.dpdv != isect.surface.dpdv {
            let dpdu = self.inverse_transform_vector(isect.surface.dpdu).unwrap_or(Vector3::zero());
            let dpdv = self.inverse_transform_vector(isect.surface.dpdv).unwrap_or(Vector3::zero());
            isect_inv.set_surface_shading(dpdu, dpdv);
        }

        // Transform normal if available
        if let Some(n) = isect.n {
            isect_inv.n = Some(self.inverse_transform_normal(n));
        }
        isect_inv
    }

}

#[inline] fn min<S: BaseFloat>(a: S, b: S) -> S { if a < b { a } else { b } }
#[inline] fn max<S: BaseFloat>(a: S, b: S) -> S { if a < b { b } else { a } }
