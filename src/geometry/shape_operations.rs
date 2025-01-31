use std::f64::consts::PI;

use super::{
    shapes::{Circle, Rectangle, Square},
    visitor::Visitor,
};

pub struct AreaCalculator;

impl Visitor for AreaCalculator {
    fn visit_circle(&self, circle: &Circle) -> f64 {
        circle.radius.powi(2) * PI
    }

    fn visit_square(&self, square: &Square) -> f64 {
        square.side.powi(2)
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64 {
        rectangle.width * rectangle.height
    }
}
