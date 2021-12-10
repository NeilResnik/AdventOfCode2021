pub enum PointError {
    InvalidSize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl From<((i32, i32), (i32, i32))> for Line {
    fn from(l: ((i32, i32), (i32, i32))) -> Self {
        Line {
            p1: Point::from(l.0),
            p2: Point::from(l.1),
        }
    }
}

impl From<(Point, Point)> for Line {
    fn from(l: (Point, Point)) -> Self {
        Line { p1: l.0, p2: l.1 }
    }
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }
}
