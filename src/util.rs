use std::num::Float;

use graph::Point;

pub fn distance_between(a: &Point, b: &Point) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

#[test]
fn test_distance_between() {
    let p1 = &[0.0, 0.0];
    let p2 = &[3.0, 4.0];

    assert_eq!(distance_between(p1, p2), 5.0);
}
