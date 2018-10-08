#[allow(dead_code)]
pub fn plane() -> (Box<[f32]>, Box<[u32]>) {
    let vertices = Box::new([
        -1.0, 0.0, -1.0,
        1.0, 0.0, -1.0,
        1.0, 0.0, 1.0,
        -1.0, 0.0, 1.0,
    ]);

    let faces = Box::new([
        0, 2, 1,
        0, 3, 2,
    ]);

    (vertices, faces)
}
