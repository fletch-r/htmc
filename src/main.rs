#![deny(warnings)]

use std::net::SocketAddr;
use std::fs::read_to_string;
use std::io::Write;

use hyper::server::conn::http1;
use tokio::net::TcpListener;

use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, Result, StatusCode};

use select::document::Document;
use select::predicate::{Attr};

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("frontend/src/index.html", RecursiveMode::Recursive).unwrap();

    // server().await?;

    loop {
        match receiver.recv() {
            Ok(event) => {
                println!("File Name: {:?}", event);
                server().await?;
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

async fn server() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    replace_html().await;

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

static INDEX: &str = "frontend/src/index.html";
static NOT_FOUND: &[u8] = b"Not Found";

async fn replace_html() {
    // Read the HTML file from disk
    let original_html = read_to_string(INDEX).unwrap();

    // Parse the HTML document
    let document = Document::from(original_html.as_str());

    let mut x = String::from("");
    for path_element in document.find(Attr("path", ())) {
        let path = path_element.attr("path").unwrap();
        println!("Path {:#?}", path);

        let replacement_html = read_to_string(path.replace("./", "frontend/src/")).unwrap();

        if x == "" {
            // Replace the contents of the element with the contents of the replacement HTML file
            let new_elements = original_html.as_str().replace(path_element.html().as_str(), replacement_html.as_str());
            x = new_elements;
        } else {
            let new_elements = x.replace(path_element.html().as_str(), replacement_html.as_str());
            x = new_elements;
        }
    }
    // Create a new merged file in the out folder
    let mut file = std::fs::File::create("./out/index.html").expect("create failed");
    file.write_all(x.as_bytes()).expect("write failed");
    println!("data written to file" );
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