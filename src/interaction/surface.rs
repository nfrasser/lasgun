use std::f64;
use cgmath::{Point3, BaseFloat, Array};
use crate::{space::{normal::Normal3}, scene::MaterialRef};

/// Surface interaction retrived by casting a specific ray through a scene. The
/// `t` parameter is specified to compare previous parametric ray intersection
/// distances and avoid extra computation in some cases.
#[derive(Copy, Clone)]
pub struct SurfaceInteraction<N: BaseFloat> {
    pub t: f64,
    pub p: Point3<N>,
    pub n: Normal3<N>,

    // Reference to a material definition in the scene settings
    pub material: Option<MaterialRef>
}

impl<N: BaseFloat> SurfaceInteraction<N> {
    pub fn new(t: f64, p: Point3<N>, n: Normal3<N>) -> SurfaceInteraction<N> {
        SurfaceInteraction { t, p, n, material: None }
    }

    pub fn none() -> SurfaceInteraction<N> {
        SurfaceInteraction {
            t: f64::INFINITY,
            p: Point3::from_value(N::zero()),
            n: Normal3::new(N::zero(), N::zero(), N::zero()),
            material: None
        }
    }

    pub fn exists(&self) -> bool {
        self.t >= 0.0 && self.t < f64::INFINITY
    }
}
