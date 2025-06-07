//! Native Unix Socket vs HTTP Performance Benchmark
//! 
//! This benchmark tests the performance difference between:
//! 1. HTTP over TCP (127.0.0.1:8899)
//! 2. Native Unix Socket JSON-RPC (/tmp/solana-rpc.sock)

use serde_json::{json, Value};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

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

// 1. Native Unix Socket test - 直接发送JSON-RPC，无HTTP包装
fn test_native_unix_socket(socket_path: &Path, request: &str) -> Result<(String, u128), Box<dyn std::error::Error>> {
    use std::io::BufRead;
    use std::io::BufReader;
    
    let start_time = get_time_micros();
    
    // 直接连接Unix Socket
    let mut stream = UnixStream::connect(socket_path)?;
    
    // 发送原始JSON-RPC数据（无HTTP头）
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?; // 添加换行符作为消息分隔符
    
    // 读取一行响应（服务器以换行符结束响应）
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    
    let elapsed = get_time_micros() - start_time;
    
    Ok((response.trim().to_string(), elapsed))
}

// 2. HTTP over TCP test
async fn test_http_tcp(client: &hyper::Client<hyper::client::HttpConnector>, url: &str, request: &Value) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let start_time = get_time_micros();
    
    let req = hyper::Request::builder()
        .method("POST")
        .uri(url)
        .header("Content-Type", "application/json")
        .header("Connection", "keep-alive")
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

