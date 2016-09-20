/// # Server Module
///
/// This module defines the basic server implementation
///
/// Brian Hession - hessionb@gmail.com

/// Defines the HTTP request
pub struct Request {
}

/// Defines the HTTP response
pub struct Response {
}

/// Defines how the server should handle the request
pub trait HandlesRequests {
	fn handle_request(&self, req: Request, res: Response);
}
