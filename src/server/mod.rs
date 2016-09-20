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
use std::io::Write;
use std::fmt;

/// Set the tokens
const SERVER_V4: mio::Token = mio::Token(0);
const SERVER_V6: mio::Token = mio::Token(1);

/*********************************************************************
 * SERVER IMPLEMENTATION
 *********************************************************************/

/// Defines the Server structure
pub struct Server {
	addr_v4: net::SocketAddrV4,
	addr_v6: net::SocketAddrV6,
	listener_v4: tcp::TcpListener,
	listener_v6: tcp::TcpListener,
	func: fn(&handler::Request, &handler::Response),
	backlog: usize,
	conns: Vec<ClientThread>,
}

/// Define Debug
impl fmt::Debug for Server {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{ {:?}, {:?}, {:?}, {:?}, backlog: {} }}", 
				self.addr_v4, self.addr_v6, 
				self.listener_v4, self.listener_v6,
				self.backlog)
	}
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
	pub fn new(port: u16, func: fn(&handler::Request, &handler::Response), backlog: usize) -> Result<Server, Error> {
		let sa4 = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), port);
		let sa6 = net::SocketAddrV6::new(net::Ipv6Addr::new(0, 0, 0, 0, 0 ,0, 0, 0), port, 0, 0);
		let s = Server {
			addr_v4: sa4,
			addr_v6: sa6,
			listener_v4: try!(tcp::TcpListener::bind(&net::SocketAddr::V4(sa4))),
			listener_v6: try!(tcp::TcpListener::bind(&net::SocketAddr::V6(sa6))),
			func: func,
			backlog: backlog,
			conns: Vec::new(),
		};
		Ok(s)
	}

	/// Starts the server loop
	pub fn start(&mut self) -> Result<(), Error> {
		let poll = try!(mio::Poll::new());
		try!(poll.register(&self.listener_v4, SERVER_V4, mio::Ready::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()));
		try!(poll.register(&self.listener_v6, SERVER_V6, mio::Ready::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()));
		let mut events = mio::Events::with_capacity(self.backlog);
		loop {
			try!(poll.poll(&mut events, None));
			for event in events.iter() {
				match event.token() {
					SERVER_V4 => { self.conns.push(Server::make_client_thread(try!(self.listener_v4.accept()))); }
					SERVER_V6 => { self.conns.push(Server::make_client_thread(try!(self.listener_v6.accept()))); }
					_ => { unreachable!(); }
				}
			}
		}
	}

	/// Make client thread
	fn make_client_thread(conn: (tcp::TcpStream, net::SocketAddr)) -> ClientThread {
		let (sock, addr) = conn;
		ClientThread {
			addr: addr,
			sock: sock,
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
		match self.sock.shutdown(tcp::Shutdown::Both) {
			Ok(_) => {}
			Err(e) => { write!(io::stderr(), "Error: {}", e.to_string()).unwrap(); }
		}
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

/// Implement display
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
