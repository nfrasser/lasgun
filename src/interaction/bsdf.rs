use std::mem;
use crate::space::*;
use crate::core::bxdf::{BxDFType, BxDF};
use super::SurfaceInteraction;

/// Collection of BRDF and BTDF, allowing system to work with composite BxDFs.
pub struct BSDF {
    pub eta: f64,

    /// Shading normal (Not implemented yet)
    ns: Normal,

    /// Geometry surface normal
    ng: Normal,

    // Orthogonal shading differential vectors (normalized). As we currently
    // don't have shading normals, these are generated from the geometric
    // normal.
    ss: Vector,
    ts: Vector,

    /// Internal BxDFs group (up to 8)
    bxdfs: [BxDF; MAX_BXDFS],

    /// Current actual number of bxdfs (must be below 8)
    num_bxdfs: usize
}

impl BSDF {
    pub fn new_with_eta(si: &SurfaceInteraction, funcs: &[BxDF], eta: f64) -> BSDF {
        debug_assert!(funcs.len() < MAX_BXDFS);

        // TODO: Generate these from shading parameters
        let n = si.n.as_vec();
        let v = if n.x < n.y { if n.x < n.z { Vector::unit_x() } else { Vector::unit_z() } }
            else { if n.y < n.z { Vector::unit_y() } else { Vector::unit_z() } };

        let ss = n.cross(v);
        let ts = n.cross(ss);

        // Allocate initial scattering functions
        let mut num_bxdfs = 0;
        let mut bxdfs: [BxDF; MAX_BXDFS] = unsafe { mem::uninitialized() };
        for bxdf in funcs.iter() {
            bxdfs[num_bxdfs] = *bxdf;
            num_bxdfs += 1;
        }

        BSDF { eta, ns: si.n, ng: si.n, ss, ts, bxdfs, num_bxdfs }
    }

    /// Simple in that it doesn't include eta
    pub fn new(si: &SurfaceInteraction, bxdfs: &[BxDF]) -> BSDF {
        BSDF::new_with_eta(si, bxdfs, 1.0)
    }

    pub fn empty(si: &SurfaceInteraction) -> BSDF {
        BSDF::new(si, &[])
    }

    pub fn add(&mut self, b: BxDF) {
        debug_assert!(self.num_bxdfs < MAX_BXDFS);
        self.bxdfs[self.num_bxdfs] = b;
        self.num_bxdfs += 1;
    }

    pub fn num_components(&self) -> usize {
        self.num_bxdfs
    }

    /// Compute the local-coordinates of the given vector, such that the normal
    /// is equivalent to [0, 0, 1] in the new coordinates.
    pub fn to_local(&self, v: &Vector) -> Vector {
        Vector {
            x: v.dot(self.ss),
            y: v.dot(self.ts),
            z: v.dot(self.ns.0),
        }
    }

    /// Inverse of `to_local`
    pub fn to_world(&self, v: &Vector) -> Vector {
        Vector {
            x: self.ss.x * v.x + self.ts.x * v.y + self.ns.0.x * v.z,
            y: self.ss.y * v.x + self.ts.y * v.y + self.ns.0.y * v.z,
            z: self.ss.z * v.x + self.ts.z * v.y + self.ns.0.z * v.z
        }
    }

    pub fn f(&self, wo: &Vector, wi: &Vector/*, flags: BxDFType*/) -> Color {
        // Whether reflection occurs
        let reflect = wi.dot(self.ng.0) * wo.dot(self.ng.0) > 0.0;

        // Convert to local coordinates
        let wo = self.to_local(wo);
        let wi = self.to_local(wi);

        let mut f = Color::from_value(0.0);

        // Calculate result of all the BxDFs
        for i in 0..self.num_bxdfs {
            let bxdf = &self.bxdfs[i];
            let t = bxdf.t();
            if ((reflect && (t & BxDFType::REFLECTION) != BxDFType::NONE)) ||
            ((reflect && (t & BxDFType::TRANSMISSION) != BxDFType::NONE)) {
                f += bxdf.f(&wo, &wi);
            }
        }

        f
    }
}

const MAX_BXDFS: usize = 8;
