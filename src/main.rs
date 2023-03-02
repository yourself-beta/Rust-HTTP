
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    http_request(HTTPMethod::GET, "www.google.co.uk:80", HTTPContentType::TextHtml, None, None).expect("TODO: panic message");
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
                request_content_type: HTTPContentType,
                request_body: Option<&str>,
                request_header: Option<&str>) -> std::io::Result<()>{

    let method:&str = match request_method {
        HTTPMethod::GET=> "GET",
        HTTPMethod::POST=> "POST",
        HTTPMethod::UPDATE => "UPDATE",
        HTTPMethod::DELETE=> "DELETE",
    };

    let content_type: &str = match request_content_type {
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


    let slash_index: Option<usize> = request_url.find("/");
    let url: &str;
    let path: &str;

    if let Some(index) = slash_index {
        url = &request_url[..index];
        path = &request_url[index..];
    } else {
        url = &request_url[..];
        path = "/";
    }
    println!("url:{} || path: {}", url, path);

    let mut stream = TcpStream::connect(url)?;

    stream.write_all(format!("{} / HTTP/1.0 \r\n", method).as_bytes())?;
    stream.write_all(format!("Host: {} \r\n\r\n", url).as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("{}", response);

    Ok(())
}