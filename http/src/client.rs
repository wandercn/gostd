//! Package http provides HTTP client and server implementations.
//!
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! http包提供了HTTP客户端和服务端的实现。
//! </details>

// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use error::{HTTPConnectError, HttpResult};
use gostd_builtin::*;
use gostd_io::*;
use gostd_strings as strings;
use gostd_time as time;
use gostd_url as url;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
/// DefaultMaxHeaderBytes is the maximum permitted size of the headers in an HTTP request. This can be overridden by setting Server.MaxHeaderBytes.

const DefaultMaxHeaderBytes: int32 = 1 << 20;
/// DefaultMaxIdleConnsPerHost is the default value of Transport's MaxIdleConnsPerHost.
const DefaultMaxIdleConnsPerHost: int32 = 2;

/// TimeFormat is the time format to use when generating times in HTTP headers. It is like time.RFC1123 but hard-codes GMT as the time zone. The time being formatted must be in UTC for Format to generate the correct format.
///
/// For parsing this time format, see ParseTime.
const TimeFormat: &str = "Mon, 02 Jan 2006 15:04:05 GMT";

/// TrailerPrefix is a magic prefix for ResponseWriter.Header map keys that, if present, signals that the map entry is actually for the response trailers, and not the response headers. The prefix is stripped after the ServeHTTP call finishes and the values are sent in the trailers.
///
/// This mechanism is intended only for trailers that are not known prior to the headers being written. If the set of trailers is fixed or known before the header is written, the normal Go trailers mechanism is preferred:
const TrailerPrefix: &str = "Trailer:";

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
/// use gostd_http as http;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
///     let response = http::Get(url)?;///
///     println!(
///         "{}",
///         String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
///     );
///     Ok(())
/// }
/// ```
pub fn Get(url: &str) -> HttpResult<Response> {
    Client::New().Get(url)
}

pub fn Head(url: &str) -> HttpResult<Response> {
    Client::New().Head(url)
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
/// use gostd_http as http;
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url = "https://petstore.swagger.io/v2/pet";
///     let postbody = r#"{"id":0,"category":{"id":0,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":0,"name":"string"}],"status":"available"}"#
///    .as_bytes()
///    .to_vec();
///    let response = http::Post(url, "application/json", Some(postbody.into()))?;
///
///    println!(
///        "{}",
///        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
///    );
///
///    Ok(())
/// }
///
/// ```
pub fn Post(url: &str, contentType: &str, body: Option<Bytes>) -> HttpResult<Response> {
    Client::New().Post(url, contentType, body)
}

pub fn PostForm(url: &str, data: url::Values) -> HttpResult<Response> {
    Client::New().PostForm(url, data)
}

pub fn Patch(url: &str, body: Option<Bytes>) -> HttpResult<Response> {
    Client::New().Patch(url, body)
}

pub fn Put(url: &str, body: Option<Bytes>) -> HttpResult<Response> {
    Client::New().Put(url, body)
}

pub fn Delete(url: &str) -> HttpResult<Response> {
    Client::New().Delete(url)
}

pub struct Client {
    Transport: Box<dyn RoundTripper>,
    // CheckRedirect: fn(req: &Request, via: Vec<&Request>) -> Result<(), Error>,
    Jar: Box<dyn CookieJar>,
    Timeout: time::Duration,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            Transport: Box::new(Transport::default()),
            Timeout: time::Duration::new(0),
            Jar: Box::new(Cookie::default()),
        }
    }
}

impl Client {
    pub fn New() -> Self {
        Self::default()
    }

