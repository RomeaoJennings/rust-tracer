use std::f64::consts::PI;

use crate::{
    cameras::Camera,
    lighting::PointLight,
    objects::{Hittable, Sphere},
    primitives::{Canvas, Point, RgbColor, SquareMatrix, Vector, World},
    shading::Material,
};

pub fn build(canvas: &Canvas) -> (World, Camera) {
    let mut world = World::new();
    world.add_light(PointLight::new(
        Point::new(-10., 6., -10.),
        RgbColor::new(0.5, 0.5, 0.5),
    ));
    world.add_light(PointLight::new(
        Point::new(13., 2.5, -10.),
        RgbColor::new(0.5, 0.5, 0.5),
    ));

    let mut floor = Sphere::identity();
    floor.set_transform(SquareMatrix::scaling(10., 0.01, 10.));
    let mut floor_material = Material::default();
    floor_material.set_color(RgbColor::new(1., 0.9, 0.9));
    floor_material.set_specular(0.0);
    floor.set_material(floor_material);

    let mut left_wall = floor.clone();
    let mut right_wall = floor.clone();
    let mut wall_material = left_wall.get_material().clone();
    wall_material.set_color(RgbColor::new(0.2, 0.2, 0.6));
    left_wall.set_material(wall_material.clone());
    right_wall.set_material(wall_material);
    let mut left_transform =
        &SquareMatrix::translation(0., 0., 5.) * &SquareMatrix::rotation_y(-PI / 4.);
    left_transform = &left_transform * &SquareMatrix::rotation_x(PI / 2.);
    left_transform = &left_transform * &SquareMatrix::scaling(10., 0.01, 10.);
    left_wall.set_transform(left_transform);

    let mut right_transform =
        &SquareMatrix::translation(0., 0., 5.) * &SquareMatrix::rotation_y(PI / 4.);
    right_transform = &right_transform * &SquareMatrix::rotation_x(PI / 2.);
    right_transform = &right_transform * &SquareMatrix::scaling(10., 0.01, 10.);
    right_wall.set_transform(right_transform);
    world.add_object(floor);
    world.add_object(left_wall);
    world.add_object(right_wall);

    let mut middle = Sphere::identity();
    middle.set_transform(SquareMatrix::translation(-0.5, 1., 0.5));
    let mut middle_material = Material::default();
    middle_material.set_color(RgbColor::new(0.1, 1., 0.5));
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(0.3);
    let mut left_material = middle_material.clone();
    let mut right_material = middle_material.clone();
    middle_material.set_specular(1.0);
    middle.set_material(middle_material);
    world.add_object(middle);

    let mut right = Sphere::identity();
    right.set_transform(
        &SquareMatrix::translation(1.5, 0.5, -0.5) * &SquareMatrix::scaling(0.5, 0.5, 0.5),
    );
    right_material.set_color(RgbColor::new(0.5, 1., 0.1));
    right.set_material(right_material);
    world.add_object(right);

    let mut left = Sphere::identity();
    left.set_transform(
        &SquareMatrix::translation(-1.5, 0.33, -0.75) * &SquareMatrix::scaling(0.33, 0.33, 0.33),
    );
    left_material.set_color(RgbColor::new(1., 0.8, 0.1));
    left.set_material(left_material);
    world.add_object(left);

    let mut camera = Camera::new(&canvas, PI / 2.5);
    camera.set_position(
        &Point::new(0., 1.5, -5.),
        &Point::new(0., 1., 0.),
        &Vector::new(0., 1., 0.),
    );
    (world, camera)
}
