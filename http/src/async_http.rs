#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(feature = "tokio-runtime")]
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
#[cfg(feature = "tokio-runtime")]
use tokio_rustls::{client::TlsStream, rustls, TlsConnector};

#[cfg(feature = "async-std-runtime")]
use async_std::{
    io::{BufReadExt, BufReader, ReadExt, WriteExt},
    net::TcpStream,
    prelude::*,
};

use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    sync::Arc,
};

use crate::{
    cookies::{Cookie, CookieJar},
    error::{HTTPConnectError, HttpResult},
    header::Header,
    method::Method,
    request::Request,
    response::Response,
};
use bytes::{Bytes, BytesMut};
use gostd_builtin::*;
use gostd_strings as strings;
use gostd_time as time;
use gostd_url as url;
/// Get issues a GET to the specified URL. If the response is one of the following redirect codes, Get follows the redirect,up to a maximum of 10 redirects:
/// ```text
/// 301 (Moved Permanently)
/// 302 (Found)
/// 303 (See Other)
/// 307 (Temporary Redirect)
/// 308 (Permanent Redirect)
/// ```
/// Get is a wrapper around Client.Get.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Get向指定的URL发出一个GET请求，如果回应的状态码如下，Get会在调用c.CheckRedirect后执行重定向
///Get是对包变量Client的Get方法的包装。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_http::async_http;
/// // or use gostd::net::http::async_http;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
///    let response = async_http::Get(url).await?;
///    println!(
///        "{}",
///        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
///    );
///    Ok(())
///}
/// ```
pub async fn Get(url: &str) -> HttpResult<Response> {
    AsyncClient::New().Get(url).await
}

pub async fn Head(url: &str) -> HttpResult<Response> {
    AsyncClient::New().Head(url).await
}

/// Post issues a POST to the specified URL. Post is a wrapper around DefaultClient.Post.
/// To set custom headers, use Request::New and Client.Do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Post向指定的URL发出一个POST请求。bodyType为POST数据的类型， body为POST数据，作为请求的主体
/// </details>
///
/// # Example
///
/// ```
/// use gostd_http::async_http;
/// // or gostd::net::http::async_http;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url = "https://petstore.swagger.io/v2/pet";
///     let postbody = r#"{"id":0,"category":{"id":0,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":0,"name":"string"}],"status":"available"}"#
///     .as_bytes()
///     .to_vec();
///     let response = async_http::Post(url, "application/json", Some(postbody.into())).await?;///
///     println!(
///         "{}",
///         String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
///     );///
///     Ok(())
/// }
/// ```
pub async fn Post(url: &str, contentType: &str, body: Option<Bytes>) -> HttpResult<Response> {
    AsyncClient::New().Post(url, contentType, body).await
}

pub async fn PostForm(url: &str, data: url::Values) -> HttpResult<Response> {
    AsyncClient::New().PostForm(url, data).await
}

pub async fn Patch(url: &str, body: Option<Bytes>) -> HttpResult<Response> {
    AsyncClient::New().Patch(url, body).await
}

pub async fn Put(url: &str, body: Option<Bytes>) -> HttpResult<Response> {
    AsyncClient::New().Put(url, body).await
}

pub async fn AsyncClient(url: &str) -> HttpResult<Response> {
    AsyncClient::New().Delete(url).await
}
// Async Client
pub struct AsyncClient {
    transport: Transport,
    jar: Box<dyn CookieJar>,
    timeout: time::Duration,
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self {
            transport: Transport::default(),
            jar: Box::new(Cookie::default()),
            timeout: time::Duration::new(0),
        }
    }
}

impl AsyncClient {
    pub fn New() -> Self {
        Self::default()
    }

