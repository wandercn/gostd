#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bytes::Bytes;
use gostd_builtin::*;
use gostd_io::*;
use gostd_strings as strings;
use gostd_url as url;

use crate::{error::HttpResult, header::Header, method::Method};

#[derive(Default, Clone, Debug)]
pub struct Request {
    pub Method: String,
    pub URL: url::URL,
    Proto: String,
    ProtoMajor: int,
    ProtoMinor: int,
    pub Header: Header,
    pub Body: Option<Bytes>,
    // GetBody func() (io.ReadCloser, error)
    pub ContentLength: int64,
    TransferEncoding: Vec<String>,
    pub Close: bool,
    pub Host: String,
    Form: url::Values,
    PostForm: url::Values,
    // MultipartForm:*multipart.Form,
    Trailer: Header,
    RemoteAddr: String,
    RequestURI: String,
    pub isTLS: bool,
    // TLS *tls.ConnectionState,
    // Cancel <-chan struct{}
    // ctx context.Context
}
impl Request {
    pub fn New(method: Method, url: &str, body: Option<Bytes>) -> HttpResult<Request> {
        let mut u = url::Parse(url)?;

        u.Host = removeEmptyPort(u.Host.as_str()).to_string();
        let mut req = Request {
            Method: method.String().to_owned(),
            URL: u.clone(),
            Proto: "HTTP/1.1".to_string(),
            ProtoMajor: 1,
            ProtoMinor: 1,
            Header: Header::default(),
            ContentLength: 0,
            TransferEncoding: Vec::<String>::new(),
            Close: false,
            Form: url::Values::default(),
            PostForm: url::Values::default(),
            Trailer: Header::default(),
            RemoteAddr: "".to_string(),
            RequestURI: "".to_string(),
            Body: None,
            Host: u.Host.to_owned(),
            isTLS: false,
        };
        if let Some(buf) = body {
            req.ContentLength = len!(buf) as i64;

            req.Body = Some(buf);
        }
        if strings::HasPrefix(url, "https://") {
            req.isTLS = true
        }
        Ok(req)
    }

    pub fn Write(&self) -> HttpResult<Vec<u8>> {
        self.write(false)
    }

    fn write(&self, usingProxy: bool) -> HttpResult<Vec<u8>> {
        let mut buf = strings::Builder::new();
        let host = self.Host.clone();
        let ruri = self.URL.RequestURI();
        let userAgent = "rust-http-client/1.1";
        buf.WriteString(format!("{} {} HTTP/1.1\r\n", self.Method.as_str(), ruri).as_str());
        buf.WriteString(format!("Host: {}\r\n", host).as_str());
        buf.WriteString(format!("User-Agent: {}\r\n", userAgent).as_str());
        buf.WriteString(self.writeHeader().as_str());
        buf.WriteString("\r\n");
        if let Some(body) = &self.Body {
            buf.Write(body.to_vec())?;
            buf.WriteString("\r\n");
        }
        Ok(buf.Bytes())
    }

    fn writeHeader(&self) -> String {
        let mut buf = strings::Builder::new();
        for (k, v) in &self.Header.0 {
            if len!(v) > 1 {
                let value = strings::Join(v.iter().map(|x| x.as_str()).collect(), ",");
                buf.WriteString(format!("{}: {}\r\n", k.as_str(), value.as_str()).as_str());
            } else {
                buf.WriteString(format!("{}: {}\r\n", k.as_str(), v[0].as_str()).as_str());
            }
        }
        if self.ContentLength > 0 {
            buf.WriteString(format!("Content-Length: {}\r\n", self.ContentLength).as_str());
        }
        buf.String()
    }
}

fn hasPort(s: &str) -> bool {
    strings::LastIndex(s, ":") > strings::LastIndex(s, "]")
}

// removeEmptyPort strips the empty port in ":port" to ""
// as mandated by RFC 3986 Section 6.2.3.
fn removeEmptyPort(host: &str) -> &str {
    if hasPort(host) {
        return strings::TrimSuffix(host, ":");
    }
    host
}
