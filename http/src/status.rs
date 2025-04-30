#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

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
