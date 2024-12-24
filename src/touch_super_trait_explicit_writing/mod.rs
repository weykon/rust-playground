// 显式说明一下supertrait的写法
trait Shape {
    fn area(&self) -> f64;
}

trait Circle: Shape {
    fn radius(&self) -> f64 {
        // A = pi * r^2
        // so algebraically,
        // r = sqrt(A / pi)
        (self.area() / std::f64::consts::PI)
            .sqrt()
    }
}
fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}
struct CircleStruct {
    radius: f64,
}
impl Shape for CircleStruct {
    fn area(&self) -> f64 {
        std::f64::consts::PI
            * self.radius
            * self.radius
    }
}
impl Circle for CircleStruct {
    fn radius(&self) -> f64 {
        // A = pi * r^2
        // so algebraically,
        // r = sqrt(A / pi)
        (self.area() / std::f64::consts::PI)
            .sqrt()
    }
}
fn run() {
    let circle = Box::new(CircleStruct {
        radius: 2.,
    }) as Box<dyn Circle>;
    let nonsense =
        circle.radius() * circle.area();
}
