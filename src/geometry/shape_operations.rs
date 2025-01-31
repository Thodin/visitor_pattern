use std::f64::consts::PI;

use super::{
    shapes::{Circle, Rectangle, Square},
    visitor::Visitor,
};

pub struct AreaCalculator;

impl Visitor for AreaCalculator {
    fn visit_circle(&self, circle: &Circle) {
        println!(
            "Area of circle with radius {:.4}: {:.4}",
            circle.radius,
            circle.radius.powi(2) * PI
        );
    }

    fn visit_square(&self, square: &Square) {
        println!(
            "Area of square with side {:.4}: {:.4}",
            square.side,
            square.side.powi(2)
        );
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) {
        println!(
            "Area of rectangle with width {:.4} and height {:.4}: {:.4}",
            rectangle.width,
            rectangle.height,
            rectangle.width * rectangle.height
        );
    }
}
