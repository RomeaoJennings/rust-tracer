use super::RgbColor;

#[test]
fn constructor_correctly_assigns_values() {
    let red = 0.4;
    let green = 0.6;
    let blue = 0.9;

    let color = RgbColor::new(red, green, blue);

    assert_eq!(red, color.red);
    assert_eq!(green, color.green);
    assert_eq!(blue, color.blue);
}

#[test]
fn getters_operate_correctly() {
    let red = 0.4;
    let green = 0.6;
    let blue = 0.9;

    let color = RgbColor::new(red, green, blue);

    assert_eq!(red, color.get_red());
    assert_eq!(green, color.get_green());
    assert_eq!(blue, color.get_blue());
}

#[test]
fn add_two_colors_computes_resulting_color() {
    let c1 = &RgbColor::new(0.9, 0.6, 0.75);
    let c2 = &RgbColor::new(0.7, 0.1, 0.25);
    let sum = RgbColor::new(1.6, 0.7, 1.0);

    assert_eq!(c1 + c2, sum)
}

#[test]
fn subtract_two_colors_computes_resulting_color() {
    let c1 = &RgbColor::new(1.9, 0.6, 0.75);
    let c2 = &RgbColor::new(0.7, 0.1, 0.25);
    let sum = RgbColor::new(1.2, 0.5, 0.5);

    assert_eq!(c1 - c2, sum);
}

#[test]
fn scalar_multiplication_computes_correctly() {
    let color = &RgbColor::new(1., 2., 3.);
    let scalar = 2.;

    let actual = RgbColor::new(2., 4., 6.);

    assert_eq!(actual, color * scalar);
}

#[test]
fn color_multiplication_computes_correctly() {
    let c1 = &RgbColor::new(1., 0.2, 0.5);
    let c2 = &RgbColor::new(0.9, 1., 0.1);

    assert_eq!(c1 * c2, RgbColor::new(0.9, 0.2, 0.05));
}