    pub fn Get(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Get, url, None)?;
        self.Do(&mut req)
    }

    pub fn Post(
        &mut self,
        url: &str,
        contentType: &str,
        body: Option<Bytes>,
    ) -> HttpResult<Response> {
        let mut req = Request::New(Method::Post, url, body)?;
        req.Header.Set("Content-Type", contentType);
        self.Do(&mut req)
    }

    pub fn PostForm(&mut self, url: &str, data: url::Values) -> HttpResult<Response> {
        self.Post(
            url,
            "application/x-www-form-urlencoded",
            Some(data.Encode().into_bytes().into()),
        )
    }

    pub fn Head(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Head, url, None)?;
        self.Do(&mut req)
    }

    pub fn Patch(&mut self, url: &str, body: Option<Bytes>) -> HttpResult<Response> {
        let mut req = Request::New(Method::Patch, url, body)?;
        self.Do(&mut req)
    }

    pub fn Put(&mut self, url: &str, body: Option<Bytes>) -> HttpResult<Response> {
        let mut req = Request::New(Method::Put, url, body)?;
        self.Do(&mut req)
    }

    pub fn Delete(&mut self, url: &str) -> HttpResult<Response> {
        let mut req = Request::New(Method::Delete, url, None)?;
        self.Do(&mut req)
    }

    pub fn Do(&mut self, req: &mut Request) -> HttpResult<Response> {
        self.done(req)
    }

    fn send(
        &mut self,
        req: &mut Request,
        deadline: time::Time,
    ) -> HttpResult<(Response, fn() -> bool)> {
        let (resp, didTimeout) = send(req, self.transport(), deadline)?;
        Ok((resp, didTimeout))
    }

    fn done(&mut self, req: &mut Request) -> HttpResult<Response> {
        let deadline = self.deadline();
        let (resp, didTimeout) = self.send(req, deadline)?;
        Ok(resp)
    }

    fn deadline(&mut self) -> time::Time {
        if self.Timeout > time::Duration::new(0) {
            return time::Now().Add(&self.Timeout);
        }
        time::Time::default()
    }

    fn transport(&self) -> Box<dyn RoundTripper> {
        Box::new(Transport::default())
    }
}

fn send(
    ireq: &mut Request,
    mut rt: Box<dyn RoundTripper>,
    deadline: time::Time,
) -> HttpResult<(Response, fn() -> bool)> {
    let mut resp = Response::default();
    fn didTimeout() -> bool {
        return false;
    };
    loop {
        let mut resp = rt.RoundTrip(ireq)?;
        let mut loc = resp.Header.Get("Location");
        let (redirectMethod, shouldRedirect, includeBody) =
            redirectBehavior(ireq.Method.as_str(), &resp, ireq);
        if !shouldRedirect {
            return Ok((resp, didTimeout));
        }
        let mut u = ireq.URL.Parse(loc.as_str())?;
        let urlRef = refererForURL(&ireq.URL, &u);
        ireq.Method = redirectMethod.clone();
        ireq.URL = u.clone();
        ireq.Header.Set("Referer", urlRef.as_str());
    }
}

fn redirectBehavior(reqMethod: &str, resp: &Response, ireq: &Request) -> (String, bool, bool) {
    let mut shouldRedirect = false;
    let mut includeBody = false;
    match resp.StatusCode {
        301 | 302 | 303 => return (Method::Get.String().to_string(), true, false),
        307 | 308 => {
            if resp.Header.Get("Location") == "" {
                shouldRedirect = true;
                includeBody = false;
            }
            if ireq.Body.is_none() && ireq.ContentLength != 0 {
                shouldRedirect = false;
            }
        }
        _ => (),
    }
    (reqMethod.to_string(), shouldRedirect, includeBody)
}

pub trait RoundTripper {
    fn RoundTrip(&mut self, r: &Request) -> HttpResult<Response>;
}

fn refererForURL(lastReq: &url::URL, newReq: &url::URL) -> String {
    if (lastReq.Scheme == "https") && (newReq.Scheme == "http") {
        return "".to_string();
    }
    let mut referer = lastReq.String();
    if let Some(user) = lastReq.User.clone() {
        return referer;
    }
    let auth = "@";
    referer = strings::Replace(referer.as_str(), auth, "", 1);
    referer
}

use std::iter::FromIterator;
use std::sync;
#[derive(Default, Clone)]
struct Transport {
    // idleMu: sync::Mutex,
    closeIdle: bool,
    // idleConn:HashMap<String, Vec<>>
    Proxy: Option<url::URL>,
    // Dial: fn(network: &str, addr: &str) -> Result<net::TcpConn, Error>,
    ForceAttemptHTTP2: bool,
    MaxIdleConns: int,
    // IdleConnTimeout:       90 * time.Second,
    // TLSHandshakeTimeout:   10 * time.Second,
    // ExpectContinueTimeout: 1 * time.Second,
    DisableKeepAlives: bool,

