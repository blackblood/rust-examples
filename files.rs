use std::fs::File;
use std::io::Read;

fn count_words(text: &str) -> u32 {
	let words: Vec<&str> = text.split(" ").collect();
	words.len() as u32
}

fn count_chars(text: &str) -> u32 {
	let chars: Vec<char> = text.chars().collect();
	chars.len() as u32
}

fn main() {
	let mut f = File::open("hello.txt").expect("could not open hello.txt");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("could not read file");
	println!("word count: {}", count_words(&contents));
	println!("chars count: {}", count_chars(&contents));
}