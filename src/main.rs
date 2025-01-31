use geometry::{
    shape_operations::AreaCalculator,
    shapes::{Circle, Rectangle, Shape, Square},
};

pub mod geometry;

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 0.7 }),
        Box::new(Square { side: 0.9 }),
        Box::new(Rectangle {
            height: 1.2,
            width: 0.4,
        }),
    ];

    let area_calculator = AreaCalculator {};

    for shape in &shapes {
        shape.accept(&area_calculator);
    }
}
