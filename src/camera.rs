use std::f64;
use crate::space::*;
use crate::img::Img;

#[derive(Debug)]
pub struct Camera {

    /// The position of the eye/camera in the scene
    pub origin: Point,

    /// The direction in which the view vector points.
    /// Its magnitude is the distance to the focal plane
    pub view: Vector,

    /// The orientation of the eye in the scene
    pub up: Vector,

    /// Auxilary vectory, orthogonal to the up and view vectors
    pub aux: Vector,

    /// Type of projection
    projection: Projection,

    /// Supersampling root; 0 => 1 sample, 1 => 4 samples, 2 => 9, etc.
    supersampling: Supersampling,

    /// Aperture radius in world size, for lens blur. Defaults to 0 (pinhole, no blur)
    aperture_radius: f64,

    /// Vertical extent of image plane
    image_plane_height: f64,

    /// Distance between individial photocells on the sensor as a multiple of
    /// the distance between pixels on the image plane. Tweak this value
    /// to change the perspective.
    pixel_separation: f64
}

#[derive(Clone, Copy, Debug)]
enum Projection {
    /// Standard perspective camera with a field-of-view (in degrees)
    Perspective(f64),

    /// Orthographic camera for isometric rendering w/ a scalar field that
    /// represents vertical height (along the y-axis/up vector) of focal plane
    /// in world units.
    Orthographic(f64)
}

#[derive(Clone, Copy, Debug)]
struct Supersampling {
    /// Root of how many samples to take. 0 => 1 sample, 1 => 4 samples, 2 => 9,
    /// etc.
    pub root: usize,

    /// Distance between samples within a pixel. Must be 1 for root 0
    distance: f64
}

impl Camera {
    fn new(projection: Projection) -> Self {
        Camera {
            projection,
            origin: Point::new(0., 0., 0.),
            view: Vector::unit_z(),
            up: Vector::unit_y(),
            aux: Vector::unit_x(),
            supersampling: Supersampling::new(),
            aperture_radius: 0.,
            image_plane_height: projection.image_plane_height(1.),
            pixel_separation: projection.pixel_separation()
        }
    }

    pub fn perspective(fov: f64) -> Self {
        debug_assert!(fov > 0.);
        Camera::new(Projection::Perspective(fov))
    }

    pub fn orthographic(height: f64) -> Self {
        debug_assert!(height > 0.);
        Camera::new(Projection::Orthographic(height))
    }

    pub fn look_at(&mut self, origin: [f64; 3], look: [f64; 3], up: [f64; 3]) {
        let origin = Point::from(origin);
        let view = Point::from(look) - origin;
        let aux = view.cross(up.into());
        self.origin = origin;
        self.up = aux.cross(view).normalize();
        self.aux = aux.normalize();
        self.view = view;
        self.image_plane_height = self.projection.image_plane_height(view.magnitude());
    }

    pub fn set_supersampling(&mut self, base: u8) {
        self.supersampling.set(base)
    }

    pub fn set_aperture_radius(&mut self, radius: f64) {
        self.aperture_radius = radius
    }

    #[inline]
    pub fn num_samples(&self) -> usize {
        self.supersampling.num_samples()
    }

    pub fn allocate_samples(&self) -> Vec<Ray> {
        vec![Ray::default(); self.num_samples()]
    }

    pub fn sample(&self, x: u32, y: u32, img: &impl Img, rays: &mut [Ray]) {
        debug_assert!(self.num_samples() == rays.len());
        let img_plane_height = self.image_plane_height;
        let img_plane_width = img_plane_height * img.aspect();
        let pixel_size = img_plane_height * img.hinv();
        let sample_separation = self.supersampling.distance() * pixel_size;
        let sample_origin = Point2f {
            x: (x as f64 * img.winv() - 0.5) * img_plane_width,
            y: (0.5 - (y + 1) as f64 * img.hinv()) * img_plane_height
        };

        // All sampled rays have the same origin
        let origin = self.origin
            + (sample_origin.y * self.pixel_separation * self.up)
            + (sample_origin.x * self.pixel_separation * self.aux);

        // Target direction at bottom-left corner of target pixel
        let d = self.view + (sample_origin.y * self.up) + (sample_origin.x * self.aux);

        let updiff = self.up * sample_separation;
        let auxdiff = self.aux * sample_separation;
        let halfdiff = updiff * 0.5 + auxdiff * 0.5; // centers the sample

        let dim = self.supersampling.root;
        for i in 0..dim {
            for j in 0..dim {
                let idx = i * dim + j;
                let (i, j) = (i as f64, j as f64);
                let d = d + (j * updiff) + (i * auxdiff) + halfdiff;
                // TODO: Integrate aperture radius
                rays[idx] = Ray::new(origin, d)
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::perspective(45.)
    }
}

impl Projection {
    /// Extent of the image plane in world coordinates a function of the
    /// distance to the plane
    pub fn image_plane_height(&self, focal_distance: f64) -> f64 {
        match self {
            Self::Perspective(fov) =>
                focal_distance * f64::tan(*fov * f64::consts::PI / 360.) * 2.,
            Self::Orthographic(height) => *height
        }
    }

    /// Distance between pixels on the sensor as a ratio of distance between
    /// sample centres on the image plane.
    pub fn pixel_separation(&self) -> f64 {
        match self {
            Self::Perspective(_) => 0.,
            Self::Orthographic(_) => 1.
        }
    }
}

impl Supersampling {
    pub fn new() -> Supersampling {
        Supersampling { root: 1, distance: 1. }
    }

    #[inline]
    pub fn distance(&self) -> f64 { self.distance }

    #[inline]
    pub fn num_samples(&self) -> usize {
        self.root * self.root
    }

    pub fn set(&mut self, base: u8) {
        debug_assert!(base < 255);
        self.root = base as usize + 1;
        self.distance = 1. / self.root as f64;
    }
}