async fn run_comprehensive_benchmark(method: &str, params: Value, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 Comprehensive RPC Benchmark: {} ({} iterations)", method, iterations);
    println!("{}", "=".repeat(80));
    
    let request = make_rpc_request(method, params).await;
    let request_str = request.to_string();
    
    // 创建HTTP客户端
    let http_client = hyper::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build_http();
    
    // 测试路径
    let http_url = "http://127.0.0.1:8899";
    let native_unix_socket = Path::new("/tmp/solana-rpc.sock");
    
    // 结果存储
    let mut http_tcp_times = Vec::new();
    let mut native_unix_times = Vec::new();
    
    let mut http_tcp_success = 0;
    let mut native_unix_success = 0;
    
    println!("📡 Testing HTTP over TCP (127.0.0.1:8899)...");
    // 预热
    for _ in 0..5 {
        let _ = test_http_tcp(&http_client, http_url, &request).await;
    }
    
    // 正式测试
    for i in 0..iterations {
        match test_http_tcp(&http_client, http_url, &request).await {
            Ok((response, elapsed)) => {
                http_tcp_times.push(elapsed);
                http_tcp_success += 1;
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
                    println!("   ❌ HTTP TCP failed: {}", e);
                }
            }
        }
        
        if i % 50 == 0 && i > 0 {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
    

    
    if native_unix_socket.exists() {
        println!("⚡ Testing Native Unix Socket (/tmp/solana-rpc.sock)...");
        
        // 正式测试（原生Unix Socket不需要太多预热）
        for i in 0..iterations {
            match test_native_unix_socket(native_unix_socket, &request_str) {
                Ok((response, elapsed)) => {
                    native_unix_times.push(elapsed);
                    native_unix_success += 1;
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
                        println!("   ❌ Native Unix Socket failed: {}", e);
                    }
                }
            }
            
            if i % 50 == 0 && i > 0 {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    } else {
        println!("⚠️  Native Unix Socket not available at: {:?}", native_unix_socket);
    }
    
    // 打印详细统计结果
    println!("\n📊 Comprehensive Performance Analysis:");
    println!("{}", "-".repeat(80));
    
    if http_tcp_success > 0 {
        let (avg, min, max, std, median) = calculate_stats(&http_tcp_times);
        println!("HTTP/TCP     | Avg: {:>7.1}μs | Med: {:>7.1}μs | Min: {:>6}μs | Max: {:>6}μs | Std: {:>6.1}μs | {}/{}", 
            avg, median, min, max, std, http_tcp_success, iterations);
    } else {
        println!("HTTP/TCP     | ❌ All requests failed");
    }
    
    if native_unix_success > 0 {
        let (avg, min, max, std, median) = calculate_stats(&native_unix_times);
        println!("Native/Unix  | Avg: {:>7.1}μs | Med: {:>7.1}μs | Min: {:>6}μs | Max: {:>6}μs | Std: {:>6.1}μs | {}/{}", 
            avg, median, min, max, std, native_unix_success, iterations);
    } else {
        println!("Native/Unix  | ❌ All requests failed");
    }
    
    // 性能比较
    if http_tcp_success > 0 && native_unix_success > 0 {
        let (http_tcp_avg, _, _, _, _) = calculate_stats(&http_tcp_times);
        let (native_unix_avg, _, _, _, _) = calculate_stats(&native_unix_times);
        
        println!("{}", "-".repeat(80));
        let speedup = http_tcp_avg / native_unix_avg;
        let improvement = ((http_tcp_avg - native_unix_avg) / http_tcp_avg) * 100.0;
        
        if speedup > 1.1 {
            println!("🏆 Native Unix Socket is {:.2}x faster than HTTP/TCP ({:.1}% improvement)", speedup, improvement);
        } else if speedup < 0.9 {
            println!("📈 HTTP/TCP is {:.2}x faster than Native Unix Socket ({:.1}% better)", 1.0/speedup, -improvement);
        } else {
            println!("⚖️  Native Unix Socket and HTTP/TCP have similar performance (difference: {:.1}%)", improvement.abs());
        }
    }
    
    // 百分位数分析
    if native_unix_success > 10 && http_tcp_success > 10 {
        println!("\n📈 Percentile Analysis:");
        println!("             | P50     | P95     | P99");
        
        if http_tcp_success > 10 {
            let mut sorted = http_tcp_times.clone();
            sorted.sort();
            let p50 = sorted[sorted.len() * 50 / 100];
            let p95 = sorted[sorted.len() * 95 / 100];
            let p99 = sorted[sorted.len() * 99 / 100];
            println!("HTTP/TCP     | {:>6}μs | {:>6}μs | {:>6}μs", p50, p95, p99);
        }
        
        if native_unix_success > 10 {
            let mut sorted = native_unix_times.clone();
            sorted.sort();
            let p50 = sorted[sorted.len() * 50 / 100];
            let p95 = sorted[sorted.len() * 95 / 100];
            let p99 = sorted[sorted.len() * 99 / 100];
            println!("Native/Unix  | {:>6}μs | {:>6}μs | {:>6}μs", p50, p95, p99);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Native Unix Socket Performance Benchmark");
    println!("Testing: HTTP/TCP vs Native/Unix");
    println!("{}", "=".repeat(80));
    
    // 等待服务可用
    println!("⏳ Waiting for RPC services to be available...");
    sleep(Duration::from_millis(1000)).await;
    
    // 测试不同的 RPC 方法
    let test_cases = vec![
        ("getBalance", json!([
            "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
            {
                "encoding": "base64"
            }
        ])),
        ("getAccountInfo", json!([
            "5BJpnniJB9rGPYXrv3k3RvdcUbPbqkbkXNWUefCkwKi",
            {
                "encoding": "base64"
            }
        ])),
    ];
    
    let iterations = 100; // 增加测试次数以获得更准确的统计
    
    for (method, params) in test_cases {
        run_comprehensive_benchmark(method, params, iterations).await?;
    }
    
    println!("\n✨ Comprehensive performance testing completed!");
    println!("\n💡 Performance Expectations:");
    println!("   • Native Unix Socket should be 2-5x faster than HTTP/TCP");
    println!("   • Benefits: Skip HTTP parsing, avoid TCP/IP stack, direct IPC");
    
    println!("\n🎯 If Native Unix Socket isn't significantly faster:");
    println!("   • Check if both servers are actually running");
    println!("   • Verify that /tmp/solana-rpc.sock exists");
    println!("   • Consider connection establishment overhead");
    println!("   • JSON parsing might be the bottleneck, not transport");
    
    Ok(())
} 