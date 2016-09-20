/// # Server Module
///
/// This module defines the basic server implementation
///
/// Brian Hession - hessionb@gmail.com

pub mod handler;

use mio;
use mio::tcp;
use std::net;
use std::io;

const SERVER_V4: mio::Token = mio::Token(0);
const SERVER_V6: mio::Token = mio::Token(1);

/*********************************************************************
 * SERVER IMPLEMENTATION
 *********************************************************************/

/// Defines the Server structure
#[derive(Debug)]
pub struct Server {
	backlog: usize,
	addr_v4: net::SocketAddrV4,
	addr_v6: net::SocketAddrV6,
	listener_v4: tcp::TcpListener,
	listener_v6: tcp::TcpListener,
}

/// Implement drop
impl Drop for Server {
	fn drop(&mut self) {
		drop(&self.listener_v4);
		drop(&self.listener_v6);
	}
}

/// Server methods
impl Server {

	/// Creates a server object listening on the specified port
	pub fn new(port: u16, backlog: usize) -> Result<Server, Error> {
		let sa4 = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), port);
		let sa6 = net::SocketAddrV6::new(net::Ipv6Addr::new(0, 0, 0, 0, 0 ,0, 0, 0), port, 0, 0);
		let s = Server {
			backlog: backlog,
			addr_v4: sa4,
			addr_v6: sa6,
			listener_v4: try!(tcp::TcpListener::bind(&net::SocketAddr::V4(sa4))),
			listener_v6: try!(tcp::TcpListener::bind(&net::SocketAddr::V6(sa6))),
		};
		Ok(s)
	}

	/// Starts the server
	pub fn start(&self) -> Result<(tcp::TcpStream, net::SocketAddr), Error> {
		let poll = try!(mio::Poll::new());
		try!(poll.register(&self.listener_v4, SERVER_V4, mio::Ready::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()));
		try!(poll.register(&self.listener_v6, SERVER_V6, mio::Ready::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()));
		let mut events = mio::Events::with_capacity(self.backlog);
		loop {
			try!(poll.poll(&mut events, None));
			for event in events.iter() {
				match event.token() {
					SERVER_V4 => { return Ok(try!(self.listener_v4.accept())); }
					SERVER_V6 => { return Ok(try!(self.listener_v6.accept())); }
					_ => { unreachable!(); }
				}
			}
		}
	}
}

/*********************************************************************
 * HANDLES THE CLIENT
 *********************************************************************/

/// Defines the server handler
struct ClientThread {
	addr: net::SocketAddr,
	sock: tcp::TcpStream,
}

/// Implements a destructor
impl Drop for ClientThread {
	fn drop(&mut self) {
		self.sock.shutdown(tcp::Shutdown::Both);
	}
}

/// Functions for the handler
impl ClientThread {

	/// Handles the server connection
	fn handle_client<T>(&self, handler: T) where T: handler::HandlesRequests {
		
	}
}

/*********************************************************************
 * ERROR IMPLEMENTATION
 *********************************************************************/

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