    pub async fn Get(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Get, url, None)?;
        self.Do(&mut req).await
    }

    pub async fn Post(
        &mut self,
        url: &str,
        content_type: &str,
        body: Option<Bytes>,
    ) -> HttpResult<Response> {
        let mut req = Request::New(Method::Post, url, body)?;
        req.Header.Set("Content-Type", content_type);
        self.Do(&mut req).await
    }

    pub async fn PostForm(&mut self, url: &str, data: url::Values) -> HttpResult<Response> {
        self.Post(
            url,
            "application/x-www-form-urlencoded",
            Some(data.Encode().into_bytes().into()),
        )
        .await
    }

    pub async fn Head(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Head, url, None)?;
        self.Do(&mut req).await
    }

    pub async fn Patch(&mut self, url: &str, body: Option<Bytes>) -> HttpResult<Response> {
        let mut req = Request::New(Method::Patch, url, body)?;
        self.Do(&mut req).await
    }

    pub async fn Put(&mut self, url: &str, body: Option<Bytes>) -> HttpResult<Response> {
        let mut req = Request::New(Method::Put, url, body)?;
        self.Do(&mut req).await
    }

    pub async fn Delete(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Delete, url, None)?;
        self.Do(&mut req).await
    }

    pub async fn Do(&mut self, req: &mut Request) -> HttpResult<Response> {
        self.done(req).await
    }

    async fn send(
        &mut self,
        req: &mut Request,
        deadline: time::Time,
    ) -> HttpResult<(Response, fn() -> bool)> {
        let (resp, did_timeout) = send(req, self.transport(), deadline).await?;
        Ok((resp, did_timeout))
    }

    async fn done(&mut self, req: &mut Request) -> HttpResult<Response> {
        let deadline = self.deadline();
        let (resp, _did_timeout) = self.send(req, deadline).await?;
        Ok(resp)
    }

    fn deadline(&mut self) -> time::Time {
        if self.timeout > time::Duration::new(0) {
            return time::Now().Add(&self.timeout);
        }
        time::Time::default()
    }

    fn transport(&self) -> Transport {
        Transport::default()
    }
}

async fn send(
    ireq: &mut Request,
    mut rt: Transport,
    deadline: time::Time,
) -> HttpResult<(Response, fn() -> bool)> {
    let mut resp = Response::default();
    fn did_timeout() -> bool {
        false
    };
    loop {
        let mut resp = rt.round_trip(ireq).await?;
        let loc = resp.Header.Get("Location");
        let (redirect_method, should_redirect, include_body) =
            redirect_behavior(ireq.Method.as_str(), &resp, ireq);
        if !should_redirect {
            return Ok((resp, did_timeout));
        }
        let u = ireq.URL.Parse(loc.as_str())?;
        let url_ref = referer_for_url(&ireq.URL, &u);
        ireq.Method = redirect_method.clone();
        ireq.URL = u.clone();
        ireq.Header.Set("Referer", url_ref.as_str());
    }
}

fn redirect_behavior(req_method: &str, resp: &Response, ireq: &Request) -> (String, bool, bool) {
    let mut should_redirect = false;
    let mut include_body = false;
    match resp.StatusCode {
        301 | 302 | 303 => return (Method::Get.String().to_string(), true, false),
        307 | 308 => {
            if resp.Header.Get("Location") == "" {
                should_redirect = true;
                include_body = false;
            }
            if ireq.Body.is_none() && ireq.ContentLength != 0 {
                should_redirect = false;
            }
        }
        _ => (),
    }
    (req_method.to_string(), should_redirect, include_body)
}

trait AsyncRoundTripper {
    async fn round_trip(&mut self, r: &Request) -> HttpResult<Response>;
}

fn referer_for_url(last_req: &url::URL, new_req: &url::URL) -> String {
    if last_req.Scheme == "https" && new_req.Scheme == "http" {
        return "".to_string();
    }
    let mut referer = last_req.String();
    if let Some(user) = last_req.User.clone() {
        return referer;
    }
    let auth = "@";
    referer = strings::Replace(referer.as_str(), auth, "", 1);
    referer
}

#[derive(Default, Clone)]
struct Transport {
    close_idle: bool,
    proxy: Option<url::URL>,
    force_attempt_http2: bool,
    max_idle_conns: i32,
    disable_keep_alives: bool,
    disable_compression: bool,
    i_max_idle_conns_per_host: i32,
    max_conns_per_host: i32,
    max_response_header_bytes: i64,
    write_buffer_size: i32,
    read_buffer_size: i32,
    tls_next_proto_was_nil: bool,
}

impl AsyncRoundTripper for Transport {
    async fn round_trip(&mut self, req: &Request) -> HttpResult<Response> {
        self.round_trip(req).await
    }
}

impl Transport {
    async fn round_trip(&mut self, req: &Request) -> HttpResult<Response> {
        let treq = &mut transportRequest {
            Req: req.clone(),
            extra: None,
        };
        let cm = self.connect_method_for_request(treq)?;
        let (mut pconn, conn) = self.get_conn(treq, cm).await?;
        pconn.round_trip(treq, conn).await
    }

    async fn get_conn(
        &mut self,
        treq: &transportRequest,
        cm: connectMethod,
    ) -> HttpResult<(persistConn, TcpStream)> {
        let conn = self.dial_conn(cm).await?;
        let pconn = persistConn::default();
        Ok((pconn, conn))
    }

    async fn dial_conn(&mut self, cm: connectMethod) -> HttpResult<TcpStream> {
        self.dial("tcp", cm.addr().as_str()).await
    }

