use crate::{
    primitives::{Hit, Point, Ray, Vector, SquareMatrix},
    shading::Material,
};

pub trait Hittable {
    fn get_hits(&self, ray: &Ray) -> Vec<Hit>;
    fn get_normal(&self, hit_point: &Point) -> Vector;
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
    fn get_transform(&self) -> &SquareMatrix;
    fn get_transform_inverted(&self) -> &SquareMatrix;
    fn set_transform(&mut self, transform: SquareMatrix);
}
