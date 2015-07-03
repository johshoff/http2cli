use std::io::Write;
use std::io::Result;

//const HTTP_2_PREFACE : &'static [u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub struct Header<'a> {
	pub key:   &'a str,
	pub value: &'a str,
}

pub struct Request<'a> {
    pub verb     : Verb,
    pub resource : &'a str,
    pub body     : Option<&'a str>,
    pub headers  : Vec<Header<'a>>,
    pub host     : &'a str,
}

#[derive(Debug)]
pub enum Verb {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

///
/// Generate the first request line in HTTP/1.1
///
pub fn request_line_1_1(request: &Request, out: &mut Write) -> Result<usize> {
    // TODO: This function assumes that the write calls write the entire buffer passed
    //       to it. This is not the case, and should be handeled properly.
	// TODO: Encode resource

    let bytes_written =
        try!(out.write(format!("{:?}", request.verb).as_bytes())) +
        try!(out.write(b" ")) +
        try!(out.write(request.resource.as_bytes())) +
        try!(out.write(b" HTTP/1.1\n"));

    Ok(bytes_written)
}

pub fn header_field_1_1(header: &Header, out: &mut Write) -> Result<usize> {
    // TODO: This function assumes that the write calls write the entire buffer passed
    //       to it. This is not the case, and should be handeled properly.
	// TODO: Encode headers

    let bytes_written =
        try!(out.write(header.key.as_bytes())) +
        try!(out.write(b": ")) +
        try!(out.write(header.value.as_bytes())) +
        try!(out.write(b"\n"));

    Ok(bytes_written)
}

///
/// Request the resource and try to upgrade the connection
///
pub fn request_try_upgrade(request: &Request, out: &mut Write) -> Result<usize> {
    // TODO: This function assumes that the write calls write the entire buffer passed
    //       to it. This is not the case, and should be handeled properly.

    let mut bytes_written =
        try!(request_line_1_1(request, out)) +
        try!(header_field_1_1(&Header { key: "Host", value: request.host }, out)) +
        try!(header_field_1_1(&Header { key: "Connection", value: "Upgrade, HTTP2-Settings" }, out)) +
        try!(header_field_1_1(&Header { key: "Upgrade", value: "h2c" }, out)) +
        try!(header_field_1_1(&Header { key: "HTTP2-Settings", value: "???" }, out)) +
        try!(out.write(b"\n"));

	for header in &request.headers {
		bytes_written += try!(header_field_1_1(header, out));
	}

    Ok(bytes_written)
}

#[cfg(test)]
mod tests {
    use super::{ request_line_1_1, request_try_upgrade, Verb, Request };

    #[test]
    fn test_request_line_1_1() {
        let mut out = Vec::new();

        let request = Request { verb:     Verb::GET,
                                resource: "/index.html",
                                body:     None,
                                headers:  vec![],
                                host:     "example.com",
                                };

        request_line_1_1(&request, &mut out).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "GET /index.html HTTP/1.1\n");
    }

	#[test]
    fn test_request_try_upgrade() {
        let mut out = Vec::new();

        let request = Request { verb:     Verb::GET,
                                resource: "/index.html",
                                body:     None,
                                headers:  vec![],
                                host:     "example.com",
                                };

        request_try_upgrade(&request, &mut out).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "GET /index.html HTTP/1.1\nHost: example.com\nConnection: Upgrade, HTTP2-Settings\nUpgrade: h2c\nHTTP2-Settings: ...\n\n");
    }

}