    async fn dial(&mut self, network: &str, addr: &str) -> HttpResult<TcpStream> {
        Ok(TcpStream::connect(addr).await?)
    }

    fn connect_method_for_request(&mut self, treq: &transportRequest) -> HttpResult<connectMethod> {
        let mut cm = connectMethod::default();
        cm.target_scheme = treq.Req.URL.Scheme.clone();
        cm.target_addr = canonical_addr(&treq.Req.URL.clone());
        cm.proxy_url = None;
        cm.only_h1 = true;
        Ok(cm)
    }

    fn wirte_buffer_size(self) -> i32 {
        if self.write_buffer_size > 0 {
            return self.write_buffer_size;
        }
        4 << 10
    }

    fn read_buffer_size(self) -> i32 {
        if self.read_buffer_size > 0 {
            return self.read_buffer_size;
        }
        4 << 10
    }
}

fn canonical_addr(url: &url::URL) -> String {
    let port_map: HashMap<String, String> = [
        ("http".to_string(), "80".to_string()),
        ("https".to_string(), "443".to_string()),
        ("socks5".to_string(), "1080".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let addr = url.Hostname().to_string();
    let mut port = url.Port().to_string();
    if port == "" {
        port = port_map.get(url.Scheme.as_str()).unwrap().to_string();
    }
    strings::Join(vec![addr.as_str(), port.as_str()], ":")
}

#[derive(Default, Clone)]
struct transportRequest {
    pub Req: Request,
    extra: Option<Header>,
}

impl transportRequest {
    fn extra_headers(&mut self) -> Header {
        if let Some(extra) = self.extra.clone() {
            return extra;
        }
        Header::default()
    }
}

#[derive(Default, PartialEq, PartialOrd, Clone)]
struct connectMethod {
    proxy_url: Option<url::URL>,
    target_scheme: String,
    target_addr: String,
    only_h1: bool,
}

impl connectMethod {
    fn scheme(&self) -> String {
        self.target_scheme.clone()
    }

    fn addr(&self) -> String {
        self.target_addr.clone()
    }
}

type TcpConn = TcpStream;

#[derive(Default, Clone)]
struct persistConn {
    t: Transport,
    nwrite: i64,
    is_proxy: bool,
    saw_eof: bool,
    read_limit: i64,
    num_expected_responses: i32,
    broken: bool,
    reused: bool,
}

impl persistConn {
    async fn round_trip(
        &mut self,
        req: &mut transportRequest,
        mut conn: TcpConn,
    ) -> HttpResult<Response> {
        self.num_expected_responses += 1;
        let mut requested_gzip = false;
        if !self.t.disable_compression
            && req.Req.Header.Get("Accept-Encoding") == ""
            && req.Req.Header.Get("Range") == ""
            && req.Req.Method != "HEAD".to_string()
        {
            requested_gzip = true;
        }
        if req.Req.Close {
            req.Req.Header.Set("Connection", "close");
        }

        let r = req.Req.Write()?;
        #[cfg(feature = "tokio-runtime")]
        {
            if req.Req.isTLS {
                let mut tls_conn = get_tls_conn(req.Req.Host.as_str(), conn).await?;
                tls_conn.write_all(r.as_slice()).await?;
                let mut reader = tokio::io::BufReader::new(tls_conn);
                let resp = read_response(&mut reader, &req.Req).await?;
                Ok(resp)
            } else {
                conn.write_all(r.as_slice()).await?;
                let mut reader = tokio::io::BufReader::new(conn);
                let resp = read_response(&mut reader, &req.Req).await?;
                Ok(resp)
            }
        }
        #[cfg(feature = "async-std-runtime")]
        {
            if req.Req.isTLS {
                let mut tls_conn = get_tls_conn(req.Req.Host.as_str(), conn).await?;
                tls_conn.write_all(r.as_slice()).await?;
                let mut reader = BufReader::new(tls_conn);
                let resp = read_response(&mut reader, &req.Req).await?;
                Ok(resp)
            } else {
                conn.write_all(r.as_slice()).await?;
                let mut reader = BufReader::new(conn);
                let resp = read_response(&mut reader, &req.Req).await?;
                Ok(resp)
            }
        }
    }
}

#[cfg(feature = "tokio-runtime")]
fn get_tls_config() -> Arc<rustls::ClientConfig> {
    let mut root_cert_store = rustls::RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    Arc::new(
        rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth(),
    )
}

#[cfg(feature = "tokio-runtime")]
async fn get_tls_conn(dns_name: &str, socket: TcpStream) -> HttpResult<TlsStream<TcpStream>> {
    let tls_config = get_tls_config();
    let server_name = dns_name.to_owned().try_into()?;
    let connector = TlsConnector::from(tls_config.clone());
    let tls_stream = connector.connect(server_name, socket).await?;
    Ok(tls_stream)
}

#[cfg(feature = "async-std-runtime")]
use async_tls::{client::TlsStream, TlsConnector};
// 使用 async-std 运行时
#[cfg(feature = "async-std-runtime")]
async fn get_tls_conn(
    dns_name: &str,
    socket: async_std::net::TcpStream,
) -> HttpResult<TlsStream<async_std::net::TcpStream>> {
    let server_name = dns_name.to_owned();
    let connector = TlsConnector::default();
    let tls_stream = connector.connect(server_name, socket).await?;
    Ok(tls_stream)
}
#[cfg(feature = "tokio-runtime")]
async fn read_response<R>(mut r: R, req: &Request) -> HttpResult<Response>
where
    R: AsyncBufRead + Unpin,
{
    let mut resp = Response {
        request: req.clone(),
        ..Default::default()
    };

    // Parse status line.
    let mut line = String::new();
    r.read_line(&mut line).await?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP response".to_string(),
        ));
    }
    resp.Proto = parts[0].to_string();
    resp.Status = parts[1..].join(" ");
    resp.StatusCode = parts[1].parse::<isize>().unwrap_or(-1);
    let vers = parse_http_version(&resp.Proto);
    let ok = vers.2;
    if !ok {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP version".to_string(),
        ));
    }
    resp.ProtoMajor = vers.0;
    resp.ProtoMinor = vers.1;

    // Get response headers until the first "\r\n".
    let mut head_part = BytesMut::new();
    let mut head_line = String::new();
    loop {
        head_line.clear();
        r.read_line(&mut head_line).await?;
        if head_line.as_bytes() == b"\r\n" {
            break;
        }
        head_part.extend_from_slice(head_line.as_bytes());
    }

    // Parse headers.
    resp.Header = Header::NewWithHashMap(parse_header(&head_part)?);
    fix_pragma_cache_control(&mut resp.Header);

    // Set body based on transfer encoding or content length.
    if resp.Header.Get("Transfer-Encoding") == "chunked" {
        resp.Body = Some(parse_chunked_body(&mut r).await?);
    } else {
        let ln: usize = resp
            .Header
            .Get("Content-Length")
            .parse::<usize>()
            .expect("Content-Length is not exist");
        let mut buf = vec![0; ln];
        r.read_exact(&mut buf).await?;
        resp.Body = Some(BytesMut::from(&buf[..]));
    }

    resp.ContentLength = resp.Body.as_ref().map_or(0, |b| b.len() as i64);
    Ok(resp)
}
#[cfg(feature = "tokio-runtime")]
async fn parse_chunked_body<R>(mut r: R) -> HttpResult<BytesMut>
where
    R: AsyncBufRead + Unpin,
{
    let mut body = BytesMut::new();
    let mut size_buf = vec![];
    while r.read_until(b'\n', &mut size_buf).await.is_ok() {
        if size_buf.ends_with(b"\r\n") {
            size_buf.truncate(size_buf.len() - 2); // Remove "\r\n"
            let size_str = std::str::from_utf8(&size_buf)?;
            if size_str == "0" {
                break;
            }
            let chunk_size = usize::from_str_radix(size_str, 16)?;
            let mut chunk_data = vec![0u8; chunk_size];
            r.read_exact(&mut chunk_data).await?;
            body.extend_from_slice(&chunk_data);
            let mut crlf = [0u8; 2];
            r.read_exact(&mut crlf).await?;
            size_buf.clear();
        }
    }
    Ok(body)
}

