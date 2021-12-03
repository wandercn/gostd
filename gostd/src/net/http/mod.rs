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

pub struct Client {
    Transport: Box<dyn RoundTripper>,
    CheckRedirect: fn(req: &Request, via: Vec<&Request>) -> Result<(), Error>,
    Jar: Box<dyn CookieJar>,
    Timeout: time::Duration,
}

type CResponse = Result<Response, Error>;
impl Client {
    pub fn Get(&mut self, url: &str) -> CResponse {
        let mut req = Request::New(Method::Get, url)?;
        self.Do(&req)
    }
    pub fn Post(&mut self, url: &str, contentType: &str, body: Box<dyn Reader>) -> CResponse {
        let req = Request::NewWithBody(Method::Post, url, body)?;
        // req.Header.Set("Content-Type", contentType)
        self.Do(&req)
    }

    pub fn PostForm(&mut self, url: &str, data: url::Values) -> CResponse {
        self.Post(
            url,
            "application/x-www-form-urlencoded",
            Box::new(strings::Reader::new(data.Encode().as_str())),
        )
    }
    pub fn Head(&mut self, url: &str) -> CResponse {
        let mut req = Request::New(Method::Head, url)?;
        self.Do(&req)
    }

    pub fn Do(&mut self, req: &Request) -> CResponse {
        self.done(req)
    }

    fn done(&mut self, req: &Request) -> CResponse {
        todo!()
    }
}

pub trait RoundTripper {
    fn RoundTrip(&self, r: Request) -> Result<Response, Error>;
}

fn refererForURL(lastReq: &url::URL, newReq: &url::URL) -> String {
    if (lastReq.Scheme == "https") && (newReq.Scheme == "http") {
        return "".to_string();
    }
    let mut referer = lastReq.String();
    if let Some(user) = lastReq.User {
        return referer;
    }
    let auth = "@";
    referer = strings::Replace(referer.as_str(), auth, "", 1);
    referer
}

pub struct Request<'a> {
    Method: Method,
    URL: url::URL<'a>,
    Proto: &'a str,
    ProtoMajor: int,
    ProtoMinor: int,
    Header: Header,
    // Body io.ReadCloser
    // GetBody func() (io.ReadCloser, error)
    ContentLength: int64,
    TransferEncoding: Vec<String>,
    Close: bool,
    Host: &'a str,
    Form: url::Values,
    PostForm: url::Values,
    // MultipartForm:*multipart.Form,
    Trailer: Header,
    RemoteAddr: &'a str,
    RequestURI: &'a str,
    // TLS *tls.ConnectionState,
    // Cancel <-chan struct{}
    // ctx context.Context
}

impl<'a> Request<'a> {
    pub fn New(method: Method, url: &str) -> Result<Request, Error> {
        todo!()
    }
    pub fn NewWithBody(method: Method, url: &str, body: Box<dyn Reader>) -> Result<Request, Error> {
        todo!()
    }
}
#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Response {}

pub trait CookieJar {
    fn SetCookies(&self, u: &url::URL, cookies: Vec<&Cookie>);

    fn Cookies(&self, u: &url::URL) -> Vec<&Cookie>;
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Header(HashMap<String, Vec<String>>);

impl Header {
    pub fn Add(&mut self, key: &str, value: &str) {
        todo!()
    }

    pub fn Set(&mut self, key: &str, value: &str) {
        todo!()
    }
}

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Cookie<'a> {
    Name: &'a str,
    Value: &'a str,
    Path: &'a str,       // optional
    Domain: &'a str,     // optional
    Expires: time::Time, // optional
    RawExpires: &'a str, // for reading cookies only

    // MaxAge=0 means no 'Max-Age' attribute specified.
    // MaxAge<0 means delete cookie now, equivalently 'Max-Age: 0'
    // MaxAge>0 means Max-Age attribute present and given in seconds
    MaxAge: int,
    Secure: bool,
    HttpOnly: bool,
    SameSite: SameSite,
    Raw: &'a str,
    Unparsed: Vec<&'a str>, // Raw text of unparsed attribute-value pairs
}

// SameSite allows a server to define a cookie attribute making it impossible for
// the browser to send this cookie along with cross-site requests. The main
// goal is to mitigate the risk of cross-origin information leakage, and provide
// some protection against cross-site request forgery attacks.
//
// See https://tools.ietf.org/html/draft-ietf-httpbis-cookie-same-site-00 for details.
type SameSite = int;
