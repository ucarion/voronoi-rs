use util;

#[deriving(Copy, PartialEq, Show)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

#[deriving(Copy, PartialEq, Show)]
pub struct Edge {
    pub v1: Point,
    pub v2: Point
}

impl Edge {
    pub fn new(v1: Point, v2: Point) -> Edge {
        Edge { v1: v1, v2: v2 }
    }
}

#[deriving(Copy)]
pub struct Triangle {
    pub vertices: [Point, ..3],
    pub edges: [Edge, ..3],
    pub circumcenter: Point,
    pub circumradius: f64
}

impl Triangle {
    pub fn new(vertices: [Point, ..3], edges: [Edge, ..3]) -> Triangle {
        let circumcenter = Triangle::get_circumcenter(
            &vertices[0], &vertices[1], &vertices[2]);

        let circumradius = util::distance_between(&circumcenter, &vertices[0]);

        Triangle {
            vertices: vertices,
            edges: edges,
            circumcenter: circumcenter,
            circumradius: circumradius
        }
    }

    fn get_circumcenter(a: &Point, b: &Point, c: &Point) -> Point {
        // http://en.wikipedia.org/wiki/Circumscribed_circle#Cartesian_coordinates
        let d = 2.0 * (a.x * (b.y - c.y)
                     + b.x * (c.y - a.y)
                     + c.x * (a.y - b.y));

        let axy2 = a.x * a.x + a.y * a.y;
        let bxy2 = b.x * b.x + b.y * b.y;
        let cxy2 = c.x * c.x + c.y * c.y;

        let x = axy2 * (b.y - c.y) + bxy2 * (c.y - a.y) + cxy2 * (a.y - b.y);
        let y = axy2 * (c.x - b.x) + bxy2 * (a.x - c.x) + cxy2 * (b.x - a.x);

        Point::new(x / d, y / d)
    }

    pub fn circumcircle_contains(&self, point: &Point) -> bool {
        util::distance_between(&self.circumcenter, point) < self.circumradius
    }
}

#[test]
fn test_circumcircle() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(4.0, 0.0);
    let p3 = Point::new(0.0, 5.0);

    let e1 = Edge::new(p1, p2);
    let e2 = Edge::new(p2, p3);
    let e3 = Edge::new(p3, p1);

    let circumcenter = Point::new(2.0, 2.5);
    let circumradius = util::distance_between(&circumcenter, &p1);

    let triangle = Triangle::new([p1, p2, p3], [e1, e2, e3]);
    assert_eq!(triangle.circumcenter, circumcenter);
    assert_eq!(triangle.circumradius, circumradius);
}

#[test]
fn test_circumcircle_contains() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(4.0, 0.0);
    let p3 = Point::new(0.0, 5.0);

    let e1 = Edge::new(p1, p2);
    let e2 = Edge::new(p2, p3);
    let e3 = Edge::new(p3, p1);

    let triangle = Triangle::new([p1, p2, p3], [e1, e2, e3]);

    assert!(triangle.circumcircle_contains(&Point::new(1.0, 1.0)));
    assert!(!triangle.circumcircle_contains(&Point::new(-1.0, -1.0)));
}
