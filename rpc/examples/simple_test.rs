use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

fn main() {
    let socket_path = "/tmp/solana-rpc.sock";
    
    println!("Testing basic Unix Socket connection to: {}", socket_path);
    
    // Test single connection
    match test_connection(socket_path) {
        Ok(duration) => {
            println!("✅ Single request successful in {:.2}μs", duration.as_micros());
        }
        Err(e) => {
            println!("❌ Single request failed: {}", e);
            return;
        }
    }
    
    // Test multiple sequential requests
    println!("\nTesting 10 sequential requests...");
    let mut successful = 0;
    let mut total_time = Duration::new(0, 0);
    
    for i in 0..10 {
        match test_connection(socket_path) {
            Ok(duration) => {
                successful += 1;
                total_time += duration;
                println!("Request {}: {:.2}μs", i + 1, duration.as_micros());
            }
            Err(e) => {
                println!("Request {} failed: {}", i + 1, e);
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    
    println!("\n=== Results ===");
    println!("Successful: {}/10", successful);
    if successful > 0 {
        println!("Average latency: {:.2}μs", total_time.as_micros() / successful);
    }
}

fn test_connection(socket_path: &str) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Connect to Unix socket
    let mut stream = UnixStream::connect(socket_path)?;
    stream.set_read_timeout(Some(Duration::from_millis(2000)))?;
    stream.set_write_timeout(Some(Duration::from_millis(2000)))?;
    
    // Send getHealth request
    let request = r#"{"jsonrpc":"2.0","id":1,"method":"getHealth"}"#;
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    
    // Read response
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    let bytes_read = reader.read_line(&mut response)?;
    
    let duration = start.elapsed();
    
    if bytes_read == 0 || response.trim().is_empty() {
        return Err(format!("Empty response (bytes_read: {})", bytes_read).into());
    }
    
    let response = response.trim();
    println!("Response: {}", response);
    
    // Validate JSON format
    if !response.contains("jsonrpc") && !response.contains("result") {
        return Err(format!("Invalid response format: {}", response).into());
    }
    
    Ok(duration)
} 