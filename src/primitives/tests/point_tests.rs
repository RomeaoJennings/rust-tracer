use super::Point;


#[test]
fn new_correctly_assigns_values() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.x);
    assert_eq!(expected_y, point.y);
    assert_eq!(expected_z, point.z);
}

#[test]
fn get_x_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.get_x());
}

#[test]
fn get_y_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_y, point.get_y());
}

#[test]
fn get_z_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_z, point.get_z());
}