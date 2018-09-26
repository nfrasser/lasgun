#[cfg(feature = "bin")]

use ::lasgun::{Scene, scene, output};

fn main() {
    // TODO: Make this interface friendlier
    let options = scene::Options {
        width: 256,
        height: 256,
        ambient: [0.2; 3],
        eye: [0.0; 3],
        view: [0.0; 3],
        up: [0.0; 3],
        fov: 50.0,
        supersampling: 1,
        threads: 1
    };

    let scene = Scene::new(options);
    output::render(&scene, "image.png")
}