    DisableCompression: bool,
    iMaxIdleConnsPerHost: int,
    MaxConnsPerHost: int,
    MaxResponseHeaderBytes: int64,
    WriteBufferSize: int,
    ReadBufferSize: int,
    tlsNextProtoWasNil: bool,
}

use std::net;
use std::sync::mpsc;
impl RoundTripper for Transport {
    fn RoundTrip(&mut self, req: &Request) -> HttpResult<Response> {
        self.roundTrip(req)
    }
}
impl Transport {
    fn roundTrip(&mut self, req: &Request) -> HttpResult<Response> {
        let treq = &mut transportRequest {
            Req: req.clone(),
            extra: None,
        };
        let cm = self.connectMethodForRequest(treq)?;
        let (mut pconn, mut conn) = self.getConn(treq, cm)?;
        // conn.set_write_timeout(Some(std::time::Duration::new(5, 0)));
        // conn.set_read_timeout(Some(std::time::Duration::new(5, 0)));

        pconn.roundTrip(treq, conn)
    }

    fn getConn(
        &mut self,
        treq: &transportRequest,
        cm: connectMethod,
    ) -> HttpResult<(persistConn, TcpConn)> {
        let conn = self.dialConn(cm)?;
        let pconn = persistConn::default();
        Ok((pconn, conn))
    }

    fn dialConn(&mut self, cm: connectMethod) -> HttpResult<TcpConn> {
        // pconn.t = self;
        // pconn.reqch = mpsc::channel();
        // pconn.writech = mpsc::channel();
        // pconn.writeLoopDone = mpsc::channel();
        self.dial("tcp", cm.addr().as_str())
        // pconn.conn = conn;
        // pconn.br = bufio::NewReaderSize(pconn, self.readBufferSize());
        // pconn.bw = bufio::NewWriterSize(persistConnWriter { pconn }, self.writeBufferSize());
        // 待实现读写进程
        /* go pconn.readLoop()
        go pconn.writeLoop() */
        // Ok(pconn)
    }

    fn dial(&mut self, network: &str, addr: &str) -> HttpResult<TcpConn> {
        Ok(net::TcpStream::connect(addr)?)
    }

    fn connectMethodForRequest(&mut self, treq: &transportRequest) -> HttpResult<connectMethod> {
        let mut cm = connectMethod::default();
        cm.targetScheme = treq.Req.URL.Scheme.clone();
        cm.targetAddr = canonicalAddr(&treq.Req.URL.clone());
        cm.proxyURL = None;
        cm.onlyH1 = true; //待优化
        Ok(cm)
    }

    fn wirteBufferSize(self) -> int {
        if self.WriteBufferSize > 0 {
            return self.WriteBufferSize;
        }
        4 << 10
    }

    fn readBufferSize(self) -> int {
        if self.ReadBufferSize > 0 {
            return self.ReadBufferSize;
        }
        4 << 10
    }
}

fn canonicalAddr(url: &url::URL) -> String {
    let portMap: HashMap<String, String> = [
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
        port = portMap.get(url.Scheme.as_str()).unwrap().to_string();
    }
    strings::Join(vec![addr.as_str(), port.as_str()], ":")
}

#[derive(Default, Clone)]
struct transportRequest {
    pub Req: Request,
    extra: Option<Header>,
}

impl transportRequest {
    fn extraHeaders(&mut self) -> Header {
        if let Some(extra) = self.extra.clone() {
            return extra;
        }
        Header::default()
    }
}

#[derive(Default, PartialEq, PartialOrd, Clone)]
struct connectMethod {
    proxyURL: Option<url::URL>, // nil for no proxy, else full proxy URL
    targetScheme: String,       // "http" or "https"
    // If proxyURL specifies an http or https proxy, and targetScheme is http (not https),
    // then targetAddr is not included in the connect method key, because the socket can
    // be reused for different targetAddr values.
    targetAddr: String,
    onlyH1: bool, // whether to disable HTTP/2 and force HTTP/1
}
impl connectMethod {
    fn scheme(&self) -> String {
        self.targetScheme.clone()
    }

