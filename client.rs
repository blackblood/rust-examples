use std::net::TcpStream;
use std::io::Read;
use std::fs;
use std::str;

fn main() {
	let mut conn = TcpStream::connect("localhost:8000").expect("could not connect to facebook.com");
	let mut buffer = [0; 12];
	if let Ok(n) = conn.read(&mut buffer[..]) {
		println!("read bytes: {}", n);
		println!("{}", str::from_utf8(&buffer).unwrap());
	} else {
		println!("Could not read data into the buffer");
	}
	fs::write("facebook.html", buffer).expect("could not write to file");
}