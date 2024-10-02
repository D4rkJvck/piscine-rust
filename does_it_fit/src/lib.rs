pub mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    use GeometricalShapes::*;

    let rect_area = rectangle_area(x, y);

    let shape_area = match objects {
        Square => square_area(a),
        Triangle => triangle_area(a, b) as usize,
        Circle => circle_area(a) as usize,
        Rectangle => rectangle_area(a, b),
    };

    shape_area * times <= rect_area
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    use GeometricalVolumes::*;

    let box_volume = parallelepiped_volume(x, y, z);

    let body_volume = match objects {
        Cube => cube_volume(a),
        Sphere => sphere_volume(a) as usize,
        Pyramid => triangle_area(a, b) as usize,
        Cone => cone_volume(a, b) as usize,
        Parallelepiped => parallelepiped_volume(a, b, c),
    };

    body_volume * times <= box_volume
}
