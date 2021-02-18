use tracy::rendering::{Canvas, Color};
pub use utils::*;

mod utils;

#[test]
fn colors_are_rgb_tuples() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_f32!(c.r, -0.5);
    assert_f32!(c.g, 0.4);
    assert_f32!(c.b, 1.7);
}

#[test]
fn adding_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_abs_diff!(c1 + c2, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn subtracting_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_abs_diff!(c1 - c2, Color::new(0.2, 0.5, 0.5));
}

#[test]
fn multiplying_a_color_by_a_scalar() {
    let c = Color::new(0.2, 0.3, 0.4);
    assert_abs_diff!(c * 2., Color::new(0.4, 0.6, 0.8));
}

#[test]
fn multiplying_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert_abs_diff!(c1 * c2, Color::new(0.9, 0.2, 0.04));
}

#[test]
fn creating_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);
    c.iter().for_each(|p| assert_abs_diff!(p, Color::BLACK));
}

#[test]
fn writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1., 0., 0.);
    c.put(2, 3, red);
    assert_abs_diff!(c.get(2, 3).unwrap(), red);
}

#[test]
fn constructing_the_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.convert_to_ppm();
    assert!(ppm.starts_with("P3\n5 3\n255"));
}

#[test]
fn constructing_the_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    c.put(0, 0, Color::new(1.5, 0., 0.));
    c.put(2, 1, Color::new(0., 0.5, 0.));
    c.put(4, 2, Color::new(-0.5, 0., 1.));

    let ppm = c.convert_to_ppm();
    assert_eq!(
        ppm.lines().skip(3).take(3).collect::<Vec<_>>(),
        vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        ]
    );
}

#[test]
fn splitting_long_lines_in_ppm_file() {
    let mut c = Canvas::new(10, 2);
    for pixel in c.iter_mut() {
        *pixel = Color::new(1., 0.8, 0.6);
    }

    let ppm = c.convert_to_ppm();
    assert_eq!(
        ppm.lines().skip(3).take(4).collect::<Vec<_>>(),
        vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
        ]
    );
}

#[test]
fn ppm_files_are_terminated_by_a_newline_character() {
    let c = Canvas::new(5, 3);
    let ppm = c.convert_to_ppm();
    assert_eq!(ppm.chars().last(), Some('\n'));
}
