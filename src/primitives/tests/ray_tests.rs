use crate::primitives::{Point, Vector};

use super::Ray;

#[test]
fn constructor_operates_correctly() {
    let origin = Point::new(1., -2., 0.5);
    let direction = Vector::new(5., 3., 2.);

    let ray = Ray::new(origin.clone(), direction.clone());

    assert_eq!(ray.get_direction(), &direction);
    assert_eq!(ray.get_origin(), &origin);
}

#[test]
fn at_computes_correctly() {
    let ray = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));

    assert_eq!(ray.at(0.), Point::new(2., 3., 4.));
    assert_eq!(ray.at(1.), Point::new(3., 3., 4.));
    assert_eq!(ray.at(-1.), Point::new(1., 3., 4.));
    assert_eq!(ray.at(2.5), Point::new(4.5, 3., 4.));
}
