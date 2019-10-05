fn divide(a: u32, b: u32) -> Result<u32, &'static str> {
	if b == 0 {
		return Err("Divide by zero error");
	}
	return Ok(a / b);
}

fn main() {
	let x: u32 = 10;
	let y: u32 = 2;
	let res = divide(x,y).unwrap();
	println!("{}", res);
}