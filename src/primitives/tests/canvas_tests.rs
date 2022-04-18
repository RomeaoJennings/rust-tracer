use crate::primitives::RgbColor;

use super::Canvas;

#[test]
fn new_canvas_creates_with_proper_size() {
    let height = 25;
    let width = 20;
    let canvas = Canvas::new(width, height, None);

    assert_eq!(canvas.get_height(), height);
    assert_eq!(canvas.get_width(), width);
}

#[test]
fn new_canvas_has_minimum_size_of_1x1() {
    let canvas = Canvas::new(0,0,None);
    assert_eq!(1, canvas.get_height());
    assert_eq!(1, canvas.get_width());
}

#[test]
fn new_canvas_creates_correct_pixel_color() {
    let color = RgbColor::new(1.,1.,0.);

    let canvas = Canvas::new(10,10,Some(color.clone()));

    assert_eq!(color, canvas.get_pixel(5,5));
}

#[test]
fn set_pixel_updates_correctly() {
    let color = RgbColor::new(1.,1.,0.);
    let mut canvas = Canvas::new(10,10, None);

    canvas.set_pixel(5,5,color.clone());

    assert_eq!(color, canvas.get_pixel(5,5));
}