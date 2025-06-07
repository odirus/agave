// HTTP vs Unix Socket RPC 性能对比示例
//
// 编译: cargo build --example unix_socket_rpc_example
// 运行: cargo run --example unix_socket_rpc_example

use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

#[cfg(unix)]
use {
    hyperlocal::{UnixClientExt, Uri},
    std::path::PathBuf,
};

fn get_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_micros()
}

async fn make_rpc_request(method: &str, params: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params
    })
}

async fn test_http_rpc(url: &str, request: &Value) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();
    
    let start_time = get_time_micros();
    
    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(request.to_string()))?;
    
    let response = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((body_str, elapsed))
}

#[cfg(unix)]
async fn test_unix_socket_rpc(socket_path: &PathBuf, request: &Value) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let client = hyper::Client::unix();
    
    let start_time = get_time_micros();
    
    let uri = Uri::new(socket_path, "/");
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(request.to_string()))?;
    
    let response = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((body_str, elapsed))
}

async fn run_performance_test(method: &str, params: Value, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 Testing RPC method: {} ({} iterations)", method, iterations);
    println!("{}", "=".repeat(60));
    
    let request = make_rpc_request(method, params).await;
    
    // HTTP 测试
    let http_url = "http://127.0.0.1:8899";
    let mut http_times = Vec::new();
    let mut http_success_count = 0;
    
    println!("📡 Testing HTTP RPC...");
    for i in 0..iterations {
        match test_http_rpc(http_url, &request).await {
            Ok((response, elapsed)) => {
                http_times.push(elapsed);
                http_success_count += 1;
                if i == 0 {
                    // 只打印第一次的响应内容
                    if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                        if let Some(result) = json_response.get("result") {
                            println!("   ✅ Response: {}", serde_json::to_string_pretty(result)?);
                        }
                    }
                }
            }
            Err(e) => {
                if i == 0 {
                    println!("   ❌ HTTP request failed: {}", e);
                }
            }
        }
    }
    
    // Unix Socket 测试
    #[cfg(unix)]
    {
        let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
        let mut unix_times = Vec::new();
        let mut unix_success_count = 0;
        
        if socket_path.exists() {
            println!("🔌 Testing Unix Socket RPC...");
            for i in 0..iterations {
                match test_unix_socket_rpc(&socket_path, &request).await {
                    Ok((response, elapsed)) => {
                        unix_times.push(elapsed);
                        unix_success_count += 1;
                        if i == 0 {
                            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                                if let Some(result) = json_response.get("result") {
                                    println!("   ✅ Response: {}", serde_json::to_string_pretty(result)?);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if i == 0 {
                            println!("   ❌ Unix Socket request failed: {}", e);
                        }
                    }
                }
            }
            
            // 打印统计结果
            println!("\n📊 Performance Comparison:");
            println!("{}", "-".repeat(60));
            
            if http_success_count > 0 {
                let http_avg = http_times.iter().sum::<u128>() as f64 / http_times.len() as f64;
                let http_min = *http_times.iter().min().unwrap();
                let http_max = *http_times.iter().max().unwrap();
                
                println!("HTTP        | Avg: {:>8.1}μs | Min: {:>8}μs | Max: {:>8}μs | Success: {}/{}", 
                    http_avg, http_min, http_max, http_success_count, iterations);
            } else {
                println!("HTTP        | ❌ All requests failed");
            }
            
            if unix_success_count > 0 {
                let unix_avg = unix_times.iter().sum::<u128>() as f64 / unix_times.len() as f64;
                let unix_min = *unix_times.iter().min().unwrap();
                let unix_max = *unix_times.iter().max().unwrap();
                
                println!("Unix Socket | Avg: {:>8.1}μs | Min: {:>8}μs | Max: {:>8}μs | Success: {}/{}", 
                    unix_avg, unix_min, unix_max, unix_success_count, iterations);
                
                // 计算性能提升
                if http_success_count > 0 {
                    let http_avg = http_times.iter().sum::<u128>() as f64 / http_times.len() as f64;
                    let speedup = http_avg / unix_avg;
                    let improvement = ((http_avg - unix_avg) / http_avg) * 100.0;
                    
                    println!("{}", "-".repeat(60));
                    if speedup > 1.0 {
                        println!("🏆 Unix Socket is {:.2}x faster ({:.1}% improvement)", speedup, improvement);
                    } else {
                        println!("📈 HTTP is {:.2}x faster ({:.1}% better)", 1.0/speedup, -improvement);
                    }
                }
            } else {
                println!("Unix Socket | ❌ All requests failed");
            }
        } else {
            println!("⚠️  Unix Socket not available at: {:?}", socket_path);
            println!("   Make sure the RPC server is running with Unix Socket enabled");
        }
    }
    
    #[cfg(not(unix))]
    {
        println!("⚠️  Unix Socket testing is only available on Unix-like systems");
        if http_success_count > 0 {
            let http_avg = http_times.iter().sum::<u128>() as f64 / http_times.len() as f64;
            let http_min = *http_times.iter().min().unwrap();
            let http_max = *http_times.iter().max().unwrap();
            
            println!("\n📊 HTTP Performance:");
            println!("Average: {:.1}μs | Min: {}μs | Max: {}μs | Success: {}/{}", 
                http_avg, http_min, http_max, http_success_count, iterations);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Solana RPC Performance Comparison: HTTP vs Unix Socket");
    println!("{}", "=".repeat(60));
    
    // 等待服务可用
    println!("⏳ Waiting for RPC services to be available...");
    sleep(Duration::from_millis(500)).await;
    
    // 测试不同的 RPC 方法
    let test_cases = vec![
        ("getHealth", json!([])),
        ("getVersion", json!([])),
        ("getSlot", json!([])),
    ];
    
    let iterations = 100; // 测试次数
    
    for (method, params) in test_cases {
        run_performance_test(method, params, iterations).await?;
    }
    
    println!("\n✨ Performance testing completed!");
    
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    println!("Unix socket example is only available on Unix-like systems");
} 