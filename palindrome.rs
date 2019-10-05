use std::io;

fn main() {
	let mut s = String::new();
	io::stdin().read_line(&mut s).expect("something went wrong in the input");
	s.pop();
	let size = s.len() - 1;
	let mut end = size;
	let mut palin: bool = true;
	
	let s_bytes = s.as_bytes();
	for start in 0..(size / 2) {
		if s_bytes[start] != s_bytes[end] {
			palin = false;
			break;
		}
		end -= 1;
	}
	
	println!("The input string is a palindrome? : {}", palin);
}