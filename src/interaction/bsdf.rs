use std::mem;
use crate::space::*;
use crate::core::bxdf::{BxDFType, BxDF, BxDFSample};
use super::SurfaceInteraction;

/// Collection of BRDF and BTDF, allowing system to work with composite BxDFs.
pub struct BSDF {
    pub eta: f64,

    /// Geometry surface normal
    ng: Normal,

    /// Shading normal
    ns: Normal,

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

        let ng = si.ng;
        let ns = si.ns;
        let ss = si.surface.dpdu;
        let ts = ns.0.cross(ss);

        // Allocate initial scattering functions
        let mut num_bxdfs = 0;
        let mut bxdfs: [BxDF; MAX_BXDFS] = unsafe { mem::uninitialized() };
        for bxdf in funcs.iter() {
            bxdfs[num_bxdfs] = *bxdf;
            num_bxdfs += 1;
        }

        BSDF { eta, ns, ng, ss, ts, bxdfs, num_bxdfs }
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

    #[inline]
    pub fn num_components(&self) -> usize {
        self.num_bxdfs
    }

    #[inline]
    pub fn num_matching_components(&self, flags: BxDFType) -> usize {
        self.iter().filter(|bxdf| bxdf.matches(flags)).count()
    }

    pub fn f(&self, wo: &Vector, wi: &Vector/*, flags: BxDFType*/) -> Color {
        // Whether reflection occurs
        let reflect = wi.dot(self.ng.0) * wo.dot(self.ng.0) > 0.0;

        // Convert to local coordinates
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);

        if wo_local.z == 0.0 { return Color::zero() };

        // Calculate result of all the BxDFs
        self.iter().fold(Color::zero(), |f, bxdf| {
            if (reflect && bxdf.has_t(BxDFType::REFLECTION))
            || (!reflect && bxdf.has_t(BxDFType::TRANSMISSION)) {
                f + bxdf.f(&wo_local, &wi_local)
            } else {
                f
            }
        })
    }

    pub fn sample_f(&self, wo: &Vector, sample: &Point2f, flags: BxDFType) -> BxDFSample {
        let matching_comps = self.num_matching_components(flags);
        if matching_comps == 0 { return BxDFSample::zero() }

        let comp = ((sample.x * matching_comps as f64)
            .floor() as usize)
            .min(matching_comps - 1);

        // Get BxDF reference for chosen component
        let bxdf = self.iter().filter(|bxdf| bxdf.matches(flags)).nth(comp);
        debug_assert!(bxdf.is_some()); let bxdf = bxdf.unwrap();

        // Remap BxDF sample to [0,1)^2
        let sample = Point2f::new(
            ONE_MINUS_EPSILON.min(sample.x * matching_comps as f64 - comp as f64),
            sample.y);

        // Sample chosen BxDF
        let wo_local = self.to_local(wo);
        if wo_local.z == 0.0 { return BxDFSample::zero() }; // No contribution
        let f_sample = bxdf.sample_f(&wo_local, &sample);
        if f_sample.pdf == 0.0 { return f_sample } // No contribution from this sample

        // Determine incident sample vector in world coordinates
        let wi_local = f_sample.wi;
        let wi = self.to_world(&wi_local);

        // Compute value of BSDF for sampled direction
        let spectrum = if bxdf.has_t(BxDFType::SPECULAR) {
            f_sample.spectrum
        } else {
            // Add contribution from each matching component
            let reflect = wi.dot(self.ng.0) * wo.dot(self.ng.0) > 0.0;
            self.iter().filter(|bxdf| bxdf.matches(flags))
            .filter(|bxdf| //
                (reflect && bxdf.has_t(BxDFType::REFLECTION)) ||
                (!reflect && bxdf.has_t(BxDFType::TRANSMISSION))
            )
            .fold(Color::zero(), |f, bxdf| f + bxdf.f(&wo_local, &wi_local))
        }.map(|i| i.max(0.0).min(1.0)); // Clamp

        // Compute overall PDF with all _other_ matching BxDFs
        let pdf = if !bxdf.has_t(BxDFType::SPECULAR) && matching_comps > 1 {
            self.iter().filter(|bxdf| bxdf.matches(flags))
            .filter(|f| *f as *const BxDF != bxdf as *const BxDF)
            .fold(f_sample.pdf, |pdf, bxdf| pdf + bxdf.pdf(&wo_local, &wi_local))
        } else {
            f_sample.pdf
        } / matching_comps as f64; // Scale by contribution of each comp

        BxDFSample::new(spectrum, wi, pdf)
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &BxDF> {
        self.bxdfs[0..self.num_bxdfs].iter()
    }

    /// Compute the local-coordinates of the given vector, such that the normal
    /// is equivalent to [0, 0, 1] in the new coordinates.
    #[inline]
    fn to_local(&self, v: &Vector) -> Vector {
        Vector {
            x: v.dot(self.ss),
            y: v.dot(self.ts),
            z: v.dot(self.ns.0),
        }
    }

    /// Inverse of `to_local`
    #[inline]
    fn to_world(&self, v: &Vector) -> Vector {
        Vector {
            x: self.ss.x * v.x + self.ts.x * v.y + self.ns.0.x * v.z,
            y: self.ss.y * v.x + self.ts.y * v.y + self.ns.0.y * v.z,
            z: self.ss.z * v.x + self.ts.z * v.y + self.ns.0.z * v.z
        }
    }
}

const MAX_BXDFS: usize = 8;
const ONE_MINUS_EPSILON: f64 = 1.0 - std::f64::EPSILON;
