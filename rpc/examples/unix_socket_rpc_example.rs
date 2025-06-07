// HTTP vs Unix Socket RPC 性能对比示例
//
// 编译: cargo build --example unix_socket_rpc_example
// 运行: cargo run --example unix_socket_rpc_example

use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

#[cfg(unix)]
use {
    hyperlocal::{UnixClientExt, Uri},
    std::os::unix::fs::PermissionsExt,
};

fn get_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
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

// 优化的HTTP测试 - 支持连接复用
async fn test_http_rpc_optimized(client: &hyper::Client<hyper::client::HttpConnector>, url: &str, request: &Value) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let start_time = get_time_micros();
    
    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header("Content-Type", "application/json")
        .header("Connection", "keep-alive") // 启用连接复用
        .body(hyper::Body::from(request.to_string()))?;
    
    let response = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((body_str, elapsed))
}

// 优化的Unix Socket测试 - 支持连接复用
#[cfg(unix)]
async fn test_unix_socket_rpc_optimized(socket_path: &PathBuf, request: &Value) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let start_time = get_time_micros();
    
    let client = hyper::Client::unix();
    let uri = Uri::new(socket_path, "/");
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Connection", "keep-alive") // 启用连接复用
        .body(hyper::Body::from(request.to_string()))?;
    
    let response = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((body_str, elapsed))
}

// 计算统计信息
fn calculate_stats(times: &[u128]) -> (f64, u128, u128, f64, f64) {
    if times.is_empty() {
        return (0.0, 0, 0, 0.0, 0.0);
    }
    
    let mut sorted_times = times.to_vec();
    sorted_times.sort();
    
    let avg = times.iter().sum::<u128>() as f64 / times.len() as f64;
    let min = *sorted_times.first().unwrap();
    let max = *sorted_times.last().unwrap();
    
    // 计算标准差
    let variance = times.iter()
        .map(|x| (*x as f64 - avg).powi(2))
        .sum::<f64>() / times.len() as f64;
    let std_dev = variance.sqrt();
    
    // 计算中位数
    let median = if sorted_times.len() % 2 == 0 {
        (sorted_times[sorted_times.len() / 2 - 1] + sorted_times[sorted_times.len() / 2]) as f64 / 2.0
    } else {
        sorted_times[sorted_times.len() / 2] as f64
    };
    
    (avg, min, max, std_dev, median)
}

