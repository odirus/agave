use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let socket_path = "/tmp/solana-rpc.sock";
    let num_threads = 10; // 测试10个并发连接
    let requests_per_thread = 10;
    
    println!("Testing improved Unix Socket RPC with {} threads, {} requests each", 
             num_threads, requests_per_thread);
    
    let total_requests = Arc::new(AtomicU64::new(0));
    let total_time = Arc::new(AtomicU64::new(0));
    let errors = Arc::new(AtomicU64::new(0));
    
    let mut handles = Vec::new();
    
    let start_time = Instant::now();
    
    // 启动多个线程进行并发测试
    for thread_id in 0..num_threads {
        let socket_path = socket_path.to_string();
        let total_requests = Arc::clone(&total_requests);
        let total_time = Arc::clone(&total_time);
        let errors = Arc::clone(&errors);
        
        let handle = thread::spawn(move || {
            for i in 0..requests_per_thread {
                // Retry mechanism for failed requests
                let mut retry_count = 0;
                let max_retries = 0;
                
                loop {
                    match test_single_request(&socket_path, thread_id, i) {
                        Ok(duration) => {
                            total_requests.fetch_add(1, Ordering::Relaxed);
                            total_time.fetch_add(duration.as_micros() as u64, Ordering::Relaxed);
                            break;
                        }
                        Err(e) => {
                            retry_count += 1;
                            if retry_count > max_retries {
                                errors.fetch_add(1, Ordering::Relaxed);
                                eprintln!("Thread {}, Request {}: Final Error after {} retries: {}", thread_id, i, max_retries, e);
                                break;
                            } else {
                                // Wait a bit before retrying
                                thread::sleep(Duration::from_millis(10 * retry_count));
                            }
                        }
                    }
                }
                
                // 小间隔避免过载
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_duration = start_time.elapsed();
    let successful_requests = total_requests.load(Ordering::Relaxed);
    let total_time_micros = total_time.load(Ordering::Relaxed);
    let error_count = errors.load(Ordering::Relaxed);
    
    println!("\n=== 并发测试结果 ===");
    println!("总请求数: {}", num_threads * requests_per_thread);
    println!("成功请求: {}", successful_requests);
    println!("错误请求: {}", error_count);
    println!("总耗时: {:.2}s", total_duration.as_secs_f64());
    
    if successful_requests > 0 {
        let avg_latency = total_time_micros as f64 / successful_requests as f64;
        let requests_per_sec = successful_requests as f64 / total_duration.as_secs_f64();
        
        println!("平均延迟: {:.2}μs", avg_latency);
        println!("吞吐量: {:.2} requests/sec", requests_per_sec);
        println!("并发效率: {:.2}%", (successful_requests as f64 / (num_threads * requests_per_thread) as f64) * 100.0);
    }
}

fn test_single_request(socket_path: &str, thread_id: usize, request_id: usize) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // 连接到Unix socket，设置超时
    let mut stream = UnixStream::connect(socket_path)?;
    stream.set_read_timeout(Some(Duration::from_millis(1000)))?;
    stream.set_write_timeout(Some(Duration::from_millis(1000)))?;
    
    // 发送getHealth请求
    let request = format!(
        r#"{{"jsonrpc":"2.0","id":{},"method":"getHealth"}}"#,
        thread_id * 1000 + request_id
    );
    
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?; // 确保数据发送
    
    // 读取响应
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    
    // 尝试读取多次，处理可能的延迟
    let bytes_read = reader.read_line(&mut response)?;
    
    let duration = start.elapsed();
    
    // 验证响应
    if bytes_read == 0 || response.trim().is_empty() {
        return Err(format!("Empty response (bytes_read: {})", bytes_read).into());
    }
    
    let response = response.trim();
    
    // 简单验证JSON格式
    if !response.contains("jsonrpc") && !response.contains("result") {
        return Err(format!("Invalid response format: {}", response).into());
    }
    
    Ok(duration)
} 