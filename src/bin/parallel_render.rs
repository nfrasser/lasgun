use ::lasgun::prelude::*;
use std::{thread, sync::{Mutex, Arc}};
use self::spooky::build_spooky_scene;

/// Render every nth pixel, starting with pixel at index k in the pixel buffer
/// representation arranged in row-major order.
///
/// n should equal the number of threads in use.
fn render_subset(k: usize, n: usize, shared_scene: Arc<Scene>, shared_film: Arc<Mutex<Film>>) {
    // Dereference the shared pointer and use underlying scene as a reference
    let scene: &Scene = &*shared_scene;

    let (width, height) = (
        scene.options.width as usize,
        scene.options.height as usize
    );

    let npixels = width * height;
    let root: Accel = Accel::from(scene);

    for offset in (k..npixels).step_by(n) {
        let x = offset % width;
        let y = offset / height;

        let ray = PrimaryRay::at(scene, x, y);
        let colour = ray.cast(&root);

        // Acquire the film lock so we can write to it. If another thread has
        // the lock, this thread blocks until the lock is available.
        let mut locked_film = shared_film.lock().unwrap();
        locked_film.set(x, y, &colour)

        // locked_film is released here when it goes out of scope
    }
}

fn render(scene: Scene, filename: &str) {
    let film = Arc::new(Mutex::new(output::film_for(&scene)));
    let scene = Arc::new(scene);

    // Clone the Arc to make new instances. This atomically increments the
    // internal reference count and does not clone the scene or film data
    let shared_scene = scene.clone();
    let shared_film = film.clone();
    let handle = thread::spawn(move || {
        render_subset(0, 2, shared_scene, shared_film)
    });

    render_subset(1, 2, scene.clone(), film.clone());

    // Wait for the other thread to finish
    handle.join().unwrap();

    film.lock().unwrap().save(filename);
}

fn main() {
    let scene = build_spooky_scene();
    render(scene, "spooky2.png")
}

mod spooky;