#[cfg(feature = "async-std-runtime")]
async fn read_response<R>(mut r: R, req: &Request) -> HttpResult<Response>
where
    R: BufReadExt + Unpin,
{
    let mut resp = Response {
        request: req.clone(),
        ..Default::default()
    };

    // Parse status line.
    let mut line = String::new();

    r.read_line(&mut line).await?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP response".to_string(),
        ));
    }
    resp.Proto = parts[0].to_string();
    resp.Status = parts[1..].join(" ");
    resp.StatusCode = parts[1].parse::<isize>().unwrap_or(-1);
    let vers = parse_http_version(&resp.Proto);
    let ok = vers.2;
    if !ok {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP version".to_string(),
        ));
    }
    resp.ProtoMajor = vers.0;
    resp.ProtoMinor = vers.1;

    // Get response headers until the first "\r\n".
    let mut head_part = BytesMut::new();
    let mut head_line = String::new();
    loop {
        head_line.clear();
        r.read_line(&mut head_line).await?;
        if head_line.as_bytes() == b"\r\n" {
            break;
        }
        head_part.extend_from_slice(head_line.as_bytes());
    }

    // Parse headers.
    resp.Header = Header::NewWithHashMap(parse_header(&head_part)?);
    fix_pragma_cache_control(&mut resp.Header);

    // Set body based on transfer encoding or content length.
    if resp.Header.Get("Transfer-Encoding") == "chunked" {
        resp.Body = Some(parse_chunked_body(&mut r).await?);
    } else {
        let ln: usize = resp
            .Header
            .Get("Content-Length")
            .parse::<usize>()
            .expect("Content-Length is not exist");
        let mut buf = vec![0; ln];
        r.read_exact(&mut buf).await?;
        resp.Body = Some(BytesMut::from(&buf[..]));
    }

    resp.ContentLength = resp.Body.as_ref().map_or(0, |b| b.len() as i64);
    Ok(resp)
}

