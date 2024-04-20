use std::io;
use async_std::io::BufReader;
use async_std::io::WriteExt;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;
use async_std::task::spawn;
use futures::AsyncBufReadExt;
use crate::http_request::HttpRequest;
use crate::router::Router;

pub mod http_request;
pub mod http_response;
pub mod router;
pub mod classifiers;
pub mod models;
pub mod core;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
    .await;
}

async fn handle_connection(mut stream: TcpStream ) -> io::Result<()> {

    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();
    
    let mut contents: Vec<String> = Vec::new();
    while let Some(line) = lines.next().await {
        let line = line?;
        contents.push(line.to_owned());
    }

    let request: Result<HttpRequest, &str> = HttpRequest::parse(&contents);
    Router::route(request.unwrap()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let response_text = "hello world!";
    let content_length = response_text.len();
    let response = format!(
        "{status_line}\r\nContent-Length:{content_length}\r\n\r\n{response_text}"
    );
    stream.write_all(response.as_bytes()).await.unwrap();
    Ok(())
}