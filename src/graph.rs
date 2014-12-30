use std::num::FloatMath;

pub type Vertex = [f64, ..2];
pub type Edge = [Vertex, ..2];

#[deriving(Copy)]
pub struct Triangle {
    vertices: [Vertex, ..3],
    edges: [Edge, ..3]
}

pub fn enclosing_triangle(points: &[Vertex]) -> Triangle {
    let xs: Vec<f64> = points.iter().map(|p| { p[0] }).collect();
    let ys: Vec<f64> = points.iter().map(|p| { p[1] }).collect();

    let min_x = xs.iter().fold(points[0][0], |a, b| { a.min(*b) }) - 1.0;
    let max_x = xs.iter().fold(points[0][0], |a, b| { a.max(*b) }) + 1.0;
    let min_y = ys.iter().fold(points[1][0], |a, b| { a.min(*b) }) - 1.0;
    let max_y = ys.iter().fold(points[1][0], |a, b| { a.max(*b) }) + 1.0;

    let width = 2.0 * (max_x - min_x);
    let height = 2.0 * (max_y - min_y);

    let bottom_left  = [min_x, min_y];
    let bottom_right = [max_x + width, min_y];
    let top_left     = [min_x, max_y + height];

    let edge_1 = [bottom_left, bottom_right];
    let edge_2 = [bottom_right, top_left];
    let edge_3 = [top_left, bottom_left];

    Triangle {
        vertices: [bottom_left, bottom_right, top_left],
        edges: [edge_1, edge_2, edge_3]
    }
}

#[test]
fn test_enclosing_triangle() {
    let points = &[
        [0.0, 0.0],
        [1.0, 1.0],
    ];

    let vertices: &[Vertex] = &[
        [-1.0, -1.0],
        [8.0, -1.0],
        [-1.0, 8.0]
    ];

    let triangle = enclosing_triangle(points);
    assert_eq!(triangle.vertices, vertices);
}

