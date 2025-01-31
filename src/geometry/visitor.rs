use super::shapes::{Circle, Rectangle, Square};

pub trait Visitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_square(&self, square: &Square);
    fn visit_rectangle(&self, rectangle: &Rectangle);
}
