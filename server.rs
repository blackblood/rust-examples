use std::net::{TcpStream, TcpListener};
use std::io;
use std::io::Write;

fn handle_connection(conn: &mut TcpStream) -> io::Result<()> {
	let data = "Hello World";
	println!("Sending: {}", data);
	conn.write(data.as_bytes())?;
	Ok(())
}

fn main() -> io::Result<()> {
	let conn = TcpListener::bind("localhost:8000")?;
	println!("Listening on localhost:8000...");
	for stream in conn.incoming() {
		handle_connection(&mut stream?)?;
	}
	Ok(())
}