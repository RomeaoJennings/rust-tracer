use crate::primitives::{Point, RgbColor};

use super::PointLight;

#[test]
fn constructor_operates_correctly() {
    let location = Point::new(1., -2., 3.);
    let intensity = RgbColor::new(0.5, 0.6, 0.7);
    let light = PointLight::new(location.clone(), intensity.clone());

    assert_eq!(location, light.location);
    assert_eq!(intensity, light.intensity);
}

#[test] 
fn get_location_returns_correct_value() {
    let location = Point::new(1., -2., 3.);
    let intensity = RgbColor::new(0.5, 0.6, 0.7);
    let light = PointLight::new(location.clone(), intensity);

    assert_eq!(&location, light.get_location());
}

#[test]
fn get_intensity_returns_correct_value() {
    let location = Point::new(1., -2., 3.);
    let intensity = RgbColor::new(0.5, 0.6, 0.7);
    let light = PointLight::new(location, intensity.clone());
    assert_eq!(&intensity, light.get_intensity());
}