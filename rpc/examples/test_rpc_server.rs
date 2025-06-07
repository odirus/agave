//! Simple test RPC server with HTTP and Native Unix Socket support
//! This is used to test the performance difference between protocols

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde_json::{json, Value};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use std::os::unix::fs::PermissionsExt;

// Simple JSON-RPC request handler
fn handle_jsonrpc_request(request: &str) -> String {
    // Parse JSON-RPC request
    let parsed: Result<Value, _> = serde_json::from_str(request);
    
    match parsed {
        Ok(req) => {
            let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
            let id = req.get("id");
            
            let result = match method {
                "getHealth" => json!("ok"),
                "getVersion" => json!({
                    "solana-core": "1.18.0",
                    "feature-set": 12345
                }),
                "getSlot" => json!(12345678),
                _ => json!(null)
            };
            
            let response = json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": result
            });
            
            response.to_string()
        }
        Err(_) => {
            json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {
                    "code": -32700,
                    "message": "Parse error"
                }
            }).to_string()
        }
    }
}

// HTTP server handler
async fn handle_http_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::POST => {
            let body_bytes = match hyper::body::to_bytes(req.into_body()).await {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Failed to read request body"))
                        .unwrap());
                }
            };
            
            let body_str = match std::str::from_utf8(&body_bytes) {
                Ok(s) => s,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid UTF-8 in request body"))
                        .unwrap());
                }
            };
            
            let response = handle_jsonrpc_request(body_str);
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(response))
                .unwrap())
        }
        &Method::OPTIONS => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                .body(Body::empty())
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::from("Method not allowed"))
                .unwrap())
        }
    }
}

// HTTP server
async fn start_http_server(addr: SocketAddr, exit: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_http_request))
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    println!("HTTP RPC server listening on http://{}", addr);
    
    let graceful = server.with_graceful_shutdown(async move {
        while !exit.load(Ordering::Relaxed) {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    
    if let Err(e) = graceful.await {
        eprintln!("HTTP server error: {}", e);
    }
    
    Ok(())
}



// Native Unix Socket server
async fn start_native_unix_server(socket_path: &Path, exit: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    
    let listener = UnixListener::bind(socket_path)?;
    std::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o660))?;
    println!("Native Unix Socket RPC server listening on {:?}", socket_path);
    
    loop {
        if exit.load(Ordering::Relaxed) {
            break;
        }
        
        match tokio::time::timeout(tokio::time::Duration::from_millis(100), listener.accept()).await {
            Ok(Ok((mut stream, _))) => {
                tokio::spawn(async move {
                    let mut buffer = [0; 4096];
                    let mut accumulated_data = String::new();
                    
                    loop {
                        match stream.read(&mut buffer).await {
                            Ok(0) => break, // EOF
                            Ok(n) => {
                                let data = String::from_utf8_lossy(&buffer[..n]);
                                accumulated_data.push_str(&data);
                                
                                // Process complete lines
                                while let Some(newline_pos) = accumulated_data.find('\n') {
                                    let line = accumulated_data[..newline_pos].trim().to_string();
                                    accumulated_data.drain(..=newline_pos);
                                    
                                    if line.is_empty() {
                                        continue;
                                    }
                                    
                                    // Process JSON-RPC request directly
                                    let response = handle_jsonrpc_request(&line);
                                    
                                    if let Err(e) = stream.write_all(response.as_bytes()).await {
                                        eprintln!("Failed to write response: {}", e);
                                        return;
                                    }
                                    if let Err(e) = stream.write_all(b"\n").await {
                                        eprintln!("Failed to write newline: {}", e);
                                        return;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading from native Unix socket: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Ok(Err(e)) => {
                eprintln!("Failed to accept native Unix connection: {}", e);
            }
            Err(_) => {
                // Timeout, continue loop
                continue;
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Test RPC Server");
    println!("Modes: HTTP/TCP + Native/Unix");
    println!("{}", "=".repeat(50));
    
    let exit = Arc::new(AtomicBool::new(false));
    
    // Start HTTP server
    let http_exit = exit.clone();
    let http_handle = tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8899));
        if let Err(e) = start_http_server(addr, http_exit).await {
            eprintln!("HTTP server error: {}", e);
        }
    });
    
    // Start Native Unix server
    let native_unix_exit = exit.clone();
    let native_unix_handle = tokio::spawn(async move {
        let socket_path = Path::new("/tmp/solana-rpc.sock");
        if let Err(e) = start_native_unix_server(socket_path, native_unix_exit).await {
            eprintln!("Native Unix server error: {}", e);
        }
    });
    
    println!("\n✅ All servers started successfully!");
    println!("🌐 HTTP: http://127.0.0.1:8899");
    println!("⚡ Native/Unix: /tmp/solana-rpc.sock");
    println!("\nPress Ctrl+C to stop...");
    
    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;
    
    println!("\n🛑 Shutting down servers...");
    exit.store(true, Ordering::Relaxed);
    
    // Wait for all servers to shutdown
    let _ = tokio::join!(http_handle, native_unix_handle);
    
    // Cleanup socket files
    let _ = std::fs::remove_file("/tmp/solana-rpc.sock");
    
    println!("✅ All servers stopped successfully!");
    
    Ok(())
} 