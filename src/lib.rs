extern crate nalgebra as na;

mod space;

pub mod material;
pub mod shape;
pub mod primitive;
pub mod light;
pub mod ray;

pub mod scene;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
