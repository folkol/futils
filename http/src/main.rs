const CODES: &[&str] = &[
    "100 Continue (Informational)",
    "101 Switching Protocols (Informational)",
    "102 Processing (Informational)",
    "200 OK (Success)",
    "201 Created (Success)",
    "202 Accepted (Success)",
    "203 Non-authoritative Information (Success)",
    "204 No Content (Success)",
    "205 Reset Content (Success)",
    "206 Partial Content (Success)",
    "207 Multi-Status (Success)",
    "208 Already Reported (Success)",
    "226 IM Used (Success)",
    "300 Multiple Choices (Redirection)",
    "301 Moved Permanently (Redirection)",
    "302 Found (Redirection)",
    "303 See Other (Redirection)",
    "304 Not Modified (Redirection)",
    "305 Use Proxy (Redirection)",
    "307 Temporary Redirect (Redirection)",
    "308 Permanent Redirect (Redirection)",
    "400 Bad Request (Client Error)",
    "401 Unauthorized (Client Error)",
    "402 Payment Required (Client Error)",
    "403 Forbidden (Client Error)",
    "404 Not Found (Client Error)",
    "405 Method Not Allowed (Client Error)",
    "406 Not Acceptable (Client Error)",
    "407 Proxy Authentication Required (Client Error)",
    "408 Request Timeout (Client Error)",
    "409 Conflict (Client Error)",
    "410 Gone (Client Error)",
    "411 Length Required (Client Error)",
    "412 Precondition Failed (Client Error)",
    "413 Payload Too Large (Client Error)",
    "414 Request-URI Too Long (Client Error)",
    "415 Unsupported Media Type (Client Error)",
    "416 Requested Range Not Satisfiable (Client Error)",
    "417 Expectation Failed (Client Error)",
    "418 I'm a teapot (Client Error)",
    "421 Misdirected Request (Client Error)",
    "422 Unprocessable Entity (Client Error)",
    "423 Locked (Client Error)",
    "424 Failed Dependency (Client Error)",
    "426 Upgrade Required (Client Error)",
    "428 Precondition Required (Client Error)",
    "429 Too Many Requests (Client Error)",
    "431 Request Header Fields Too Large (Client Error)",
    "444 Connection Closed Without Response (Client Error)",
    "451 Unavailable For Legal Reasons (Client Error)",
    "499 Client Closed Request (Client Error)",
    "500 Internal Server Error (Server Error)",
    "501 Not Implemented (Server Error)",
    "502 Bad Gateway (Server Error)",
    "503 Service Unavailable (Server Error)",
    "504 Gateway Timeout (Server Error)",
    "505 HTTP Version Not Supported (Server Error)",
    "506 Variant Also Negotiates (Server Error)",
    "507 Insufficient Storage (Server Error)",
    "508 Loop Detected (Server Error)",
    "510 Not Extended (Server Error)",
    "511 Network Authentication Required (Server Error)",
    "599 Network Connect Timeout Error (Server Error)",
];

fn main() {
    let mut found = false;
    let query: Vec<_> = std::env::args()
        .skip(1)
        .map(|x| x.replace('x', ""))
        .collect();
    for code in CODES {
        if query.iter().any(|x| code.starts_with(x)) {
            found = true;
            println!("{}", code)
        }
    }
    if !found {
        println!("No match")
    }
}