#[cfg(feature = "async-std-runtime")]
async fn parse_chunked_body<R>(mut r: R) -> HttpResult<BytesMut>
where
    R: BufReadExt + Unpin,
{
    let mut body = BytesMut::new();
    let mut size_buf = vec![];
    while r.read_until(b'\n', &mut size_buf).await.is_ok() {
        if size_buf.ends_with(b"\r\n") {
            size_buf.truncate(size_buf.len() - 2); // Remove "\r\n"
            let size_str = std::str::from_utf8(&size_buf)?;
            if size_str == "0" {
                break;
            }
            let chunk_size = usize::from_str_radix(size_str, 16)?;
            let mut chunk_data = vec![0u8; chunk_size];
            r.read_exact(&mut chunk_data).await?;
            body.extend_from_slice(&chunk_data);
            let mut crlf = [0u8; 2];
            r.read_exact(&mut crlf).await?;
            size_buf.clear();
        }
    }
    Ok(body)
}
pub type MIMEHeader = HashMap<String, Vec<String>>;

fn fix_pragma_cache_control(header: &mut Header) {
    if let Some(hp) = header.0.get("Pragma") {
        if hp.len() > 0 && &hp[0] == "no-cache" && header.0.get("Cache-Control").is_none() {
            header.Set("Cache-Control", "no-cache");
        }
    }
}

fn parse_header(head_part: &[u8]) -> HttpResult<MIMEHeader> {
    let mut m: MIMEHeader = HashMap::new();
    let lines = std::str::from_utf8(head_part)?;

    for kv in lines.split("\r\n") {
        if let Some((key, value)) = kv.split_once(':') {
            let key = canonical_mime_header_key(key);
            if key.is_empty() {
                continue;
            }

            let value = value
                .trim_start_matches(|c: char| c == ' ' || c == '\t')
                .trim_matches('"')
                .to_string();

            m.entry(key).or_insert_with(Vec::new).push(value);
        }
    }
    Ok(m)
}

fn start_index_of_body(response: &Vec<u8>) -> Option<usize> {
    let mut sep: Vec<u8> = vec![];
    for (i, b) in response.iter().map(|&x| x).enumerate() {
        if b == b'\r' || b == b'\n' {
            sep.push(b);
        } else {
            sep.clear();
        }
        if sep.as_slice() == b"\r\n\r\n" {
            return Some(i);
        }
    }
    None
}

const TO_LOWER: u8 = b'a' - b'A';

fn canonical_mime_header_key(s: &str) -> String {
    let mut upper = true;
    let mut new = String::with_capacity(s.len());
    for &byte in s.as_bytes() {
        let c = if upper && byte >= b'a' && byte <= b'z' {
            byte - TO_LOWER
        } else if !upper && byte >= b'A' && byte <= b'Z' {
            byte + TO_LOWER
        } else {
            byte
        };
        upper = c == b'-';
        new.push(c as char);
    }
    new
}

pub fn parse_http_version(vers: &str) -> (isize, isize, bool) {
    let big: isize = 1_000_000;

    if !vers.starts_with("HTTP/") {
        return (0, 0, false);
    }

    let version_part = &vers[5..];
    let parts: Vec<&str> = version_part.split('.').collect();

    if parts.len() != 2 {
        return (0, 0, false);
    }

    match (parts[0].parse::<isize>(), parts[1].parse::<isize>()) {
        (Ok(major), Ok(minor)) if major >= 0 && major <= big && minor >= 0 && minor <= big => {
            (major, minor, true)
        }
        _ => (0, 0, false),
    }
}
