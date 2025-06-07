use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

fn main() {
    let socket_path = "/tmp/solana-rpc.sock";
    
    println!("Testing getBalance and getAccountInfo via Unix Socket");
    println!("Socket path: {}", socket_path);
    println!("{}", "=".repeat(60));
    
    // Test getBalance
    let balance_request = r#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",{"encoding":"base64"}]}"#;
    
    println!("\n🔍 Testing getBalance...");
    match test_request(socket_path, balance_request, "getBalance") {
        Ok(duration) => {
            println!("✅ getBalance successful in {:.2}μs", duration.as_micros());
        }
        Err(e) => {
            println!("❌ getBalance failed: {}", e);
        }
    }
    
    // Test getAccountInfo
    let account_request = r#"{"jsonrpc":"2.0","id":2,"method":"getAccountInfo","params":["5BJpnniJB9rGPYXrv3k3RvdcUbPbqkbkXNWUefCkwKi",{"encoding":"base64"}]}"#;
    
    println!("\n🔍 Testing getAccountInfo...");
    match test_request(socket_path, account_request, "getAccountInfo") {
        Ok(duration) => {
            println!("✅ getAccountInfo successful in {:.2}μs", duration.as_micros());
        }
        Err(e) => {
            println!("❌ getAccountInfo failed: {}", e);
        }
    }
    
    // Test multiple requests
    println!("\n📊 Testing 10 sequential requests of each type...");
    
    let mut balance_times = Vec::new();
    let mut account_times = Vec::new();
    
    for i in 0..10 {
        // Test getBalance
        match test_request(socket_path, balance_request, "getBalance") {
            Ok(duration) => {
                balance_times.push(duration);
                println!("getBalance #{}: {:.2}μs", i + 1, duration.as_micros());
            }
            Err(e) => {
                println!("getBalance #{} failed: {}", i + 1, e);
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
        
        // Test getAccountInfo
        match test_request(socket_path, account_request, "getAccountInfo") {
            Ok(duration) => {
                account_times.push(duration);
                println!("getAccountInfo #{}: {:.2}μs", i + 1, duration.as_micros());
            }
            Err(e) => {
                println!("getAccountInfo #{} failed: {}", i + 1, e);
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    // Calculate statistics
    println!("\n📈 Performance Summary:");
    if !balance_times.is_empty() {
        let avg_balance = balance_times.iter().map(|d| d.as_micros()).sum::<u128>() as f64 / balance_times.len() as f64;
        let min_balance = balance_times.iter().min().unwrap().as_micros();
        let max_balance = balance_times.iter().max().unwrap().as_micros();
        println!("getBalance:     Avg: {:.1}μs, Min: {}μs, Max: {}μs, Success: {}/10", 
                 avg_balance, min_balance, max_balance, balance_times.len());
    }
    
    if !account_times.is_empty() {
        let avg_account = account_times.iter().map(|d| d.as_micros()).sum::<u128>() as f64 / account_times.len() as f64;
        let min_account = account_times.iter().min().unwrap().as_micros();
        let max_account = account_times.iter().max().unwrap().as_micros();
        println!("getAccountInfo: Avg: {:.1}μs, Min: {}μs, Max: {}μs, Success: {}/10", 
                 avg_account, min_account, max_account, account_times.len());
    }
}

fn test_request(socket_path: &str, request: &str, method_name: &str) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Connect to Unix socket
    let mut stream = UnixStream::connect(socket_path)?;
    stream.set_read_timeout(Some(Duration::from_millis(2000)))?;
    stream.set_write_timeout(Some(Duration::from_millis(2000)))?;
    
    // Send request
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    
    // Read response
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    let bytes_read = reader.read_line(&mut response)?;
    
    let duration = start.elapsed();
    
    if bytes_read == 0 || response.trim().is_empty() {
        return Err(format!("Empty response for {} (bytes_read: {})", method_name, bytes_read).into());
    }
    
    let response = response.trim();
    
    // Parse and validate JSON response
    if let Ok(json_response) = serde_json::from_str::<serde_json::Value>(&response) {
        if let Some(error) = json_response.get("error") {
            println!("   ⚠️  RPC Error: {}", error);
        } else if let Some(result) = json_response.get("result") {
            // Show a brief summary of the result
            match method_name {
                "getBalance" => {
                    if let Some(value) = result.get("value") {
                        println!("   💰 Balance: {} lamports", value);
                    }
                }
                "getAccountInfo" => {
                    if let Some(value) = result.get("value") {
                        if value.is_null() {
                            println!("   📭 Account not found");
                        } else if let Some(lamports) = value.get("lamports") {
                            println!("   📄 Account found with {} lamports", lamports);
                        }
                    }
                }
                _ => {
                    println!("   ✅ Got result: {}", serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string()));
                }
            }
        }
    } else {
        return Err(format!("Invalid JSON response for {}: {}", method_name, response).into());
    }
    
    Ok(duration)
} 