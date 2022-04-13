use rust_tracer::{
    objects::Sphere,
    primitives::{Canvas, Point, Ray, RgbColor, SquareMatrix},
};

fn main() {
    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels, None);
    let color = RgbColor::new(1., 0.5, 0.25);
    let mut shape = Sphere::new(Point::new(0., 0.5, 0.), 0.5);
    let transform = SquareMatrix::shearing(-0.3, 0., 0., 0.2, 0., 0.);
    shape.set_transform(transform);
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin.clone(), (&position - &ray_origin).get_normal());
            if shape.get_hits(&ray).len() > 0 {
                canvas.set_pixel(y, x, color.clone());
            }
        }
    }
    let result = canvas.to_ppm("C:/Users/romea/source/rust-tracer/output.ppm");
    if result.is_err() {
        println!("{:?}", result.err());
    }
}
