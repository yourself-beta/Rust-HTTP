use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    http_request(HTTPMethod::GET, "www.google.co.uk:80", HTTPContentType::TextHtml).expect("TODO: panic message");
}

enum HTTPMethod{
    GET,
    POST,
    UPDATE,
    DELETE,
}

enum HTTPContentType{
    TextHtml,
    TextPlain,
    ApplicationJson,
    ApplicationXml,
    ApplicationOctetStream,
    MultipartFormData,
    ApplicationXWwwFormUrlEncoded,
    ImageJpeg,
    ImagePng,
    AudioMpeg,
    VideoMp4,
}

fn http_request(request_method: HTTPMethod,
                request_url: &str,
                //request_body: String,
                //request_header: String,
                request_content_type: HTTPContentType) -> std::io::Result<()>{

    let mut method:&str = match request_method {
        HTTPMethod::GET=> "GET",
        HTTPMethod::POST=> "POST",
        HTTPMethod::UPDATE => "UPDATE",
        HTTPMethod::DELETE=> "DELETE",
    };

    let mut content_type: &str = match request_content_type {
        HTTPContentType::TextHtml => "text/html",
        HTTPContentType::TextPlain => "text/plain",
        HTTPContentType::ApplicationJson => "application/json",
        HTTPContentType::ApplicationXml => "application/xml",
        HTTPContentType::ApplicationOctetStream => "application/octet-stream",
        HTTPContentType::MultipartFormData => "multipart/form-data",
        HTTPContentType::ApplicationXWwwFormUrlEncoded => "application/x-www-form-urlencoded",
        HTTPContentType::ImageJpeg => "image/jpeg",
        HTTPContentType::ImagePng => "image/png",
        HTTPContentType::AudioMpeg => "audio/mpeg",
        HTTPContentType::VideoMp4 => "video/mp4",
    };



    let mut stream = TcpStream::connect(request_url)?;
    stream.write(format!("{} / HTTP/1.0\r\n", method).as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("{}", response);

    Ok(())
}