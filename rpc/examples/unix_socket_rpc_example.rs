// 使用 Unix Socket 连接 Solana RPC 的示例
//
// 编译: cargo build --example unix_socket_rpc_example
// 运行: cargo run --example unix_socket_rpc_example

use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(unix)]
use {
    hyperlocal::{UnixClientExt, Uri},
    serde_json::{json, Value},
    std::path::PathBuf,
    tokio::time::{sleep, Duration},
};

fn get_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_micros()
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Unix Socket RPC Client Example");
    
    // Unix socket 路径 (这个路径需要和 RPC 服务器配置的路径一致)
    let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
    
    // 等待socket可用
    println!("Waiting for Unix socket at: {:?}", socket_path);
    while !socket_path.exists() {
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("Socket found, connecting...");
    
    // 创建 Unix socket 客户端
    let client = hyper::Client::unix();
    
    // 构造请求
    let rpc_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth",
        "params": []
    });
    
    let uri = Uri::new(&socket_path, "/");
    
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(rpc_request.to_string()))?;
    
    // 发送请求
    match client.request(req).await {
        Ok(response) => {
            let status = response.status();
            println!("Response status: {}", status);
            
            let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
            let body_str = String::from_utf8(body_bytes.to_vec())?;
            
            println!("Response body: {}", body_str);
            
            // 解析 JSON 响应
            if let Ok(json_response) = serde_json::from_str::<Value>(&body_str) {
                if let Some(result) = json_response.get("result") {
                    println!("Health status: {}", result);
                } else if let Some(error) = json_response.get("error") {
                    println!("RPC Error: {}", error);
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
        }
    }
    
    // 再试一个 getVersion 请求
    println!("\n--- Testing getVersion ---");

    let begin_time = get_time();
    let version_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "getVersion",
        "params": []
    });
    
    let uri = Uri::new(&socket_path, "/");
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(version_request.to_string()))?;
    
    match client.request(req).await {
        Ok(response) => {
            let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
            let body_str = String::from_utf8(body_bytes.to_vec())?;
            
            println!("elapsed: {}, Version response: {}", get_time() - begin_time, body_str);
        }
        Err(e) => {
            eprintln!("Version request failed: {}", e);
        }
    }
    
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    println!("Unix socket example is only available on Unix-like systems");
} 