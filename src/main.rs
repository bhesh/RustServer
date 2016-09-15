/// # RustServer
///
/// A simple server to demonstrate Rust functionality
///
/// Brian Hession - hessionb@gmail.com

mod net;
use net::server;

fn main() {
	let s = server::Server::new(8080).expect("Unable to create the server");
	println!("Server: {:?}", s);
}
