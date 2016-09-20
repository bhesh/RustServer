/// # RustServer
///
/// A simple server to demonstrate Rust functionality
///
/// Brian Hession - hessionb@gmail.com

extern crate mio;

mod server;

use server::handler;
use std::io;
use std::io::Write;

/// Handles the request
fn handle_request(req: &handler::Request, res: &handler::Response) {}

/// Main entry point
fn main() {
	let port: u16 = 8080;
	let mut server = server::Server::new(port, handle_request, 1024).expect("Error: Unable to create the server");
	println!("Starting server on port {}", port);
	match server.start() {
		Ok(_) => {}
		Err(e) => { write!(io::stderr(), "Error: {}", e.to_string()).unwrap(); }
	}
}
