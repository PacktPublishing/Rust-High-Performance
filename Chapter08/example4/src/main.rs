#[macro_use]
extern crate nom;

use std::str;

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Method {
    fn from_bytes(b: &[u8]) -> Result<Self, String> {
        match b {
            b"GET" => Ok(Method::Get),
            b"POST" => Ok(Method::Post),
            b"PUT" => Ok(Method::Put),
            b"DELETE" => Ok(Method::Delete),
            _ => {
                let error = format!(
                    "invalid method: {}",
                    str::from_utf8(b).unwrap_or("not UTF-8")
                );
                Err(error)
            }
        }
    }
}

#[derive(Debug)]
struct Request {
    method: Method,
    url: String,
    version: String,
}

named!(parse_method<&[u8], Method>,
    alt!(
        map_res!(tag!("GET"), Method::from_bytes) |
        map_res!(tag!("POST"), Method::from_bytes) |
        map_res!(tag!("PUT"), Method::from_bytes) |
        map_res!(tag!("DELETE"), Method::from_bytes)
    )
);

named!(parse_request<&[u8], Request>, ws!(do_parse!(
    method: parse_method >>
    url: map_res!(take_until!(" "), str::from_utf8) >>
    tag!("HTTP/") >>
    version: map_res!(take_until!("\r"), str::from_utf8) >>
    (Request {
        method,
        url: url.to_owned(),
        version: version.to_owned()
    })
)));

fn main() {
    let first_line = b"GET /home/ HTTP/1.1\r\n";
    println!("{:?}", parse_request(&first_line[..]));
}
