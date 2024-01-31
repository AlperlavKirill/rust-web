pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const CREATED_RESPONSE: &str = "HTTP/1.1 201 CREATED\r\n\r\n";


pub const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
pub const NOT_AUTHORIZED: &str = "HTTP/1.1 401 UNAUTHORIZED\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const ALREADY_EXISTS: &str = "HTTP/1.1 409 CONFLICT\r\n\r\n";


pub const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
pub const NOT_IMPLEMENTED: &str = "HTTP/1.1 501 NOT IMPLEMENTED\r\n\r\n";