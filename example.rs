struct Person {
	first_name: String,
	last_name: String,
	age: u32
}

impl Person {
	fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}
}

fn main() {
	let p = Person { first_name: String::from("akshay"), last_name: String::from("takkar"), age: 27 };
	println!("full_name: {}", p.full_name());
	println!("first_name: {}", p.first_name);
}