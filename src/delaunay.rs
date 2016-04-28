use geometry::{Edge, Point, Triangle};

pub fn enclosing_triangle(points: &[Point]) -> Triangle {
    let xs: Vec<f64> = points.iter().map(|p| { p.x }).collect();
    let ys: Vec<f64> = points.iter().map(|p| { p.y }).collect();

    let min_x = xs.iter().fold(points[0].x, |a, b| { a.min(*b) }) - 1.0;
    let max_x = xs.iter().fold(points[0].x, |a, b| { a.max(*b) }) + 1.0;
    let min_y = ys.iter().fold(points[0].y, |a, b| { a.min(*b) }) - 1.0;
    let max_y = ys.iter().fold(points[0].y, |a, b| { a.max(*b) }) + 1.0;

    let width = 2.0 * (max_x - min_x);
    let height = 2.0 * (max_y - min_y);

    let bottom_left  = Point::new(min_x, min_y);
    let bottom_right = Point::new(max_x + width, min_y);
    let top_left     = Point::new(min_x, max_y + height);

    let edge_1 = Edge::new(bottom_left, bottom_right);
    let edge_2 = Edge::new(bottom_right, top_left);
    let edge_3 = Edge::new(top_left, bottom_left);

    Triangle::new(
        [bottom_left, bottom_right, top_left],
        [edge_1, edge_2, edge_3]
    )
}

#[test]
fn test_enclosing_triangle() {
    let points = &[
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0)
    ];

    let vertices: &[Point] = &[
        Point::new(-1.0, -1.0),
        Point::new(8.0, -1.0),
        Point::new(-1.0, 8.0)
    ];

    let triangle = enclosing_triangle(points);
    assert_eq!(triangle.vertices, vertices);
}
