//! Simple Unix Socket client test to verify our fix

use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

fn test_native_unix_socket(socket_path: &Path, request: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("🔗 Connecting to Unix socket: {:?}", socket_path);
    
    // 直接连接Unix Socket
    let mut stream = UnixStream::connect(socket_path)?;
    println!("✅ Connected successfully");
    
    // 发送原始JSON-RPC数据（无HTTP头）
    println!("📤 Sending request: {}", request);
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?; // 添加换行符作为消息分隔符
    println!("✅ Request sent");
    
    // 读取一行响应（服务器以换行符结束响应）
    println!("📥 Reading response...");
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("✅ Response received: {}", response.trim());
    
    Ok(response.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Unix Socket Client Fix");
    println!("{}", "=".repeat(50));
    
    let socket_path = Path::new("/tmp/solana-rpc.sock");
    
    // 检查socket文件是否存在
    if !socket_path.exists() {
        println!("❌ Socket file does not exist: {:?}", socket_path);
        println!("💡 Please start the test RPC server first:");
        println!("   cargo run --example test_rpc_server");
        return Ok(());
    }
    
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth",
        "params": []
    });
    
    match test_native_unix_socket(socket_path, &request.to_string()) {
        Ok(response) => {
            println!("\n🎉 SUCCESS!");
            println!("Response: {}", response);
            
            // 验证响应格式
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                if parsed.get("jsonrpc").is_some() && parsed.get("id").is_some() {
                    println!("✅ Valid JSON-RPC response format");
                } else {
                    println!("⚠️  Response format may be incorrect");
                }
            } else {
                println!("❌ Invalid JSON response");
            }
        }
        Err(e) => {
            println!("\n❌ FAILED!");
            println!("Error: {}", e);
        }
    }
    
    Ok(())
} 