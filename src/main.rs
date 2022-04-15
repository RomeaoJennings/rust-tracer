use rust_tracer::{
    lighting::PointLight,
    objects::{Hittable, Sphere},
    primitives::{Canvas, Point, Ray, RgbColor, SquareMatrix, World},
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

    let mut shape2 = Sphere::new(Point::new(0.5, 0.15, -0.3), 0.7);
    let mut material2 = Material::default();
    material2.set_color(RgbColor::new(0.1, 0.1, 0.9));
    material2.set_shininess(50.);
    material2.set_specular(0.5);
    material2.set_diffuse(0.9);
    shape2.set_material(material2);
    world.add_object(shape1);
    world.add_object(shape2);
    world
}

fn main() {
    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 8.0;
    let wall_size = 7.0;

    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);
    let world = setup_world();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin.clone(), (&position - &ray_origin).get_normal());
            let all_hits = world.get_hits(&ray);

            if let Some(hit) = World::get_first_visible_hit(&all_hits) {
                let point = ray.at(hit.get_t());
                let normal = hit.get_object().get_normal(&point);
                let eye = -ray.get_direction();
                let mut color = RgbColor::new(0., 0., 0.);
                for light in world.get_lights().iter() {
                    color = &color
                        + &light.shade(hit.get_object().get_material(), &point, &eye, &normal);
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
