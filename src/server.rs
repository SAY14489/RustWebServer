use crate::handler::Handler;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use http::{Request, Response};

pub struct Server {
    addr: SocketAddr,
    routes: HashMap<String, Arc<dyn Handler>>,
}

impl Server {
    pub fn bind(addr: impl Into<SocketAddr>) -> Self {
        Self {
            addr: addr.into(),
            routes: HashMap::new(),
        }
    }

    pub fn route(mut self, path: &str, handler: impl Handler + 'static) -> Self{
      self.routes.insert(path.to_owned(), Arc::new(handler));
      return self;
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(self.addr).await?;
        println!("Server listening on {}", self.addr);

        loop {
            let (mut stream, _) = listener.accept().await?;
            let routes = self.routes.clone();
            
            tokio::spawn(async move {
              let mut buffer = [0; 1024];
              
              // Read the request
              match stream.read(&mut buffer).await {
                  Ok(n) => {
                      let request = String::from_utf8_lossy(&buffer[..n]);
                      
                      // Request parsing
                      let path = request.lines()
                          .next()
                          .and_then(|line| line.split_whitespace().nth(1))
                          .unwrap_or("/");
                          
                      // Find the handler for this path
                      let maybe_handler = routes.get(path);
                      let http_request = Request::builder()
                          .uri(path)
                          .body(request.to_string())
                          .unwrap();
                      let response = match maybe_handler {
                          Some(handler) => {
                              // We found a handler
                              handler.handle(&http_request).await
                          }
                          None => {
                              // No handler found for this path, return 404
                              Response::builder()
                                  .status(404)
                                  .body("404: Not Found".to_string())
                                  .unwrap()
                          }
                      };
                      
                      // Convert response to HTTP format
                      let response_string = format!(
                          "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                          response.status().as_str(),
                          response.status().canonical_reason().unwrap_or(""),
                          response.body().len(),
                          response.body()
                      );
                      
                      // Send the response
                      let write_result = stream.write_all(response_string.as_bytes()).await;
                      match write_result {
                          Ok(_) => {
                              // Write succeeded, nothing to do
                          }
                          Err(e) => {
                              eprintln!("Failed to write response: {}", e);
                          }
                      }
                  }
                  Err(e) => eprintln!("Failed to read from connection: {}", e),
              }
          });
        }
    }
}