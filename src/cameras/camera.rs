use crate::primitives::{Canvas, Point, Ray, SquareMatrix, Vector};

pub struct Camera {
    height: usize,
    width: usize,
    fov_in_radians: f64,
    transform: SquareMatrix,
    inverted: SquareMatrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(canvas: &Canvas, fov_in_radians: f64) -> Self {
        let half_view = (fov_in_radians / 2.0).tan();
        let aspect = canvas.get_width() as f64 / canvas.get_height() as f64;
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = half_width * 2.0 / canvas.get_width() as f64;
        Camera {
            height: canvas.get_height(),
            width: canvas.get_width(),
            fov_in_radians,
            transform: SquareMatrix::identity(4),
            inverted: SquareMatrix::identity(4),
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn set_position(&mut self, from: &Point, look_at: &Point, up_vector: &Vector) {
        let mut forward = (look_at - from).get_normal();
        let mut up = up_vector.get_normal();
        let left = &forward ^ &up;
        up = &left ^ &forward;
        let mut transform = SquareMatrix::identity(4);
        forward = -&forward;
        transform.set_all(&[
            (0, 0, left.get_x()),
            (0, 1, left.get_y()),
            (0, 2, left.get_z()),
            (1, 0, up.get_x()),
            (1, 1, up.get_y()),
            (1, 2, up.get_z()),
            (2, 0, forward.get_x()),
            (2, 1, forward.get_y()),
            (2, 2, forward.get_z()),
        ]);
        self.transform =
            &transform * &SquareMatrix::translation(-from.get_x(), -from.get_y(), -from.get_z());
        self.inverted = self.transform.invert();
    }

    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let x = x.clamp(0, self.width) as f64;
        let y = y.clamp(0, self.height) as f64;

        let x_offset = (x + 0.5) * self.pixel_size;
        let y_offset = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = &self.inverted * &Point::new(world_x, world_y, -1.0);
        let origin = &self.inverted * &Point::new(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).get_normal();
        Ray::new(origin, direction)
    }
}
