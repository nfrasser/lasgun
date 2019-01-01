use ::lasgun::prelude::*;
use self::spooky::build_spooky_scene;

fn render(scene: Scene, filename: &str) {
    // Dimensions of image to create
    let (width, height) = (
        scene.options.width as usize,
        scene.options.height as usize
    );

    let npixels = width * height; // number of pixels in the final image

    // Create acceleration primitive from scene contents
    let root = Accel::from(&scene);

    // Generate a pixel buffer from the given scene with capacity of
    // width * height
    let mut film = output::film_for(&scene);

    for offset in 0..npixels {
        let x = offset % width;
        let y = offset / height;

        // Get the ray into the scene for the given pixel position
        // Cast the ray into the scene and see what colour it gets
        let ray = PrimaryRay::at(&scene, x, y);
        let colour = ray.cast(&root);

        // Update the pixel colour
        film.set(x, y, &colour)
    }

    film.save(filename)
}

fn main() {
    let scene = build_spooky_scene();
    render(scene, "spooky1.png")
}

mod spooky;
