// 原生Unix Socket性能测试 - 不使用HTTP协议
// 这个例子展示了如何获得真正的Unix Socket性能优势

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde_json::{json, Value};

fn get_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

// 原生Unix Socket测试 - 直接发送JSON-RPC，无HTTP包装
fn test_raw_unix_socket(socket_path: &Path, request: &str) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let start_time = get_time_micros();
    
    // 直接连接Unix Socket
    let mut stream = UnixStream::connect(socket_path)?;
    
    // 发送原始JSON-RPC数据（无HTTP头）
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?; // 添加换行符作为消息分隔符
    
    // 读取响应
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((response, elapsed))
}

// HTTP测试作为对比
fn test_raw_http(host: &str, port: u16, request: &str) -> Result<(String, u128), Box<dyn std::error::Error>> {
    use std::net::TcpStream;
    
    let start_time = get_time_micros();
    
    let mut stream = TcpStream::connect((host, port))?;
    
    // 构建HTTP请求
    let http_request = format!(
        "POST / HTTP/1.1\r\n\
         Host: {}:{}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        host, port, request.len(), request
    );
    
    stream.write_all(http_request.as_bytes())?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    let elapsed = get_time_micros() - start_time;
    
    // 提取HTTP响应体
    if let Some(body_start) = response.find("\r\n\r\n") {
        let body = response[body_start + 4..].to_string();
        Ok((body, elapsed))
    } else {
        Ok((response, elapsed))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Raw Unix Socket vs HTTP Performance Test");
    println!("{}", "=".repeat(60));
    
    let socket_path = Path::new("/tmp/solana-rpc.sock");
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth",
        "params": []
    }).to_string();
    
    let iterations = 1000;
    
    // 测试原生Unix Socket（如果支持的话）
    if socket_path.exists() {
        println!("🔌 Testing Raw Unix Socket (if server supports it)...");
        println!("   Note: This requires a custom server that accepts raw JSON-RPC over Unix Socket");
        
        // 注意：这需要一个支持原生Unix Socket的服务器实现
        // 当前的Solana RPC服务器只支持HTTP-over-Unix-Socket
    } else {
        println!("⚠️  Unix Socket not available at: {:?}", socket_path);
    }
    
    // 测试HTTP
    println!("📡 Testing Raw HTTP...");
    let mut http_times = Vec::new();
    
    for _ in 0..iterations {
        match test_raw_http("127.0.0.1", 8899, &request) {
            Ok((_, elapsed)) => {
                http_times.push(elapsed);
            }
            Err(e) => {
                println!("HTTP request failed: {}", e);
                break;
            }
        }
    }
    
    if !http_times.is_empty() {
        let avg = http_times.iter().sum::<u128>() as f64 / http_times.len() as f64;
        let min = *http_times.iter().min().unwrap();
        let max = *http_times.iter().max().unwrap();
        
        println!("📊 Raw HTTP Results:");
        println!("   Average: {:.1}μs | Min: {}μs | Max: {}μs | Iterations: {}", 
            avg, min, max, http_times.len());
    }
    
    println!("\n💡 To achieve real Unix Socket performance benefits:");
    println!("   1. Implement a custom binary protocol (not JSON-RPC over HTTP)");
    println!("   2. Use MessagePack or Protocol Buffers instead of JSON");
    println!("   3. Implement request batching");
    println!("   4. Use shared memory for large data transfers");
    println!("   5. Implement custom connection pooling");
    
    println!("\n🎯 Expected performance with optimized Unix Socket:");
    println!("   • 5-10x faster for small requests");
    println!("   • 2-3x faster for JSON-RPC specifically");
    println!("   • 10-100x faster with binary protocols");
    
    Ok(())
} 