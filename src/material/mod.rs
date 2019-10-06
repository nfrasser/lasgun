use crate::{space::*, interaction::{SurfaceInteraction, BSDF}};

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Matte(matte::Matte),
    Plastic(plastic::Plastic),
    Metal(metal::Metal),
    Glass(glass::Glass),
    Mirror(mirror::Mirror)
}

impl Material {
    /// Default material for cases where a specific one may not be required
    /// (e.g., for triangle meshes that come with their own material libraries).
    pub fn default() -> Material {
        Self::matte([0.5, 0.5, 0.5], 0.0)
    }

    pub fn matte(kd: [f64; 3], sigma: f64) -> Material {
        let kd = Color::new(kd[0], kd[1], kd[2]);
        Material::Matte(matte::Matte::new(kd, sigma))
    }

    pub fn plastic(kd: [f64; 3], ks: [f64; 3], roughness: f64) -> Material {
        let kd = Color::new(kd[0], kd[1], kd[2]);
        let ks = Color::new(ks[0], ks[1], ks[2]);
        Material::Plastic(plastic::Plastic::new(kd, ks, roughness))
    }

    pub fn metal(eta: [f64; 3], k: [f64; 3], u_roughness: f64, v_roughness: f64) -> Material {
        let eta = Color::new(eta[0], eta[1], eta[2]);
        let k = Color::new(k[0], k[1], k[2]);
        Material::Metal(metal::Metal::new(eta, k, u_roughness, v_roughness))
    }

    pub fn glass(kr: [f64; 3], kt: [f64; 3], eta: f64) -> Material {
        let kr = Color::new(kr[0], kr[1], kr[2]);
        let kt = Color::new(kt[0], kt[1], kt[2]);
        // TODO: Fix and implement roughtness
        Material::Glass(glass::Glass::new(kr, kt, eta, 0.0, 0.0))
    }

    pub fn mirror(kr: [f64; 3]) -> Material {
        let kr = Color::new(kr[0], kr[1], kr[2]);
        Material::Mirror(mirror::Mirror::new(kr))
    }

    /// Computes the function for how light is handled at the material at the
    /// given point of interaction.
    pub fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        match self {
            Material::Matte(mat) => mat.scattering(interaction),
            Material::Plastic(mat) => mat.scattering(interaction),
            Material::Metal(mat) => mat.scattering(interaction),
            Material::Glass(mat) => mat.scattering(interaction),
            Material::Mirror(mat) => mat.scattering(interaction),
        }
    }
}

pub use background::Background;

mod background;
mod matte;
mod plastic;
mod metal;
mod glass;
mod mirror;
