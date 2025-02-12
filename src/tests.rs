use crate::{Server, Handler, StaticFile};
use std::net::SocketAddr;
use http::{Request, Response};
use async_trait::async_trait;
use std::time::Instant;

// Test handler that returns "pong"
struct PingHandler;

#[async_trait]
impl Handler for PingHandler {
    async fn handle(&self, _request: &Request<String>) -> Response<String> {
        Response::builder()
            .status(200)
            .body("pong".to_string())
            .unwrap()
    }
}

// Test handler that returns what was in the request
struct EchoHandler;

#[async_trait]
impl Handler for EchoHandler {
    async fn handle(&self, request: &Request<String>) -> Response<String> {
        Response::builder()
            .status(200)
            .body(format!("You requested: {}", request.uri().path()))
            .unwrap()
    }
}

pub async fn test_basic_server() {
    println!("\n=== Testing Basic Server ===");
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    println!("Visit http://{} in your browser, a static file will be served", addr);

    let _server = Server::bind(addr)
        .route("/", StaticFile::new("static/index.html"))
        .serve()
        .await;
}

pub async fn test_dynamic_handler() {
  println!("\n=== Testing Dynamic Handler (EchoHandler) ===");
  let addr = SocketAddr::from(([127, 0, 0, 1], 8084));
  
  println!("Starting server at http://{}", addr);
  println!("\nTry these paths to test dynamic handling:");
  println!("1. http://{}/dynamic -> Should show 'You requested: /dynamic'", addr);
  println!("2. http://{}/dynamic/test -> Should show 'You requested: /dynamic/test'", addr);
  println!("3. http://{}/notfound -> Should show 404 error", addr);
  
  let _server = Server::bind(addr)
      .route("/dynamic", EchoHandler)
      .route("/dynamic/test", EchoHandler)
      .serve()
      .await;
}

pub async fn test_duplicate_routes() {
    println!("\n=== Testing Duplicate Routes ===");
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));

    println!("Visit http://{}/test in your browser", addr);
    println!("You should see 'pong': The original route had EchoHandler but was overwritten by PingHandler");

    let _server = Server::bind(addr)
        .route("/test", EchoHandler)
        .route("/test", PingHandler)  // Should override PingHandler
        .serve()
        .await;
}

pub async fn test_nonexistent_file() {
    println!("\n=== Testing Nonexistent File ===");
    let addr = SocketAddr::from(([127, 0, 0, 1], 8082));
    
    println!("Visit http://{} in your browser", addr);
    println!("Should see a 404 error");

    let _server = Server::bind(addr)
        .route("/", StaticFile::new("nonexistent.html"))
        .serve()
        .await;
}

pub async fn test_multiple_routes() {
    println!("\n=== Testing Multiple Routes ===");
    let addr = SocketAddr::from(([127, 0, 0, 1], 8083));
    
    println!("Try these URLs:");
    println!("1. http://{} (should show index.html)", addr);
    println!("2. http://{}/ping (should show 'pong')", addr);
    println!("3. http://{}/echo (should show the path you requested)", addr);
    println!("4. http://{}/unknown (should show 'Page Not Found')", addr);

    let _server = Server::bind(addr)
        .route("/", StaticFile::new("static/index.html"))
        .route("/ping", PingHandler)
        .route("/echo", EchoHandler)
        .serve()
        .await;
}

// Server setup and benchmark
pub async fn test_overhead_measurement() {
  println!("\n=== Testing Overhead for Dynamic Content ===");
  let addr = SocketAddr::from(([127, 0, 0, 1], 8085));

  println!("Starting server at http://{}", addr);
  println!("Use a benchmarking tool to generate load and measure overhead.");

  let _server = Server::bind(addr)
      .route("/overhead", TimedHandler)
      .serve()
      .await;
}

struct TimedHandler;

#[async_trait]
impl Handler for TimedHandler {
    async fn handle(&self, request: &Request<String>) -> Response<String> {
        let total_start = Instant::now();

        // Simulate some work in the handler
        let handler_start = Instant::now();
        let response_body = format!("You requested: {}", request.uri().path());
        let handler_duration = handler_start.elapsed();

        let response = Response::builder()
            .status(200)
            .body(response_body)
            .unwrap();

        let total_duration = total_start.elapsed();

        // Calculate overhead
        let overhead = total_duration - handler_duration;
        println!(
            "Handler time: {:?}, Overhead: {:?}, Total time: {:?}",
            handler_duration, overhead, total_duration
        );

        response
    }
}