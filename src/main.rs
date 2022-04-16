use std::f64::consts::PI;

use rust_tracer::{
    cameras::Camera,
    lighting::PointLight,
    objects::{Hittable, Sphere},
    primitives::{Canvas, Point, RgbColor, Vector, World},
    shading::Material,
};

fn setup_world() -> World {
    let mut world = World::new();
    let light = PointLight::new(Point::new(-10., 10., -10.), RgbColor::new(0.7, 0.7, 0.7));
    world.add_light(light);
    world.add_light(PointLight::new(
        Point::new(10., -15., -10.),
        RgbColor::new(0.5, 0.5, 0.5),
    ));

    let mut shape1 = Sphere::new(Point::new(-0.5, 0.0, 0.), 0.5);
    //let transform =
    //   &SquareMatrix::translation(-0.6, 0., 0.) * &SquareMatrix::scaling(0.3, 0.4, 0.2);
    //shape1.set_transform(transform);
    let mut material1 = Material::default();
    material1.set_color(RgbColor::new(0.9, 0.5, 0.2));
    shape1.set_material(material1);

    let mut shape2 = Sphere::new(Point::new(0.5, 0.15, -0.3), 0.65);
    let mut material2 = Material::default();
    material2.set_color(RgbColor::new(0.1, 0.7, 0.9));
    material2.set_shininess(50.);
    material2.set_specular(0.5);
    material2.set_diffuse(0.9);
    shape2.set_material(material2);
    world.add_object(shape1);
    world.add_object(shape2);
    world
}

fn main() {
    let canvas_pixels = 1000;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);
    let world = setup_world();
    let mut camera = Camera::new(&canvas, PI / 8.0);
    camera.set_position(
        &Point::new(3.0, 0.0, -5.),
        &Point::new(0., 0., 0.),
        &Vector::new(-0.3, 1., 0.),
    );
    for y in 0..canvas_pixels {
        for x in 0..canvas_pixels {
            let ray = camera.get_ray(x, y);
            let all_hits = world.get_hits(&ray);

            if let Some(hit) = World::get_first_visible_hit(&all_hits) {
                let record = hit.get_hit_record(&ray);
                let mut color = RgbColor::new(0., 0., 0.);
                for light in world.get_lights().iter() {
                    color = &color + &light.shade(&record);
                }
                canvas.set_pixel(y, x, color);
            }
        }
    }
    canvas.correct_colors();
    let result = canvas.to_ppm("C:/Users/romea/source/rust-tracer/output.ppm");
    if result.is_err() {
        println!("{:?}", result.err());
    }
}
