use std::f64::consts::PI;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, dest: &Point) -> f64 {
        ((dest.x - self.x).powf(2.0) + (dest.y - self.y).powf(2.0)).sqrt()
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) < self.radius &&
        other.center.distance(&self.center) < other.radius
    }
}