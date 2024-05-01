use http::StatusCode;
use std::collections::HashMap;
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}   