    fn addr(&self) -> String {
        self.targetAddr.clone()
    }
}
type TcpConn = TcpStream;
use std::sync::mpsc::channel;
#[derive(Default, Clone)]
struct persistConn {
    t: Transport,
    // br: bufio.Reader,
    // bw: bufio.Writer,
    nwrite: int64,
    // reqch: channel,
    // writech: channel,
    isProxy: bool,
    sawEOF: bool,
    readLimit: int64,
    // writeErrch: channel,
    // writeLoopDone: channel,
    numExpectedResponses: int,
    broken: bool,
    reused: bool,
}

use bytes::Bytes;
use rustls::ClientConnection;
use rustls::StreamOwned;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::Shutdown;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;
use webpki_roots::TLS_SERVER_ROOTS;
impl persistConn {
    fn roundTrip(&mut self, req: &mut transportRequest, mut conn: TcpConn) -> HttpResult<Response> {
        self.numExpectedResponses += 1;
        let mut requestedGzip = false;
        if !self.t.DisableCompression
            && req.Req.Header.Get("Accept-Encoding") == ""
            && req.Req.Header.Get("Range") == ""
            && req.Req.Method != "HEAD".to_string()
        {
            requestedGzip = true;
            // req.extra = Some(req.Req.Header.clone());
            // let mut hd = req.extra.take().unwrap();
            // // hd.Set("Accept-Encoding", "gzip");

            // req.extra = Some(hd.clone());
            // req.Req.Header = hd;
        }
        if req.Req.Close {
            req.Req.Header.Set("Connection", "close");
        }

        let r = req.Req.Write()?;

        if req.Req.isTLS {
            let mut tlsConn = getTLSConn(req.Req.Host.as_str(), conn)?;
            tlsConn.write(r.as_slice())?;
            let mut reader = BufReader::new(tlsConn);
            let resp = ReadResponse(reader, &req.Req)?;
            Ok(resp)
        } else {
            conn.write(r.as_slice())?;
            let mut reader = BufReader::new(conn);
            let resp = ReadResponse(reader, &req.Req)?;
            Ok(resp)
        }
    }
}
use bytes::{Buf, BytesMut};
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, RootCertStore};
use std::io::ErrorKind;

use crate::cookies::Cookie;
use crate::cookies::CookieJar;
use crate::error;
use crate::header::Header;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;

fn get_tls_config() -> Arc<ClientConfig> {
    let mut clientRootCert = RootCertStore::from_iter(TLS_SERVER_ROOTS.iter().cloned());

    Arc::new(
        ClientConfig::builder()
            .with_root_certificates(clientRootCert)
            .with_no_client_auth(),
    )
}

fn getTLSConn(
    dnsName: &str,
    socket: TcpConn,
) -> HttpResult<StreamOwned<ClientConnection, TcpConn>> {
    let tlsconfig = get_tls_config();
    let serverName = ServerName::try_from(dnsName.to_owned())?;
    let mut tlsClient = ClientConnection::new(tlsconfig, serverName)?;
    let mut tlsConn = StreamOwned::new(tlsClient, socket);
    Ok(tlsConn)
}

pub fn ReadResponse(mut r: impl BufRead, req: &Request) -> HttpResult<Response> {
    let mut resp = Response::default();
    resp.Request = req.clone();
    // parse status line。
    let mut line = String::new();
    r.read_line(&mut line)?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP response".to_string(),
        ));
    }
    resp.Proto = parts[0].to_string();
    resp.Status = parts[1..].join(" ");
    resp.StatusCode = parts[1].parse::<isize>()?;
    let vers = ParseHTTPVersion(&resp.Proto);
    let ok = vers.2;
    if !ok {
        return Err(HTTPConnectError::ConnectionFailure(
            "malformed HTTP version".to_string(),
        ));
    }
    resp.ProtoMajor = vers.0;
    resp.ProtoMinor = vers.1;

    // 1. 获取response的header部分，到第一个 '\r\b'独立行为header的结束。
    let mut headPart = BytesMut::new();
    let mut head_line = String::new();
    while r.read_line(&mut head_line).is_ok() {
        if head_line.as_bytes() == b"\r\n" {
            break;
        }
        headPart.extend_from_slice(head_line.as_bytes());
        head_line.clear()
    }

    // parse headPart
    resp.Header = Header::NewWithHashMap(parseHeader(&headPart)?);
    fixPragmaCacheControl(&mut resp.Header);

    // set Body
    if resp.Header.Get("Transfer-Encoding").as_str() == "chunked" {
        // 2.chunked方式传输方式。获取body数据。
        resp.Body = Some(parseChunkedBody(r)?);
    } else {
        // 3. 除chunked外的其他传输方式，都有Content-Length字段,根据长度获取body
        let ln: usize = resp
            .Header
            .Get("Content-Length")
            .as_str()
            .parse::<usize>()
            .expect("Content-Length is not exist");

        let mut buf = vec![0; ln]; // 生成固定长度的数组，用于读取定长数据;
        r.read_exact(&mut buf)?;
        resp.Body = Some(BytesMut::from(&buf[..]));
    }
    resp.ContentLength = resp.Body.as_ref().map_or(0, |b| b.len() as i64);
    Ok(resp)
}

