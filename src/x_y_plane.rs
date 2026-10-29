#![allow(unused)]

pub trait Graph {
    fn summarize(&self) -> String;
    fn print(&self) {
        println!("{}", self.summarize());
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

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

}

impl Graph for Point {
    fn summarize(&self) -> String {
        format!("Point at ({}, {})", self.x, self.y)
    }
}


pub enum Line {
    Horizontal(i32),
    Vertical(i32),
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

impl Graph for Line {
     fn summarize(&self) -> String {
        match self {
            Self::Horizontal(x) => format!("Horizontal Line at x: {x}"),
            Self::Vertical(y) => format!("Vertical Line at y: {y}")
        }
    }
}

pub struct Circle {
    center: Point,
    radius: u32
}

impl Circle{
    pub fn new( center: Point, radius: u32) -> Self {
        Self { center, radius }
    }

    pub fn new_on_origin(radius: u32) -> Self {
        Self { center: Point { x: 0, y: 0 }, radius }
    }

    pub fn base_origin(&mut self) {
        self.center = Point { x: 0, y: 0 }
    }

    pub fn make_from(point: &Point, radius: u32) -> Self {
        Self { center: *point, radius }
    }

    pub fn move_center(&mut self, point: &Point) {
        self.center = *point;
    }
}

impl Graph for Circle {
    fn summarize(&self) -> String {
        format!("Circle: center at ({}, {}), radius: {}", self.center.x, self.center.y, self.radius)
    }
}

pub fn square_i32(x: i32) -> i32 {
    x * x
}
