/// # Server Module
///
/// This module defines the basic server implementation
///
/// Brian Hession - hessionb@gmail.com

use std::net;
use std::io;

/// # Server Error
///
/// Defines different server errors
#[derive(Debug)]
pub enum Error {
	Addr(net::AddrParseError),
	Io(io::Error),
}

/// Server error methods
impl From<net::AddrParseError> for Error {
	fn from(err: net::AddrParseError) -> Error {
		Error::Addr(err)
	}
}

/// Server error methods
impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

/// # Server Structure
///
/// Defines the Server structure
#[derive(Debug)]
pub struct Server {
	addr_v4: net::SocketAddrV4,
	addr_v6: net::SocketAddrV6,
	listener_v4: net::TcpListener,
	listener_v6: net::TcpListener,
}

/// Implement drop
impl Drop for Server {
	fn drop(&mut self) {
		drop(&self.listener_v4);
		drop(&self.listener_v6);
	}
}

/// # Server methods
impl Server {

	/// # new
	///
	/// Creates a server object listening on the specified port
	pub fn new(port: u16) -> Result<Server, Error> {
		let sa4 = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), port);
		let sa6 = net::SocketAddrV6::new(net::Ipv6Addr::new(0, 0, 0, 0, 0 ,0, 0, 0), port, 0, 0);
		let s = Server {
			addr_v4: sa4,
			addr_v6: sa6,
			listener_v4: try!(net::TcpListener::bind(sa4)),
			listener_v6: try!(net::TcpListener::bind(sa6)),
		};
		//s.listener_v4.set_nonblocking(true);
		//s.listener_v6.set_nonblocking(true);
		s.listener_v6.set_only_v6(true);
		Ok(s)
	}

	/// # accept_v4
	///
	/// Accepts a connection on Ipv4
	pub fn accept(&self) -> (net::TcpStream, net::SocketAddr) {
		let l4 = self.listener_v4;
		let l6 = self.listener_v6;
		select! {
			c4 = l4.accept() => return c4,
			c6 = l6.accept() => return c6
		}
	}
}
