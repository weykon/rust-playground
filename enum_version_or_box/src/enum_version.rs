use std::time::Instant;

use rand::Rng;
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Square(Square),
    Triangle(Triangle),
}

impl Shape {
    fn area (&self) -> f32 {
        match self {
            Shape::Rectangle(r) => r.area(),
            Shape::Circle(c) =>  c.area(),
            Shape::Square(s) =>  s.area(),
            Shape::Triangle(t) =>  t.area(),
        }
    }
}
struct Rectangle {
    width: f32,
    height: f32,
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }
}

impl Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * (self.radius * self.radius) as f32
    }
}

impl Square {
    fn area(&self) -> f32 {
        (self.side * self.side) as f32
    }
}

impl Triangle {
    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}

fn total_area(shapes: &[Shape] ) -> f32 {
    shapes.iter().fold(0.0, |a, s| a + s.area())
}
const N: usize = 1_000_000;
const N_WARMUP: usize = 100;
fn init_shapes() -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::with_capacity(3 * N as usize);
    let mut rng = rand::thread_rng();

    for _ in 0..3 * N {
        match rng.gen_range(0..3) {
            0 => {
                shapes.push(Shape::Rectangle(Rectangle {
                    width: 4.0,
                    height: 4.0,
                }));
            }
            1 => {
                shapes.push(Shape::Triangle(Triangle {
                    base: 4.0,
                    height: 4.0,
                }));
            }
            _ => {
                shapes.push(Shape::Square(Square { side: 4.0 }));
            }
        }
    }
    shapes
}

pub fn main() {
    let shapes: Vec<Shape> = init_shapes();

    // Running benchmark
    let start = Instant::now();
    let mut total = 0.0;
    for _ in 0..N_WARMUP {
        total = total_area(&shapes);
    }
    let duration = start.elapsed();

    println!("{}. Enum took {:?}.", total, duration / N_WARMUP as u32);
}
