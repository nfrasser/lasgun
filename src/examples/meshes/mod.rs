use std::path::PathBuf;

pub fn path(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(".");
    path.push("meshes");
    path.push(name.clone());
    path.set_extension("obj");
    path
}
