use std::collections::HashMap;

fn main() {
	let s = "akshay";
	let chars = s.chars();
	let mut cache = HashMap::new();
	
	for ch in chars {
		let count = cache.entry(ch).or_insert(0);
		*count += 1;
	}
	println!("{:?}", cache);
}