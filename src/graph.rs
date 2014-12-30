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

