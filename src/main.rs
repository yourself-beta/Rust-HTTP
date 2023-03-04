mod request_definitions;
use request_definitions::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // http_request(RequestMethod::GET, "www.google.co.uk/test", RequestPort::HTTP, RequestContentType::TextHtml, None, None).expect("TODO: panic message");
    let test = Frame {
        request_method: RequestMethod::GET,
        request_url: String::from("www.google.com/test"),
        request_port: RequestPort::HTTP,
        request_content_type: RequestContentType::TextHtml,
    };
    let (a, b) = test.get_url();
    println!("{}", a);
    println!("{}", b);
    test.create_header();
}



// fn http_request(request_method: RequestMethod,
//                 request_url: &str,
//                 request_port: RequestPort,
//                 request_content_type: RequestContentType,
//                 request_body: Option<&str>,
//                 request_header: Option<&str>) -> std::io::Result<()>{
//
//
//     let mut stream = TcpStream::connect(&url)?;
//
//     stream.write_all(format!("{} / HTTP/1.0 \r\n", method).as_bytes())?;
//     stream.write_all(format!("Host: {} \r\n\r\n", url).as_bytes())?;
//     let mut response = String::new();
//     stream.read_to_string(&mut response)?;
//
//     println!("{}", response);
//
//     Ok(())
// }
