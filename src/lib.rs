use std::io::Write;
use std::io::Result;

pub struct Header; // TODO: Flesh out

pub struct Request {
    pub verb     : Verb,
    pub resource : String,
    pub body     : Option<String>,
    pub headers  : Vec<Header>,
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

    let bytes_written =
        try!(out.write(format!("{:?}", request.verb).as_bytes())) +
        try!(out.write(b" ")) +
        try!(out.write(&*request.resource.as_bytes())) +
        try!(out.write(b" HTTP/1.1"));

    Ok(bytes_written)
}

///
/// Request the resource and try to upgrade the connection
///
pub fn request_try_upgrade(request: &Request, out: &mut Write) -> Result<usize> {
    // TODO: This function assumes that the write calls write the entire buffer passed
    //       to it. This is not the case, and should be handeled properly.

    let bytes_written =
        try!(request_line_1_1(request, out));

    // TODO: Other headers

    Ok(bytes_written)
}

#[cfg(test)]
mod tests {
    use super::{ request_line_1_1, Verb, Request };

    /// Shim while waiting for String::from_str to stabilize or be replaced
    fn create_string(slice: &str) -> String {
        let mut string = String::new();
        string.push_str(slice);

        string
    }

    #[test]
    fn test_request_line_1_1() {
        let mut out = Vec::new();

        let request = Request { verb:     Verb::GET,
                                resource: create_string("/index.html"),
                                body:     None,
                                headers:  vec![],
                                };

        request_line_1_1(&request, &mut out).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "GET /index.html HTTP/1.1");
    }

}

