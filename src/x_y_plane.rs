pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum Line {
    Horizontal(i32),
    Vertical(i32),
}

pub enum Graph {
    Point(Point),
    Line(Line),
}

impl Graph {
    pub fn show(&self) {
        match self {
            Graph::Line(dir) => match dir {
                Line::Horizontal(y) => println!("Line at y: {y}"),
                Line::Vertical(x) => println!("Line at x: {x}"),
            },
            Graph::Point(point) => println!("Point at x: {}, y:{}", point.x, point.y),
        };
    }
}

impl Point {
    pub fn distance(&self, other: &Point) -> f32 {
        let f_x: f32 = (square_i32(self.x - other.x)).abs() as f32;
        let f_y: f32 = (square_i32(self.y - other.y)).abs() as f32;

        (f_x + f_y).sqrt()
    }

    pub fn distance_to_line(&self, lin: &Line) -> u32 {
        match lin {
            Line::Horizontal(lin_y) => (self.y - lin_y).abs() as u32,
            Line::Vertical(lin_x) => (self.x - lin_x).abs() as u32,
        }
    }

    pub fn show(&self) {
        println!("Point at x: {}, y:{}", self.x, self.y);
    }
}

impl Line {
    pub fn distance(&self, other: &Line) -> u32 {
        match (self, other) {
            (Line::Horizontal(y1), Line::Horizontal(y2)) => (y1 - y2).abs() as u32,

            (Line::Vertical(x1), Line::Vertical(x2)) => (x1 - x2).abs() as u32,

            _ => 0,
        }
    }

    pub fn distance_to_point(&self, point: &Point) -> u32 {
        match self {
            Line::Horizontal(y) => (point.y - y).abs() as u32,
            Line::Vertical(x) => (point.x - x).abs() as u32,
        }
    }
}

pub fn square_i32(x: i32) -> i32 {
    x * x
}
