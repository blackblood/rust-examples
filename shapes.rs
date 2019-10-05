trait Shape {
	fn area(&self) -> f64;
	fn print(&self);
}

struct Rectangle {
	top_left: (i32, i32),
	length: u32,
	breadth: u32
}

struct Circle {
	center: (i32, i32),
	radius: f32
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		(3.14 * self.radius * self.radius) as f64
	}
	fn print(&self) {
		println!("center: {} {}, radius: {}", self.center.0, self.center.1, self.radius);
	}
}

impl Shape for Rectangle {
	fn area(&self) -> f64 {
		(self.length * self.breadth) as f64
	}
	fn print(&self) {
		println!("top_left: {} {}, length: {}, breadth: {}", self.top_left.0, self.top_left.1, self.length, self.breadth);
	}
}

fn main() {
	let rect: Rectangle = Rectangle { top_left: (4,5), length: 10, breadth: 5 };
	rect.print();
	println!("{}", rect.area());

	let cir: Circle = Circle { center: (4, 5), radius: 5.0 };
	cir.print();
	println!("{}", cir.area());
}