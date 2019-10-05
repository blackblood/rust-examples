fn sum(arr: &[u32]) -> u32 {
	let mut s = 0;
	for i in arr.iter() {
		s += i;
	}
	return s;
}

fn square_it(arr: &mut [u32]) {
	let last = arr.len();
	for i in 0..last {
		arr[i] = arr[i] * arr[i];
	}
}

fn main() {
	let mut arr = [1,2,3,4,5];
	
	for (i, el) in arr.iter().enumerate() {
		println!("index: {}", i);
		println!("value: {}", el);
	}
	
	println!("sum: {}", sum(&arr));
	
	square_it(&mut arr);
	for el in arr.iter() {
		println!("{}", el);
	}
}