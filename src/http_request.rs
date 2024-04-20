use url::Url;
use http::Method;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub url: Url,
    pub method: Method,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn parse(requests: &Vec<String>) -> Result<HttpRequest, &'static str> {
        let request_line =  match requests.first() {
            Some(line) => line,
            None => return Err("request line cannot be empty"),
        };

        let request_parts  = request_line.split(" ").collect::<Vec<&str>>();
        if request_parts.len() != 3 {
            return Err("Expected thee parts in request_line: {request_line}");
        }

        let mut url = "https://example.com".to_owned(); // url:parse only supports absolute urls
        url.push_str(request_parts[1]);

        let request = HttpRequest {
            url: Url::parse(&url).unwrap(),
            method: Method::from_bytes(&request_parts[0].as_bytes()).unwrap(),
            headers: requests.iter().skip(1)
                .filter(|&x| !x.is_empty())
                .map(|s| s.split_once(':').unwrap())
                .map(|(key, value)| (key.to_string(), value.to_string())).collect()
        };

        Ok(request)
    }
}