use util;

pub type Vertex = [f64, ..2];
pub type Edge = [Vertex, ..2];

pub type Point = Vertex;

#[deriving(Copy)]
pub struct Triangle {
    pub vertices: [Vertex, ..3],
    pub edges: [Edge, ..3],
    pub circumcenter: Point,
    pub circumradius: f64
}

impl Triangle {
    pub fn new(vertices: [Vertex, ..3], edges: [Edge, ..3]) -> Triangle {
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

    fn get_circumcenter(a: &Vertex, b: &Vertex, c: &Vertex) -> Point {
        // http://en.wikipedia.org/wiki/Circumscribed_circle#Cartesian_coordinates
        let d = 2.0 * (a[0] * (b[1] - c[1])
                     + b[0] * (c[1] - a[1])
                     + c[0] * (a[1] - b[1]));

        let axy2 = a[0] * a[0] + a[1] * a[1];
        let bxy2 = b[0] * b[0] + b[1] * b[1];
        let cxy2 = c[0] * c[0] + c[1] * c[1];

        let x = axy2 * (b[1] - c[1])
                + bxy2 * (c[1] - a[1])
                + cxy2 * (a[1] - b[1]);

        let y = axy2 * (c[0] - b[0])
                + bxy2 * (a[0] - c[0])
                + cxy2 * (b[0] - a[0]);

        [x / d, y / d]
    }

    pub fn circumcircle_contains(&self, point: &Point) -> bool {
        util::distance_between(&self.circumcenter, point) < self.circumradius
    }
}

#[test]
fn test_circumcircle() {
    let p1 = [0.0, 0.0];
    let p2 = [4.0, 0.0];
    let p3 = [0.0, 5.0];

    let e1 = [p1, p2];
    let e2 = [p2, p3];
    let e3 = [p3, p1];

    let circumcenter: Point = [2.0, 2.5];
    let circumradius = util::distance_between(&circumcenter, &p1);

    let triangle = Triangle::new([p1, p2, p3], [e1, e2, e3]);
    assert_eq!(triangle.circumcenter, circumcenter);
    assert_eq!(triangle.circumradius, circumradius);
}

#[test]
fn test_circumcircle_contains() {
    let p1 = [0.0, 0.0];
    let p2 = [4.0, 0.0];
    let p3 = [0.0, 5.0];

    let e1 = [p1, p2];
    let e2 = [p2, p3];
    let e3 = [p3, p1];

    let triangle = Triangle::new([p1, p2, p3], [e1, e2, e3]);

    assert!(triangle.circumcircle_contains(&[1.0, 1.0]));
    assert!(!triangle.circumcircle_contains(&[-1.0, -1.0]));
}
