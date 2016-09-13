use std::io::Write;
use std::net::TcpListener;

/// # RustServer
///
/// A simple server to demonstrate Rust functionality
///
/// Brian Hession - hessionb@gmail.com
fn main() {

	// Todo: Allow arguments to define these
	let host = "127.0.0.1";
	let port = 8080;
	println!("Listening on {}:{}", host, port);

	// Create a new listener
	let listener = TcpListener::bind(&format!("{}:{}", host, port)[..]).unwrap();

	// Accept a connection
	match listener.accept() {
		Ok((_, addr)) => { println!("Got connection from {:?}", addr); }
		Err(e) => { write!(std::io::stderr(), "{}", e.to_string()).unwrap(); }
	}

	// Clean up the listener
	drop(listener);
}
