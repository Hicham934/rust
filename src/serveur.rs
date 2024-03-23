use hyper::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn select_backend() -> Option<String> {
    Some("http://127.0.0.1:3001".to_string())
}

pub async fn maintain_backends_health() {
    let _client = Client::new();
    println!("Maintaining backend health...");
    
}
