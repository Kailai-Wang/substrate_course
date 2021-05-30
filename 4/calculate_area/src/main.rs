
// trait declaration
trait HasArea {
    fn area(&self) -> f64;
}

// fn that calculates area
// it accepts a &T where T mus be of trait type HasArea and
// thus has the area() method
fn calculate_area<T : HasArea>(t: &T) -> f64 {
    t.area()
}

struct Rectangle {
    length: f64,
    height: f64,
}

struct Triangle {
    length: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.length * self.height * 0.5
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let rectangle = Rectangle { length: 1.0, height: 2.0 };
    let triangle  = Triangle  { length: 1.0, height: 2.0 };
    let circle = Circle { radius: 1.0 };

    println!("rectangle area : {}", calculate_area(&rectangle)); // 2
    println!("triangle area : {}", calculate_area(&triangle));   // 1
    println!("circle area : {:.2}", calculate_area(&circle));    // 3.14
}
