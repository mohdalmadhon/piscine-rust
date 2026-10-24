#[warn(unused)]
pub mod areas_volumes {
    pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    TriangularPyramid,
    Parallelepiped,
}

pub(crate) fn square_area(side: usize) -> usize {
    side.pow(2)
}

pub(crate) fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub(crate) fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

pub(crate) fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

pub(crate) fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

pub(crate) fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub(crate) fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

pub(crate) fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

pub(crate) fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}
}

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x,y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let r_area: usize = areas_volumes::rectangle_area(x,y);
    let area = match kind {
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a,b) as f64,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a) as f64,
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a,b) as f64
    };

    return (area as f64 * times as f64) < r_area as f64; 
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_volume = areas_volumes::parallelepiped_volume(x, y, z) as f64;

    let volume = match kind {
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };

    (volume * times as f64) <= box_volume
}

fn main() {
    println!(
        "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
        area_fit((2, 4), areas_volumes::GeometricalShapes::Rectangle, 100, (2, 1))
    );
    println!(
        "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
        area_fit((5, 5), areas_volumes::GeometricalShapes::Triangle, 3, (5, 3))
    );
    println!(
        "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
        volume_fit((5, 5, 5), areas_volumes::GeometricalVolumes::Sphere, 3, (2, 0, 0))
    );
    println!(
        "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
        volume_fit((5, 7, 5), areas_volumes::GeometricalVolumes::Parallelepiped, 1, (6, 7, 4))
    );
}