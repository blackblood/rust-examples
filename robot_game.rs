use std::io;
use std::fmt;

enum Direction {
	Up,
	Down,
	Left,
	Right
}

enum Command {
	FaceLeft,
	FaceRight,
	FaceUp,
	FaceDown,
	Move,
	Report,
	Place
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let val = match *self {
			Command::FaceLeft => "Facing Left",
			Command::FaceRight => "Facing Right",
			Command::FaceUp => "Facing Up",
			Command::FaceDown => "Facing Down",
		};
		write!(f, "{}", val)
	}
}


struct Table {
	length: u32,
	breadth: u32,
}

impl Table {
	fn new(length: u32, breadth: u32) -> Table {
		Table { length: length, breadth: breadth }
	}
}

struct Point {
	x: u32,
	y: u32
}

impl Point {
	fn build(x: u32, y: u32, t: Table) -> Point {
		if x < t.length && y < t.breadth {
			Point { x: x, y: y }
		} else {
			panic!("Something went wrong");
		}
	}
}

struct Robot {
	facing: Direction,
	current_position: Point
}

impl Robot {
	fn build(point: Point, direction: Direction) -> Robot {
		Robot {
			current_position: point,
			facing: direction
		}
	}
	
	fn r#move(&self) {
		match self.facing {
			Direction::Up => {
				self.current_position.y += 1;
			},
			Direction::Down => {
				self.current_position.y -= 1;
			},
			Direction::Left => {
				self.current_position.x -= 1;
			},
			Direction::Right => {
				self.current_position.x += 1;
			}
		}
	}
	
	fn face(&self, direction: Direction) {
		self.facing = direction;
	}
	
	fn report(&self) {
		println!("x = {}, y = {}", self.current_position.x, self.current_position.y);
	}
}

fn main() {
	let table = Table::new(5, 5);
	
	loop {
		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("something went wrong");
		let user_input: Vec<&str> = user_input.trim().split(" ").collect();
		
		let command = user_input[0].to_lowercase();
		
		let robot: Robot;
		let point;
		let face_direction;
		
		if command == "q" {
			break;
		}
		
		let command = if command == "left" {
					Command::FaceLeft
				} else if command == "right" {
					Command::FaceRight
				} else if command == "up" {
					Command::FaceUp
				} else if command == "down" {
					Command::FaceDown
				} else if command == "move" {
					Command::Move
				} else if command == "report" {
					Command::Report
				} else if command == "place" {
					Command::Place
				} else {
					panic!("something went wrong");
				};
		
		match command {
			Command::FaceLeft => {
				robot.face(Direction::Left);
			},
			Command::FaceRight => {
				robot.face(Direction::Right);
			},
			Command::FaceUp => {
				robot.face(Direction::Up);
			},
			Command::FaceDown => {
				robot.face(Direction::Down);
			},
			Command::Move => {
				robot.r#move();
			},
			Command::Report => {
				robot.report();
			},
			Command::Place => {
				point = Point::build(user_input[1].parse().unwrap(), user_input[2].parse().unwrap(), table);
				user_input[3].to_lowercase();
				
				robot = Robot::build(point, face_direction);
			}
		}
	}
}