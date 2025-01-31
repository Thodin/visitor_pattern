use super::visitor::Visitor;

pub trait Shape {
    fn accept(&self, visitor: &dyn Visitor) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn Visitor) -> f64 {
        visitor.visit_circle(self)
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &dyn Visitor) -> f64 {
        visitor.visit_rectangle(self)
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn accept(&self, visitor: &dyn Visitor) -> f64 {
        visitor.visit_square(self)
    }
}