// chunk数据是以16位数据长度 7acc\r\n独立行开头+ [data] 下一行以\r\n结尾数据段形式，所以数据的结尾用0\r\n表示。
fn parseChunkedBody(mut r: impl BufRead) -> HttpResult<BytesMut> {
    let mut body = BytesMut::new();
    let mut size_buf = vec![];
    while r.read_until(b'\n', &mut size_buf).is_ok() {
        // 校验开头行是\r\n结尾的chuank size行
        if size_buf.ends_with(b"\r\n") {
            // 删除尾部的\r\n,只保留表示大小的字符串
            size_buf.truncate(size_buf.len() - 2); // Remove "\r\n"

            // 16进制chunk大小字符串
            let size_str = std::str::from_utf8(&size_buf)?;

            // 如果字符串等于"0"，已经到最后一个chunk数据段。
            if size_str == "0" {
                break;
            }

            // 按chunk长度读取分段的实际数据
            let chunk_size = usize::from_str_radix(size_str, 16)?;

            let mut chunk_data = vec![0u8; chunk_size];
            r.read_exact(&mut chunk_data)?;
            body.extend_from_slice(&chunk_data);
            //读取每个chunk data 结尾的\r\n，并丢弃掉
            let mut crlf = [0u8; 2];
            r.read_exact(&mut crlf)?;
            //chuank size 行的数据要清空
            size_buf.clear();
        }
    }
    Ok(body)
}

pub type MIMEHeader = HashMap<String, Vec<String>>;

fn fixPragmaCacheControl(header: &mut Header) {
    if let Some(hp) = header.0.get("Pragma") {
        if len!(hp) > 0 && &hp[0] == "no-cache" && header.0.get("Cache-Control").is_none() {
            header.Set("Cache-Control", "no-cache");
        }
    }
}

fn parseHeader(headPart: &[u8]) -> HttpResult<MIMEHeader> {
    let mut m: MIMEHeader = HashMap::new();
    let lines = std::str::from_utf8(headPart)?;

    for kv in lines.split("\r\n") {
        if let Some((key, value)) = kv.split_once(':') {
            let key = canonicalMIMEHeaderKey(key);
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

fn startIndexOfBody(response: &Vec<u8>) -> Option<usize> {
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

// Header KE规范化 content-length|CONTENT-LENGTH => Content-Length
const toLower: byte = (b'a' - b'A');
fn canonicalMIMEHeaderKey(s: &str) -> String {
    let mut upper = true;
    let mut new = String::with_capacity(s.len());
    for &byte in s.as_bytes() {
        let c = if upper && byte >= b'a' && byte <= b'z' {
            byte - toLower
        } else if !upper && byte >= b'A' && byte <= b'Z' {
            byte + toLower
        } else {
            byte
        };
        upper = c == b'-';
        new.push(c as char);
    }
    new
}

pub fn ParseHTTPVersion(vers: &str) -> (int, int, bool) {
    let big: int = 1_000_000;

    if !vers.starts_with("HTTP/") {
        return (0, 0, false);
    }

    let version_part = &vers[5..];
    let parts: Vec<&str> = version_part.split('.').collect();

    if parts.len() != 2 {
        return (0, 0, false);
    }

    match (parts[0].parse::<int>(), parts[1].parse::<int>()) {
        (Ok(major), Ok(minor)) if major >= 0 && major <= big && minor >= 0 && minor <= big => {
            (major, minor, true)
        }
        _ => (0, 0, false),
    }
}
