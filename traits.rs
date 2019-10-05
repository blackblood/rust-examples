use std::fmt;

struct Person {
	name: String,
	email: String,
	mobile: String,
	age: u32
}

struct Dog {
	name: String,
	age: u32
}

pub trait Age {
	fn get_age(&self) -> u32;
}

impl Age for Person {
	fn get_age(&self) -> u32 {
		self.age
	}
}

impl Age for Dog {
	fn get_age(&self) -> u32 {
		self.age % 10
	}
}

impl fmt::Display for Person {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}, {}, {}", self.name, self.email, self.mobile, self.age)
	}
}

impl fmt::Display for Dog {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.name, self.age)
	}
}

fn main() {
	let user = Person {
		name: "Akshay".to_string(),
		email: "akshay.takkar@gmail.com".to_string(),
		mobile: "9920018835".to_string(),
		age: 27
	};
	
	let dog = Dog {
		name: "Doggo".to_string(),
		age: 6
	};
	
	println!("{}", user);
	println!("{}", dog);
	
	println!("Calling get_age");
	
	println!("{}", user.get_age());
	println!("{}", dog.get_age());
}