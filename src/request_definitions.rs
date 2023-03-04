
pub enum RequestMethod {
    GET,
    POST,
    UPDATE,
    DELETE,
}

pub enum RequestContentType {
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

pub enum RequestPort {
    HTTP,
    HTTPS,
}

pub struct Frame {
    pub request_method: RequestMethod,
    pub request_url: String,
    pub request_port: RequestPort,
    pub request_content_type: RequestContentType,
}

impl Frame {
    pub fn get_method(&self) -> &str {
        let method: &str = match self.request_method {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::UPDATE => "UPDATE",
            RequestMethod::DELETE => "DELETE",
        };
        method
    }
    pub fn get_url(&self) -> (&str, &str) {
        let slash_index: Option<usize> = self.request_url.find("/");
        let host: &str;
        let path: &str;
        if let Some(index) = slash_index {
            host = &self.request_url[..index];
            path = &self.request_url[index..];
        } else {
            host = &self.request_url[..];
            path = "/";
        };
        println!("url:{} || path: {}", host, path);
        (host, path)
    }
    pub fn get_port(&self) -> &str {
        let port: &str = match self.request_port {
            RequestPort::HTTP => "80",
            RequestPort::HTTPS => "443",
        };
        port
    }
    pub fn get_content_type(&self) -> &str {
        let content_type: &str = match &self.request_content_type {
            RequestContentType::TextHtml => "text/html",
            RequestContentType::TextPlain => "text/plain",
            RequestContentType::ApplicationJson => "application/json",
            RequestContentType::ApplicationXml => "application/xml",
            RequestContentType::ApplicationOctetStream => "application/octet-stream",
            RequestContentType::MultipartFormData => "multipart/form-data",
            RequestContentType::ApplicationXWwwFormUrlEncoded => {
                "application/x-www-form-urlencoded"
            }
            RequestContentType::ImageJpeg => "image/jpeg",
            RequestContentType::ImagePng => "image/png",
            RequestContentType::AudioMpeg => "audio/mpeg",
            RequestContentType::VideoMp4 => "video/mp4",
        };
        content_type
    }
    pub fn create_header(&self) -> String {
        let mut header = format!(
            "{} {} HTTP/1.0 \r\nHost: {} \r\n\r\n",
            self.get_method(),
            self.get_url().1,
            self.get_url().0
        );
        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_method() -> () {
        let frame = Frame {
            request_method: RequestMethod::GET,
            request_url: String::from("www.youtube.com/feed/library"),
            request_port: RequestPort::HTTP,
            request_content_type: RequestContentType::TextHtml,
        };
        assert_eq!(frame.get_method(), "GET");
    }

    #[test]
    fn test_get_url() -> () {
        let frame = Frame {
            request_method: RequestMethod::GET,
            request_url: String::from("www.youtube.com/feed/library"),
            request_port: RequestPort::HTTP,
            request_content_type: RequestContentType::TextHtml,
        };
        assert_eq!(frame.get_url(), ("www.youtube.com", "/feed/library"));
    }

    #[test]
    fn test_get_port() -> () {
        let frame = Frame {
            request_method: RequestMethod::GET,
            request_url: String::from("www.youtube.com/feed/library"),
            request_port: RequestPort::HTTP,
            request_content_type: RequestContentType::TextHtml,
        };
        assert_eq!(frame.get_port(), "80");
    }

    #[test]
    fn test_get_content_type() -> () {
        let frame = Frame {
            request_method: RequestMethod::GET,
            request_url: String::from("www.youtube.com/feed/library"),
            request_port: RequestPort::HTTP,
            request_content_type: RequestContentType::TextHtml,
        };
        assert_eq!(frame.get_content_type(), "text/html");
    }

    #[test]
    fn test_create_header() -> () {
        let frame = Frame {
            request_method: RequestMethod::GET,
            request_url: String::from("www.youtube.com/feed/library"),
            request_port: RequestPort::HTTP,
            request_content_type: RequestContentType::TextHtml,
        };
        assert_eq!(frame.create_header(), "TODO");
    }
}