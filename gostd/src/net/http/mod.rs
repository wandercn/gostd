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

/// Common HTTP methods.
#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Patch, // RFC 5789
    Delete,
    Connect,
    Options,
    Trace,
}

impl Method {
    pub fn String(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTHONS",
            Method::Trace => "TRACE",
        }
    }
}

/// HTTP status codes as registered with IANA. See: <https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml>
pub enum Status {
    Continue = 100,                      // RFC 7231, 6.2.1
    SwitchingProtocols = 101,            // RFC 7231, 6.2.2
    Processing = 102,                    // RFC 2518, 10.1
    EarlyHints = 103,                    // RFC 8297
    OK = 200,                            // RFC 7231, 6.3.1
    Created = 201,                       // RFC 7231, 6.3.2
    Accepted = 202,                      // RFC 7231, 6.3.3
    NonAuthoritativeInfo = 203,          // RFC 7231, 6.3.4
    NoContent = 204,                     // RFC 7231, 6.3.5
    ResetContent = 205,                  // RFC 7231, 6.3.6
    PartialContent = 206,                // RFC 7233, 4.1
    MultiStatus = 207,                   // RFC 4918, 11.1
    AlreadyReported = 208,               // RFC 5842, 7.1
    IMUsed = 226,                        // RFC 3229, 10.4.1
    MultipleChoices = 300,               // RFC 7231, 6.4.1
    MovedPermanently = 301,              // RFC 7231, 6.4.2
    Found = 302,                         // RFC 7231, 6.4.3
    SeeOther = 303,                      // RFC 7231, 6.4.4
    NotModified = 304,                   // RFC 7232, 4.1
    UseProxy = 305,                      // RFC 7231, 6.4.5
    TemporaryRedirect = 307,             // RFC 7231, 6.4.7
    PermanentRedirect = 308,             // RFC 7538, 3
    BadRequest = 400,                    // RFC 7231, 6.5.1
    Unauthorized = 401,                  // RFC 7235, 3.1
    PaymentRequired = 402,               // RFC 7231, 6.5.2
    Forbidden = 403,                     // RFC 7231, 6.5.3
    NotFound = 404,                      // RFC 7231, 6.5.4
    MethodNotAllowed = 405,              // RFC 7231, 6.5.5
    NotAcceptable = 406,                 // RFC 7231, 6.5.6
    ProxyAuthRequired = 407,             // RFC 7235, 3.2
    RequestTimeout = 408,                // RFC 7231, 6.5.7
    Conflict = 409,                      // RFC 7231, 6.5.8
    Gone = 410,                          // RFC 7231, 6.5.9
    LengthRequired = 411,                // RFC 7231, 6.5.10
    PreconditionFailed = 412,            // RFC 7232, 4.2
    RequestEntityTooLarge = 413,         // RFC 7231, 6.5.11
    RequestURITooLong = 414,             // RFC 7231, 6.5.12
    UnsupportedMediaType = 415,          // RFC 7231, 6.5.13
    RequestedRangeNotSatisfiable = 416,  // RFC 7233, 4.4
    ExpectationFailed = 417,             // RFC 7231, 6.5.14
    Teapot = 418,                        // RFC 7168, 2.3.3
    MisdirectedRequest = 421,            // RFC 7540, 9.1.2
    UnprocessableEntity = 422,           // RFC 4918, 11.2
    Locked = 423,                        // RFC 4918, 11.3
    FailedDependency = 424,              // RFC 4918, 11.4
    TooEarly = 425,                      // RFC 8470, 5.2.
    UpgradeRequired = 426,               // RFC 7231, 6.5.15
    PreconditionRequired = 428,          // RFC 6585, 3
    TooManyRequests = 429,               // RFC 6585, 4
    RequestHeaderFieldsTooLarge = 431,   // RFC 6585, 5
    UnavailableForLegalReasons = 451,    // RFC 7725, 3
    InternalServerError = 500,           //RFC 7231, 6.6.1
    NotImplemented = 501,                // RFC 7231, 6.6.2
    BadGateway = 502,                    // RFC 7231, 6.6.3
    ServiceUnavailable = 503,            // RFC 7231, 6.6.4
    GatewayTimeout = 504,                // RFC 7231, 6.6.5
    HTTPVersionNotSupported = 505,       // RFC 7231, 6.6.6
    VariantAlsoNegotiates = 506,         // RFC 2295, 8.1
    InsufficientStorage = 507,           // RFC 4918, 11.5
    LoopDetected = 508,                  // RFC 5842, 7.2
    NotExtended = 510,                   // RFC 2774, 7
    NetworkAuthenticationRequired = 511, // RFC 6585, 6
}

