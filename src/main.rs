#![deny(warnings)]

use std::net::SocketAddr;
use std::fs::read_to_string;

use hyper::server::conn::http1;
use tokio::net::TcpListener;

use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, Result, StatusCode};

use select::document::Document;
use select::predicate::{Attr};

static INDEX: &str = "frontend/src/index.html";
static NOT_FOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    // Read the HTML file from disk
    let original_html = read_to_string(INDEX).unwrap();

    // Parse the HTML document
    let document = Document::from(original_html.as_str());

    // Find all elements with an attribute called "path"
    let path_elements = document.find(Attr("path", ())).next().unwrap();
    println!("First: {}", path_elements.html().as_str());

    println!("Found element with path attribute: {:#?}", path_elements);

    let path = path_elements.attr("path");
    println!("Path {:#?}", path);
    // let Some(path_value) = path;

    if let Some(path_value) = path {
        println!("Path Value {:#?}", path_value);
        let replacement_html = read_to_string(path_value.replace("./", "frontend/src/")).unwrap();

        // Replace the contents of the element with the contents of the replacement HTML file
        // let x = path_elements.html().as_str();
        let new_elements = path_elements.html().as_str().replace(path_value, replacement_html.as_str());
        println!("Test {:#?}", new_elements);
    }

    // Print the path attribute value of each matching element
    // for path_element in path_elements {
    //     println!("Found element: {:#?}", path_element);
    //     if let Some(path_value) = path_element.attr("path") {
    //         println!("Found element with path attribute: {}", path_value);
    //         let replacement_html = fs::read_to_string(path_value).unwrap();

    //         let document = Document::from(file_content.as_str());

    //         // Replace the contents of the element with the contents of the replacement HTML file
    //         path_elements.replace_with(replacement_html.as_str());
    //     }
    // }

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(response_examples))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn response_examples(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOT_FOUND.into()))
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Full<Bytes>>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }

    Ok(not_found())
}