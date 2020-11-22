#[cfg(feature = "bin")]

use ::lasgun::{Scene, output};

fn main() {
    // TODO: Make this interface friendlier
    let scene = Scene::new();
    output::render(&scene, [512, 512], "image.png")
}