async fn run_performance_test(method: &str, params: Value, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 Testing RPC method: {} ({} iterations)", method, iterations);
    println!("{}", "=".repeat(70));
    
    let request = make_rpc_request(method, params).await;
    
    // 创建复用的HTTP客户端
    let http_client = hyper::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build_http();
    
    // HTTP 测试
    let http_url = "http://127.0.0.1:8899";
    println!("📡 Testing HTTP RPC with connection reuse...");
    
    // HTTP预热
    println!("   🔥 Warming up HTTP connection...");
    for _ in 0..5 {
        let _ = test_http_rpc_optimized(&http_client, http_url, &request).await;
    }
    
    let mut http_times = Vec::new();
    let mut http_success_count = 0;
    
    for i in 0..iterations {
        match test_http_rpc_optimized(&http_client, http_url, &request).await {
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
        
        // 添加小延迟避免过载
        if i % 50 == 0 && i > 0 {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
    
    // Unix Socket 测试
    #[cfg(unix)]
    {
        let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
        
        if socket_path.exists() {
            println!("🔌 Testing Unix Socket RPC...");
            
            // Unix Socket预热
            println!("   🔥 Warming up Unix Socket connection...");
            for _ in 0..5 {
                let _ = test_unix_socket_rpc_optimized(&socket_path, &request).await;
            }
            
            let mut unix_times = Vec::new();
            let mut unix_success_count = 0;
            
            for i in 0..iterations {
                match test_unix_socket_rpc_optimized(&socket_path, &request).await {
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
                
                // 添加小延迟避免过载
                if i % 50 == 0 && i > 0 {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            }
            
            // 打印详细统计结果
            println!("\n📊 Detailed Performance Analysis:");
            println!("{}", "-".repeat(70));
            
            if http_success_count > 0 {
                let (http_avg, http_min, http_max, http_std, http_median) = calculate_stats(&http_times);
                println!("HTTP         | Avg: {:>7.1}μs | Med: {:>7.1}μs | Min: {:>6}μs | Max: {:>6}μs | Std: {:>6.1}μs | {}/{}", 
                    http_avg, http_median, http_min, http_max, http_std, http_success_count, iterations);
            } else {
                println!("HTTP         | ❌ All requests failed");
            }
            
            if unix_success_count > 0 {
                let (unix_avg, unix_min, unix_max, unix_std, unix_median) = calculate_stats(&unix_times);
                println!("Unix Socket  | Avg: {:>7.1}μs | Med: {:>7.1}μs | Min: {:>6}μs | Max: {:>6}μs | Std: {:>6.1}μs | {}/{}", 
                    unix_avg, unix_median, unix_min, unix_max, unix_std, unix_success_count, iterations);
                
                // 计算性能对比
                if http_success_count > 0 {
                    let (http_avg, _, _, _, _) = calculate_stats(&http_times);
                    let speedup = http_avg / unix_avg;
                    let improvement = ((http_avg - unix_avg) / http_avg) * 100.0;
                    
                    println!("{}", "-".repeat(70));
                    if speedup > 1.05 { // 5%阈值
                        println!("🏆 Unix Socket is {:.2}x faster ({:.1}% improvement)", speedup, improvement);
                    } else if speedup < 0.95 {
                        println!("📈 HTTP is {:.2}x faster ({:.1}% better)", 1.0/speedup, -improvement);
                    } else {
                        println!("⚖️  Performance is similar (difference: {:.1}%)", improvement.abs());
                    }
                    
                    // 分析可能的原因
                    println!("\n🔍 Performance Analysis:");
                    if unix_avg > http_avg {
                        println!("   • Unix Socket slower than expected - possible causes:");
                        println!("     - HTTP-over-Unix-Socket has protocol overhead");
                        println!("     - Hyperlocal implementation overhead");
                        println!("     - System-level Unix Socket buffering");
                        println!("     - Additional abstraction layers");
                        println!("     - Kernel scheduling differences");
                        println!("     - Connection setup/teardown per request");
                    } else {
                        println!("   • Unix Socket shows expected performance benefits:");
                        println!("     - Avoiding TCP/IP stack overhead");
                        println!("     - Direct kernel communication");
                        println!("     - Reduced context switching");
                    }
                }
                
                // 百分位数分析
                if unix_times.len() > 10 && http_times.len() > 10 {
                    let mut sorted_unix = unix_times.clone();
                    let mut sorted_http = http_times.clone();
                    sorted_unix.sort();
                    sorted_http.sort();
                    
                    let p50_unix = sorted_unix[sorted_unix.len() * 50 / 100];
                    let p95_unix = sorted_unix[sorted_unix.len() * 95 / 100];
                    let p99_unix = sorted_unix[sorted_unix.len() * 99 / 100];
                    
                    let p50_http = sorted_http[sorted_http.len() * 50 / 100];
                    let p95_http = sorted_http[sorted_http.len() * 95 / 100];
                    let p99_http = sorted_http[sorted_http.len() * 99 / 100];
                    
                    println!("\n📈 Percentile Analysis:");
                    println!("             | P50     | P95     | P99");
                    println!("HTTP         | {:>6}μs | {:>6}μs | {:>6}μs", p50_http, p95_http, p99_http);
                    println!("Unix Socket  | {:>6}μs | {:>6}μs | {:>6}μs", p50_unix, p95_unix, p99_unix);
                }
                
            } else {
                println!("Unix Socket  | ❌ All requests failed");
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
            let (http_avg, http_min, http_max, http_std, http_median) = calculate_stats(&http_times);
            
            println!("\n📊 HTTP Performance:");
            println!("Average: {:.1}μs | Median: {:.1}μs | Min: {}μs | Max: {}μs | Std: {:.1}μs | Success: {}/{}", 
                http_avg, http_median, http_min, http_max, http_std, http_success_count, iterations);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Solana RPC Performance Comparison: HTTP vs Unix Socket (Optimized)");
    println!("{}", "=".repeat(70));
    
    // 等待服务可用
    println!("⏳ Waiting for RPC services to be available...");
    sleep(Duration::from_millis(1000)).await;
    
    // 测试不同的 RPC 方法
    let test_cases = vec![
        ("getHealth", json!([])),
        ("getVersion", json!([])),
        ("getSlot", json!([])),
    ];
    
    let iterations = 200; // 增加测试次数以获得更准确的统计
    
    for (method, params) in test_cases {
        run_performance_test(method, params, iterations).await?;
    }
    
    println!("\n✨ Performance testing completed!");
    println!("\n💡 Why Unix Socket might not be faster in this setup:");
    println!("   1. We're using HTTP-over-Unix-Socket (not raw Unix Socket)");
    println!("   2. Same JSON-RPC protocol overhead exists");
    println!("   3. Hyperlocal adds abstraction layers");
    println!("   4. TCP loopback (127.0.0.1) is highly optimized on modern systems");
    println!("   5. Connection pooling benefits HTTP more than Unix Socket");
    
    println!("\n🚀 For real Unix Socket performance gains:");
    println!("   • Use raw Unix Socket connections without HTTP");
    println!("   • Implement custom binary protocol");
    println!("   • Use MessagePack or Protocol Buffers");
    println!("   • Implement persistent connections");
    println!("   • Consider shared memory for large data");
    
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    println!("Unix socket example is only available on Unix-like systems");
} 