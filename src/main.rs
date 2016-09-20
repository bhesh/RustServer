/// # RustServer
///
/// A simple server to demonstrate Rust functionality
///
/// Brian Hession - hessionb@gmail.com

extern crate mio;

mod server;

fn main() {
	let server = server::Server::new(8080, 1024).expect("Unable to create the server");
	let (sock, addr) = server.start().expect("Error accepting");
	println!("Sock: {:?}", sock);
	println!("Addr: {:?}", addr);
}