impl Status {
    pub fn StatusText(&self) -> &str {
        match self {
            StatusContinue => "Continue",
            StatusSwitchingProtocols => "Switching Protocols",
            StatusProcessing => "Processing",
            StatusEarlyHints => "Early Hints",

            StatusOK => "OK",
            StatusCreated => "Created",
            StatusAccepted => "Accepted",
            StatusNonAuthoritativeInfo => "Non-Authoritative Information",
            StatusNoContent => "No Content",
            StatusResetContent => "Reset Content",
            StatusPartialContent => "Partial Content",
            StatusMultiStatus => "Multi-Status",
            StatusAlreadyReported => "Already Reported",
            StatusIMUsed => "IM Used",

            StatusMultipleChoices => "Multiple Choices",
            StatusMovedPermanently => "Moved Permanently",
            StatusFound => "Found",
            StatusSeeOther => "See Other",
            StatusNotModified => "Not Modified",
            StatusUseProxy => "Use Proxy",
            StatusTemporaryRedirect => "Temporary Redirect",
            StatusPermanentRedirect => "Permanent Redirect",

            StatusBadRequest => "Bad Request",
            StatusUnauthorized => "Unauthorized",
            StatusPaymentRequired => "Payment Required",
            StatusForbidden => "Forbidden",
            StatusNotFound => "Not Found",
            StatusMethodNotAllowed => "Method Not Allowed",
            StatusNotAcceptable => "Not Acceptable",
            StatusProxyAuthRequired => "Proxy Authentication Required",
            StatusRequestTimeout => "Request Timeout",
            StatusConflict => "Conflict",
            StatusGone => "Gone",
            StatusLengthRequired => "Length Required",
            StatusPreconditionFailed => "Precondition Failed",
            StatusRequestEntityTooLarge => "Request Entity Too Large",
            StatusRequestURITooLong => "Request URI Too Long",
            StatusUnsupportedMediaType => "Unsupported Media Type",
            StatusRequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            StatusExpectationFailed => "Expectation Failed",
            StatusTeapot => "I'm a teapot",
            StatusMisdirectedRequest => "Misdirected Request",
            StatusUnprocessableEntity => "Unprocessable Entity",
            StatusLocked => "Locked",
            StatusFailedDependency => "Failed Dependency",
            StatusTooEarly => "Too Early",
            StatusUpgradeRequired => "Upgrade Required",
            StatusPreconditionRequired => "Precondition Required",
            StatusTooManyRequests => "Too Many Requests",
            StatusRequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusUnavailableForLegalReasons => "Unavailable For Legal Reasons",

            StatusInternalServerError => "Internal Server Error",
            StatusNotImplemented => "Not Implemented",
            StatusBadGateway => "Bad Gateway",
            StatusServiceUnavailable => "Service Unavailable",
            StatusGatewayTimeout => "Gateway Timeout",
            StatusHTTPVersionNotSupported => "HTTP Version Not Supported",
            StatusVariantAlsoNegotiates => "Variant Also Negotiates",
            StatusInsufficientStorage => "Insufficient Storage",
            StatusLoopDetected => "Loop Detected",
            StatusNotExtended => "Not Extended",
            StatusNetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}

/// DefaultMaxHeaderBytes is the maximum permitted size of the headers in an HTTP request. This can be overridden by setting Server.MaxHeaderBytes.
use crate::builtin::*;
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

use crate::io::*;
use crate::net::url;
use crate::strings;
use crate::time;
use std::collections::HashMap;
use std::io::Error;

pub fn Get(url: &str) -> HttpResult {
    Client::New().Get(url)
}

pub fn Head(url: &str) -> HttpResult {
    Client::New().Head(url)
}

pub fn Post(url: &str, contentType: &str, body: Option<Vec<u8>>) -> HttpResult {
    Client::New().Post(url, contentType, body)
}

pub fn PostForm(url: &str, data: url::Values) -> HttpResult {
    Client::New().PostForm(url, data)
}

pub fn Patch(url: &str, body: Option<Vec<u8>>) -> HttpResult {
    Client::New().Patch(url, body)
}

pub fn Put(url: &str, body: Option<Vec<u8>>) -> HttpResult {
    Client::New().Put(url, body)
}

pub fn Delete(url: &str) -> HttpResult {
    Client::New().Delete(url)
}

pub struct Client {
    Transport: Box<dyn RoundTripper>,
    // CheckRedirect: fn(req: &Request, via: Vec<&Request>) -> Result<(), Error>,
    Jar: Box<dyn CookieJar>,
    Timeout: time::Duration,
}

pub type HttpResult = Result<Response, Error>;
impl Client {
    pub fn New() -> Client {
        Client {
            Transport: Box::new(Transport::default()),
            Timeout: time::Duration::new(0),
            Jar: Box::new(Cookie::default()),
        }
    }

    pub fn Get(&mut self, url: &str) -> HttpResult {
        let mut req = Request::New(Method::Get, url, None)?;
        self.Do(&mut req)
    }

    pub fn Post(&mut self, url: &str, contentType: &str, body: Option<Vec<u8>>) -> HttpResult {
        let mut req = Request::New(Method::Post, url, body)?;
        req.Header.Set("Content-Type", contentType);
        self.Do(&mut req)
    }

    pub fn PostForm(&mut self, url: &str, data: url::Values) -> HttpResult {
        self.Post(
            url,
            "application/x-www-form-urlencoded",
            Some(data.Encode().as_bytes().to_vec()),
        )
    }

    pub fn Head(&mut self, url: &str) -> HttpResult {
        let mut req = Request::New(Method::Head, url, None)?;
        self.Do(&mut req)
    }

    pub fn Patch(&mut self, url: &str, body: Option<Vec<u8>>) -> HttpResult {
        let mut req = Request::New(Method::Patch, url, body)?;
        self.Do(&mut req)
    }

    pub fn Put(&mut self, url: &str, body: Option<Vec<u8>>) -> HttpResult {
        let mut req = Request::New(Method::Put, url, body)?;
        self.Do(&mut req)
    }

    pub fn Delete(&mut self, url: &str) -> HttpResult {
        let mut req = Request::New(Method::Delete, url, None)?;
        self.Do(&mut req)
    }

    pub fn Do(&mut self, req: &mut Request) -> HttpResult {
        self.done(req)
    }

    fn send(
        &mut self,
        req: &mut Request,
        deadline: time::Time,
    ) -> Result<(Response, fn() -> bool), Error> {
        let (resp, didTimeout) = send(req, self.transport(), deadline)?;
        Ok((resp, didTimeout))
    }

    fn done(&mut self, req: &mut Request) -> HttpResult {
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
) -> Result<(Response, fn() -> bool), Error> {
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
        let mut host = "".to_string();
        let mut u = url::Parse(loc.as_str())?;
        if u.Scheme == "".to_string() {
            host = ireq.Host.clone();
        }
        ireq.Method = redirectMethod.clone();
        ireq.URL = u.clone();
        ireq.Host = host;
        let urlRef = refererForURL(&ireq.URL, &u);
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
    fn RoundTrip(&mut self, r: &Request) -> Result<Response, Error>;
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

#[derive(Default, Clone, Debug)]
pub struct Request {
    Method: String,
    URL: url::URL,
    Proto: String,
    ProtoMajor: int,
    ProtoMinor: int,
    pub Header: Header,
    pub Body: Option<Vec<u8>>,
    // GetBody func() (io.ReadCloser, error)
    ContentLength: int64,
    TransferEncoding: Vec<String>,
    Close: bool,
    Host: String,
    Form: url::Values,
    PostForm: url::Values,
    // MultipartForm:*multipart.Form,
    Trailer: Header,
    RemoteAddr: String,
    RequestURI: String,
    isTLS: bool,
    // TLS *tls.ConnectionState,
    // Cancel <-chan struct{}
    // ctx context.Context
}

impl Request {
    pub fn New(method: Method, url: &str, body: Option<Vec<u8>>) -> Result<Request, Error> {
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
            Close: true,
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

    pub fn Write(&self) -> Result<Vec<u8>, Error> {
        self.write(false)
    }

    fn write(&self, usingProxy: bool) -> Result<Vec<u8>, Error> {
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
#[derive(Default, Debug, Clone)]
pub struct Response {
    pub Status: String,
    pub StatusCode: int,
    pub Proto: String,
    pub ProtoMajor: int,
    pub ProtoMinor: int,
    pub Header: Header,
    pub ContentLength: int64,
    pub TransferEncoding: Vec<String>,
    pub Body: Option<Vec<u8>>,
    pub Close: bool,
    pub Uncompressed: bool,
    pub Trailer: Header,
    pub Request: Request,
}

impl Response {
    pub fn Cookies(&self) -> Vec<Cookie> {
        readSetCookies(&self.Header)
    }
}
fn isCookieNameValid(raw: &str) -> bool {
    if raw == "" {
        return false;
    }
    strings::IndexFunc(raw, isNotToken) < 0
}

fn isNotToken(r: rune) -> bool {
    !validHeaderFieldByte(r as u8)
}

fn validCookieValueByte(b: byte) -> bool {
    return 0x20 <= b && b < 0x7f && b != b'"' && b != b';' && b != b'\\';
}

fn parseCookieValue(mut raw: &str, allowDoubleQuote: bool) -> (string, bool) {
    // Strip the quotes, if present.
    if allowDoubleQuote
        && len!(raw) > 1
        && raw.bytes().nth(0) == Some(b'"')
        && raw.bytes().nth((len!(raw) - 1)) == Some(b'"')
    {
        raw = &raw[1..len!(raw) - 1]
    }
    for i in 0..len!(raw) {
        if !validCookieValueByte(raw.as_bytes()[i as usize]) {
            return ("".to_string(), false);
        }
    }
    return (raw.to_string(), true);
}
fn readSetCookies(h: &Header) -> Vec<Cookie> {
    let cookieCount = len!(h.0.get(&"Set-Cookie".to_string()).unwrap());
    if cookieCount == 0 {
        return vec![];
    }
    let mut cookies = Vec::with_capacity(cookieCount);
    for line in h.0.get("Set-Cookie").unwrap() {
        let mut parts = strings::Split(strings::TrimSpace(line.as_str()), ";");
        if len!(parts) == 1 && parts[0] == "" {
            continue;
        }
        parts[0] = strings::TrimSpace(parts[0]);

        let j = strings::Index(parts[0], "=");
        if j < 0 {
            continue;
        }
        let mut name = &parts[0][..j as usize];
        let mut value = &parts[0][j as usize + 1..];
        if !isCookieNameValid(name) {
            continue;
        }
        let cookie = parseCookieValue(value, true);
        value = &cookie.0;
        let ok = &cookie.1;
        if !ok {
            continue;
        }
        let mut c = Cookie::default();
        c.Name = name.to_string();
        c.Value = value.to_string();
        c.Raw = line.to_string();

        for i in 1..len!(parts) {
            parts[i] = strings::TrimSpace(parts[i]);
            if len!(parts[i]) == 0 {
                continue;
            }
            let mut attr = parts[i];
            let mut val = "";
            let j = strings::Index(attr, "=");
            if j >= 0 {
                attr = &attr[..j as usize];
                val = &attr[j as usize + 1..];
            }
            if !attr.is_ascii() {
                continue;
            }

            let cok = parseCookieValue(val, false);
            val = &cok.0;
            let ok = &cok.1;
            if !ok {
                c.Unparsed.push(parts[i].to_string());
                continue;
            }
            let lowerAttr = strings::ToLower(attr);
            match lowerAttr.as_str() {
                "sameste" => {
                    if !val.is_ascii() {
                        c.SameSite = SameSite::SameSiteDefaultMode;
                        continue;
                    }
                    let lowerVal = strings::ToLower(val);
                    match lowerVal.as_str() {
                        "lax" => c.SameSite = SameSite::SameSiteLaxMode,
                        "strict" => c.SameSite = SameSite::SameSiteStrictMode,
                        "none" => c.SameSite = SameSite::SameSiteNoneModepub,
                        _ => c.SameSite = SameSite::SameSiteDefaultMode,
                    }
                    continue;
                }
                "secure" => {
                    c.Secure = true;
                    continue;
                }
                "httponly" => {
                    c.HttpOnly = true;
                    continue;
                }
                "domain" => {
                    c.Domain = val.to_string();
                    continue;
                }
                "max-age" => {
                    let mut secs: int = 0;
                    let res = val.parse::<int>();
                    if res.is_err() || (secs != 0 && val.bytes().nth(0) == Some(b'0')) {
                        break;
                    }
                    secs = res.unwrap();
                    if secs <= 0 {
                        secs = -1;
                    }
                    c.MaxAge = secs;
                    continue;
                }
                "expires" => {
                    c.RawExpires = val.to_string();
                    if let Ok(mut exptime) = time::Parse(time::RFC1123, val) {
                        c.Expires = exptime.UTC();
                    } else {
                        if let Ok(mut exptime) = time::Parse("Mon, 02-Jan-2006 15:04:05 MST", val) {
                            c.Expires = exptime.UTC();
                        } else {
                            c.Expires = time::Time::default();
                            break;
                        }
                    }
                    continue;
                }
                "path" => {
                    c.Path = val.to_string();
                    continue;
                }
                _ => (),
            }
            c.Unparsed.push(parts[i].to_string());
        }
        cookies.push(c);
    }
    cookies
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum SameSite {
    SameSiteDefaultMode,
    SameSiteLaxMode,
    SameSiteStrictMode,
    SameSiteNoneModepub,
}

impl Default for SameSite {
    fn default() -> Self {
        SameSite::SameSiteDefaultMode
    }
}
trait CookieJar {
    fn SetCookies(&mut self, u: &url::URL, cookies: Vec<Cookie>);

    fn Cookies(&self, u: &url::URL) -> Vec<Cookie>;
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Header(HashMap<String, Vec<String>>);

impl Header {
    pub fn NewWithHashMap(m: HashMap<String, Vec<String>>) -> Header {
        Header(m)
    }
    pub fn Add(&mut self, key: &str, value: &str) {
        self.0
            .get_mut(&key.to_string())
            .unwrap()
            .push(value.to_string())
    }

    pub fn Set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), vec![value.to_string()]);
    }

    pub fn Get(&self, key: &str) -> String {
        self.0
            .get(key)
            .unwrap_or(&vec!["".to_string()])
            .get(0)
            .unwrap()
            .to_string()
    }
}

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Cookie {
    Name: String,
    Value: String,
    Path: String,        // optional
    Domain: String,      // optional
    Expires: time::Time, // optional
    RawExpires: String,  // for reading cookies only

    // MaxAge=0 means no 'Max-Age' attribute specified.
    // MaxAge<0 means delete cookie now, equivalently 'Max-Age: 0'
    // MaxAge>0 means Max-Age attribute present and given in seconds
    MaxAge: int,
    Secure: bool,
    HttpOnly: bool,
    SameSite: SameSite,
    Raw: String,
    Unparsed: Vec<String>, // Raw text of unparsed attribute-value pairs
}

impl CookieJar for Cookie {
    fn SetCookies(&mut self, u: &url::URL, cookies: Vec<Cookie>) {
        todo!()
    }

    fn Cookies(&self, u: &url::URL) -> Vec<Cookie> {
        todo!()
    }
}
// SameSite allows a server to define a cookie attribute making it impossible for
// the browser to send this cookie along with cross-site requests. The main
// goal is to mitigate the risk of cross-origin information leakage, and provide
// some protection against cross-site request forgery attacks.
//
// See https://tools.ietf.org/html/draft-ietf-httpbis-cookie-same-site-00 for details.
// type SameSite = int;

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
    fn RoundTrip(&mut self, req: &Request) -> HttpResult {
        self.roundTrip(req)
    }
}
impl Transport {
    fn roundTrip(&mut self, req: &Request) -> HttpResult {
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
    ) -> Result<(persistConn, TcpConn), Error> {
        let conn = self.dialConn(cm)?;
        let pconn = persistConn::default();
        Ok((pconn, conn))
    }

    fn dialConn(&mut self, cm: connectMethod) -> Result<TcpConn, Error> {
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

    fn dial(&mut self, network: &str, addr: &str) -> Result<TcpConn, Error> {
        net::TcpStream::connect(addr)
    }

    fn connectMethodForRequest(&mut self, treq: &transportRequest) -> Result<connectMethod, Error> {
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
    let addr = url.Hostname().clone();
    let mut port = url.Port().clone();
    if port == "" {
        port = portMap.get(url.Scheme.as_str()).unwrap().to_string();
    }
    strings::Join(vec![addr.as_str(), port.as_str()], ":")
}

#[derive(Default, Clone)]
struct transportRequest {
    Req: Request,
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
use crate::net::TcpConn;
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

use rustls::ClientConnection;
use rustls::StreamOwned;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::Shutdown;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;
impl persistConn {
    fn roundTrip(&mut self, req: &mut transportRequest, mut conn: TcpConn) -> HttpResult {
        self.numExpectedResponses += 1;
        let mut requestedGzip = false;
        if !self.t.DisableCompression
            && req.Req.Header.Get("Accept-Encoding") == ""
            && req.Req.Header.Get("Range") == ""
            && req.Req.Method != "HEAD".to_string()
        {
            requestedGzip = true;
            req.extra = Some(req.Req.Header.clone());
            let mut hd = req.extra.take().unwrap();
            // hd.Set("Accept-Encoding", "gzip");
            if req.Req.Close {
                hd.Set("Connection", "close");
            }
            req.extra = Some(hd.clone());
            req.Req.Header = hd;
        }

        let r = req.Req.Write()?;

        if req.Req.isTLS {
            let mut tlsConn = getTLSConn(req.Req.Host.as_str(), conn);
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

use std::io::ErrorKind;

fn getTLSConn(dnsName: &str, socket: TcpConn) -> StreamOwned<ClientConnection, TcpConn> {
    let mut clientRootCert = rustls::RootCertStore::empty();
    clientRootCert.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    let tlsconfig = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(clientRootCert)
        .with_no_client_auth();
    let serverName = rustls::ServerName::try_from(dnsName.as_ref()).expect("url error");
    let mut tlsClient = ClientConnection::new(Arc::new(tlsconfig), serverName).unwrap();
    let mut tlsConn = StreamOwned::new(tlsClient, socket);
    tlsConn
}

pub fn ReadResponse(mut r: impl BufRead, req: &Request) -> HttpResult {
    let mut resp = Response::default();
    resp.Request = req.clone();
    // parse status line。
    let mut line = String::new();
    r.read_line(&mut line)?;
    let i = strings::IndexByte(line.as_str(), b' ');
    if i == -1 {
        return Err(Error::new(ErrorKind::Other, "malformed HTTP response"));
    }
    resp.Proto = line.get(..i as usize).unwrap().to_string();
    resp.Status =
        strings::TrimLeft(&line.as_str()[i as usize + 1..len!(line) - 2], " ").to_string();
    let mut statusCode = resp.Status.as_str();
    let i = strings::IndexByte(resp.Status.as_str(), b' ');
    if i != -1 {
        statusCode = &resp.Status.as_str()[..i as usize];
    }
    if len!(statusCode) != 3 {
        return Err(Error::new(ErrorKind::Other, "malformed HTTP status code"));
    }
    resp.StatusCode = statusCode.parse::<int>().unwrap();
    if resp.StatusCode < 0 {
        return Err(Error::new(ErrorKind::Other, "malformed HTTP status code"));
    }

    let vers = ParseHTTPVersion(resp.Proto.as_str());
    let ok = vers.2;
    if !ok {
        return Err(Error::new(ErrorKind::Other, "malformed HTTP version"));
    }
    resp.ProtoMajor = vers.0;
    resp.ProtoMinor = vers.1;
    let mut response: Vec<u8> = vec![];
    // split Response to headpart and bodyPart
    r.read_to_end(&mut response);
    // 下面的loop 跟read_to_end功能一样
    /* loop {
        if let Ok(buf) = r.fill_buf() {
            response.extend_from_slice(&buf);
            let length = buf.len();
            if length == 0 {
                break;
            }
            r.consume(length);
        } else {
            break;
        }
    } */
    let startIndex = startIndexOfBody(&response).unwrap();
    let headPart: Vec<u8> = response[..(startIndex - 2_usize)].to_vec();
    let bodyPart: Vec<u8> = response[startIndex + 1..].to_vec();
    // println!("bodyPart_len: {}", bodyPart.len());
    // parse headPart
    resp.Header = Header::NewWithHashMap(parseHeader(headPart));
    fixPragmaCacheControl(&mut resp.Header);
    // set Body
    if resp.Header.Get("Transfer-Encoding").as_str() == "chunked" {
        resp.Body.replace(parseChunkedBody(&bodyPart));
    } else {
        resp.Body = Some(bodyPart);
    }
    resp.ContentLength = len!(&resp.Body.as_ref().unwrap()) as i64;
    Ok(resp)
}

fn parseChunkedBody(chunkedBody: &Vec<u8>) -> Vec<u8> {
    let mut body: Vec<u8> = Vec::new();
    let mut lineSep: Vec<u8> = Vec::new();
    let mut isSizeLine = true;
    for &b in chunkedBody {
        if b == b'\r' || b == b'\n' || b == b'0' {
            lineSep.push(b);
        } else {
            if !isSizeLine {
                body.push(b);
            }
            lineSep.clear();
        }
        if lineSep.as_slice() == b"\r\n" || lineSep.as_slice() == b"\r\n0" {
            isSizeLine = false;
        }
    }
    body
}

pub type MIMEHeader = HashMap<String, Vec<String>>;

fn fixPragmaCacheControl(header: &mut Header) {
    if let Some(hp) = header.0.get("Pragma") {
        if len!(hp) > 0 && &hp[0] == "no-cache" {
            if header.0.get("Cache-Control").is_none() {
                header.Set("Cache-Control", "no-cache");
            }
        }
    }
}

fn parseHeader(headPart: Vec<u8>) -> MIMEHeader {
    let mut m: MIMEHeader = HashMap::new();
    let lines = std::str::from_utf8(headPart.as_slice()).unwrap();

    for kv in lines.split("\r\n").into_iter() {
        let mut i = strings::IndexByte(kv, b':');
        if i < 0 {
            continue;
        }

        let key = canonicalMIMEHeaderKey(kv.get(..i as usize).unwrap());
        if key == "".to_string() {
            continue;
        }
        i += 1;
        while (uint!(i) < len!(kv.as_bytes())
            && (kv.as_bytes()[i as usize] == b' ' || kv.as_bytes()[i as usize] == b'\t'))
        {
            i += 1;
        }
        let mut vv = Vec::<String>::new();
        let value = strings::TrimFunc(string(&kv.as_bytes()[i as usize..]).trim(), |x| {
            x == '\"' as u32
        })
        .to_string();

        if let Some(mut v) = m.get(&key) {
            vv = v.to_owned();
            vv.push(value);
            m.insert(key, vv.to_owned());
        } else {
            if len!(value) > 0 {
                vv.push(value);
                m.insert(key, vv.clone());
            }
        }
    }
    m
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

fn validHeaderFieldByte(b: byte) -> bool {
    let isTokenTable: HashMap<char, bool> = [
        ('!', true),
        ('#', true),
        ('$', true),
        ('%', true),
        ('&', true),
        ('\'', true),
        ('*', true),
        ('+', true),
        ('-', true),
        ('.', true),
        ('0', true),
        ('1', true),
        ('2', true),
        ('3', true),
        ('4', true),
        ('5', true),
        ('6', true),
        ('7', true),
        ('8', true),
        ('9', true),
        ('A', true),
        ('B', true),
        ('C', true),
        ('D', true),
        ('E', true),
        ('F', true),
        ('G', true),
        ('H', true),
        ('I', true),
        ('J', true),
        ('K', true),
        ('L', true),
        ('M', true),
        ('N', true),
        ('O', true),
        ('P', true),
        ('Q', true),
        ('R', true),
        ('S', true),
        ('T', true),
        ('U', true),
        ('W', true),
        ('V', true),
        ('X', true),
        ('Y', true),
        ('Z', true),
        ('^', true),
        ('_', true),
        ('`', true),
        ('a', true),
        ('b', true),
        ('c', true),
        ('d', true),
        ('e', true),
        ('f', true),
        ('g', true),
        ('h', true),
        ('i', true),
        ('j', true),
        ('k', true),
        ('l', true),
        ('m', true),
        ('n', true),
        ('o', true),
        ('p', true),
        ('q', true),
        ('r', true),
        ('s', true),
        ('t', true),
        ('u', true),
        ('v', true),
        ('w', true),
        ('x', true),
        ('y', true),
        ('z', true),
        ('|', true),
        ('~', true),
    ]
    .iter()
    .cloned()
    .collect();

    return int!(b) < int!(len!(isTokenTable)) && isTokenTable.get(&(b as char)).is_some();
}

const toLower: byte = (b'a' - b'A');
fn canonicalMIMEHeaderKey(a: &str) -> String {
    let mut a = a.to_owned();
    for c in a.as_bytes() {
        if validHeaderFieldByte(*c) {
            continue;
        }
        return string(a.as_bytes());
    }
    let mut upper = true;
    let mut new = String::new();
    for (i, &c) in a.as_bytes().iter().enumerate() {
        let mut c1 = c;
        if upper && b'a' <= c && c <= b'z' {
            c1 -= toLower;
        } else if !upper && b'A' <= c && c <= b'Z' {
            c1 += toLower;
        }
        new.push(c1 as char);

        upper = c1 == b'_';
    }
    new.clone()
}

pub fn ParseHTTPVersion(vers: &str) -> (int, int, bool) {
    let Big = 1000000;
    match vers {
        "HTTP/1.1" => return (1, 1, true),
        "HTTP/1.0" => return (1, 0, true),
        _ => {
            if !strings::HasPrefix(vers, "HTTP/") {
                return (0, 0, false);
            }

            let dot = strings::Index(vers, ".");

            if dot < 0 {
                return (0, 0, false);
            }
            let mut major = 0;
            let mut minor = 0;

            if let Ok(mj) = vers.get(5..dot as usize).unwrap().parse::<int>() {
                major = mj;
                if major < 0 || major > Big {
                    return (0, 0, false);
                }
            } else {
                return (0, 0, false);
            }

            if let Ok(mi) = vers.get(dot as usize + 1..).unwrap().parse::<int>() {
                minor = mi;
                return (0, 0, false);
            } else {
                return (0, 0, false);
            }
            return (major, minor, true);
        }
    }
}
