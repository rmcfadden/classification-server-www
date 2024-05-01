use crate::http_request::HttpRequest;

pub struct Router {}

impl Router {
    pub fn route(request: HttpRequest) -> Result<(), &'static str> {
        println!("REQUEST:::::{:?}", request.url.path());

        let result  = match request.url.path() {
            "/classify" => "classify",
            _ => return Err("Invalid path passed"),
        };
        // classifiers
        //
        println!("result: {result}");
        Ok(())
 }  
}