use ::lasgun::prelude::*;
use std::thread;
use self::spooky::build_spooky_scene;

/// Pointers are not constrained by the Rust borrow checker.
///
/// Render every nth pixel, starting with pixel at index k in the pixel buffer
/// representation arranged in row-major order.
///
/// n should equal the number of threads in use.
fn render_subset(k: usize, n: usize, scene_ptr: *const Scene, film_ptr: *mut Film) {
    let scene: &Scene = unsafe { &*scene_ptr };
    let film: &mut Film = unsafe { &mut *film_ptr };

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

        film.set(x, y, &colour)
    }
}

fn render(scene: Scene, filename: &str) {
    let mut film = output::film_for(&scene);

    // Cast scene and film references into pointers
    let scene_ptr = &scene as *const Scene;
    let film_ptr = &mut film as *mut Film;

    // Raw pointers cannot be moved into threads.
    // Wrap them in a data structure that can (see below).
    let wrapped_scene = UnsafeThreadWrapper { ptr: scene_ptr };
    let wrapped_film = UnsafeThreadWrapperMut { ptr: film_ptr };
    let handle = thread::spawn(move || {
        let scene_ptr = wrapped_scene.ptr;
        let film_ptr = wrapped_film.ptr;
        render_subset(0, 2, scene_ptr, film_ptr)
    });

    render_subset(1, 2, scene_ptr, film_ptr);

    // Wait for the other thread to finish
    handle.join().unwrap();

    film.save(filename);
}

// Define pointer wrappers that implement the `Send` trait. Arc and Mutex do
// this already.
#[derive(Copy, Clone)] struct UnsafeThreadWrapper<T> { ptr: *const T }
#[derive(Copy, Clone)] struct UnsafeThreadWrapperMut<T> { ptr: *mut T }
unsafe impl<T> std::marker::Send for UnsafeThreadWrapper<T> {}
unsafe impl<T> std::marker::Send for UnsafeThreadWrapperMut<T> {}

fn main() {
    let scene = build_spooky_scene();
    render(scene, "spooky3.png")
}


mod spooky;
