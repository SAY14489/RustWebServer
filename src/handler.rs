use async_trait::async_trait;
use http::{Request, Response};
use std::path::PathBuf;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: &Request<String>) -> Response<String>;
}

// Basic static file handler
pub struct StaticFile {
    path: PathBuf,
}

impl StaticFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
        }
    }
}

#[async_trait]
impl Handler for StaticFile {
    async fn handle(&self, _request: &Request<String>) -> Response<String> {
        match tokio::fs::read_to_string(&self.path).await {
            Ok(contents) => Response::builder()
                .status(200)
                .body(contents)
                .unwrap(),
            Err(_) => Response::builder()
                .status(404)
                .body("404: File not found".to_string())
                .unwrap(),
        }
    }
}