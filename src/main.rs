use std::time::Instant;

use geometry::{
    shape_operations::AreaCalculator,
    shapes::{Circle, Rectangle, Shape, Square},
};
use rand::{rngs::ThreadRng, Rng};

pub mod geometry;

fn main() {
    run_and_time(create_shapes_and_calculate_area, 1_000_000);
}

fn create_shapes_and_calculate_area(rng: &mut ThreadRng) -> f64 {
    let random_num = rng.gen_range(0.5..1.5);

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: random_num }),
        Box::new(Circle { radius: random_num }),
        Box::new(Circle { radius: random_num }),
        Box::new(Circle { radius: random_num }),
        Box::new(Circle { radius: random_num }),
        Box::new(Square { side: random_num }),
        Box::new(Square { side: random_num }),
        Box::new(Square { side: random_num }),
        Box::new(Square { side: random_num }),
        Box::new(Square { side: random_num }),
        Box::new(Rectangle {
            height: random_num,
            width: random_num,
        }),
        Box::new(Rectangle {
            height: random_num,
            width: random_num,
        }),
        Box::new(Rectangle {
            height: random_num,
            width: random_num,
        }),
        Box::new(Rectangle {
            height: random_num,
            width: random_num,
        }),
        Box::new(Rectangle {
            height: random_num,
            width: random_num,
        }),
    ];

    let area_calculator = AreaCalculator {};

    let mut sum = 0_f64;
    for shape in &shapes {
        sum += shape.accept(&area_calculator);
    }
    sum
}

fn run_and_time<T>(fun: T, num_iterations: u32)
where
    T: Fn(&mut ThreadRng) -> f64,
{
    let mut rng = rand::thread_rng();
    let mut sum = 0_f64;

    let start = Instant::now();
    for _ in 0..num_iterations {
        sum += fun(&mut rng);
    }
    let duration = start.elapsed();

    println!("Duration: {:?}", duration);
    println!("Sum: {sum}");
}
