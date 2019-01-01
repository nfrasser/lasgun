use std::thread;
use ::lasgun::prelude::*;
use self::spooky::build_spooky_scene;

fn render(scene: Scene, filename: &str) {
    let (width, height) = (
        scene.options.width as usize,
        scene.options.height as usize
    );
    let npixels = width * height;

    let mut film = output::film_for(&scene);

    let handle = thread::spawn(|| {
        let root: Accel = Accel::from(&scene);
        for offset in (0..npixels).step_by(2) {
            let x = offset % width;
            let y = offset / height;

            let bg = scene.background(x, y);

            let ray = PrimaryRay::at(&scene, x, y);
            let colour = ray.cast(&root, &bg);

            film.set(x, y, &colour)
        }
    });

    let root: Accel = Accel::from(&scene);
    for offset in (1..npixels).step_by(2) {
        let x = offset % width;
        let y = offset / height;

        let bg = scene.background(x, y);

        let ray = PrimaryRay::at(&scene, x, y);
        let colour = ray.cast(&root, &bg);

        film.set(x, y, &colour)
    }

    // Wait for the thread to finish
    handle.join();

    film.save(filename)
}

fn main() {
    let scene = build_spooky_scene();
    render(scene, "spooky1.png")
}

mod spooky;
