use rust_tracer::{
    lighting::PointLight,
    objects::Sphere,
    primitives::{Canvas, Point, Ray, RgbColor},
    shading::Material,
};

fn main() {
    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 8.0;
    let wall_size = 7.0;

    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);
    let mut shape = Sphere::new(Point::new(0., 0.0, 0.), 1.0);
    //let transform = SquareMatrix::shearing(-0.3, 0., 0., 0.2, 0., 0.);
    //shape.set_transform(transform);
    let mut material = Material::default();
    material.set_color(RgbColor::new(0.6, 0.9, 1.));
    shape.set_material(material);

    let light = PointLight::new(Point::new(-10., 10., -10.), RgbColor::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin.clone(), (&position - &ray_origin).get_normal());
            let hits = shape.get_hits(&ray);
            if hits.len() > 0 {
                let point = ray.at(hits[0].get_t());
                let normal = hits[0].get_object().get_normal(&point);
                let eye = -ray.get_direction();
                let mut color = light.shade(hits[0].get_object().get_material(), point, eye, normal);
                color.gamma_correct();
                color.clamp();
                canvas.set_pixel(y, x, color);
            }
        }
    }
    let result = canvas.to_ppm("C:/Users/romea/source/rust-tracer/output.ppm");
    if result.is_err() {
        println!("{:?}", result.err());
    }
}
