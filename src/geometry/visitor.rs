use super::shapes::{Circle, Rectangle, Square};

pub trait Visitor {
    fn visit_circle(&self, circle: &Circle) -> f64;
    fn visit_square(&self, square: &Square) -> f64;
    fn visit_rectangle(&self, rectangle: &Rectangle) -> f64;
}
