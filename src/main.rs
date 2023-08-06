use clap::{App, Arg};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::Value;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let in_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let (method, uri) = (req.method().clone(), req.uri().clone());

    let bytes = hyper::body::to_bytes(req.into_body()).await?;
    let content = String::from_utf8_lossy(&bytes);

    println!("⏰ Timestamp: {:?}, 🌍 {:?} {:?}", in_ms, method, uri);

    if !content.is_empty() {
        let parsed_json = serde_json::from_str::<Value>(&content)
            .unwrap_or_else(|_| Value::String("Invalid JSON".into()));
        println!("{}", serde_json::to_string_pretty(&parsed_json).unwrap());
    }

    println!("-----------------------------------------------------------------------------");

    Ok(Response::new(Body::from("🪰")))
}

fn get_local_ip() -> Option<String> {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| {
            socket
                .connect("8.8.8.8:80")
                .and_then(|_| socket.local_addr())
        })
        .map(|addr| addr.ip().to_string())
        .ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("🪰 Mosquito")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("Sets the host IP address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets the port")
                .takes_value(true),
        )
        .get_matches();

    let host = matches
        .value_of("host")
        .map(String::from)
        .unwrap_or_else(|| get_local_ip().unwrap_or_else(|| "0.0.0.0".to_string()));
    let port = matches.value_of("port").unwrap_or("8000");
    let addr = format!("{}:{}", host, port).parse()?;

    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    }));

    println!("🪰 Mosquito Web Server running on http://{}", addr);

    server.await?;

    Ok(())
}
