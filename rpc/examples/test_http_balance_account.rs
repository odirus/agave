use std::time::{Duration, Instant};
use reqwest;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "http://127.0.0.1:8899";
    
    println!("Testing getBalance and getAccountInfo via HTTP");
    println!("RPC URL: {}", rpc_url);
    println!("{}", "=".repeat(60));
    
    // Create HTTP client with keep-alive
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .tcp_keepalive(Duration::from_secs(30))
        .build()?;
    
    // Test getBalance
    let balance_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [
            "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
            {
                "encoding": "base64"
            }
        ]
    });
    
    println!("\n🔍 Testing getBalance...");
    match test_http_request(&client, rpc_url, &balance_request, "getBalance").await {
        Ok(duration) => {
            println!("✅ getBalance successful in {:.2}μs", duration.as_micros());
        }
        Err(e) => {
            println!("❌ getBalance failed: {}", e);
        }
    }
    
    // Test getAccountInfo
    let account_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "getAccountInfo",
        "params": [
            "5BJpnniJB9rGPYXrv3k3RvdcUbPbqkbkXNWUefCkwKi",
            {
                "encoding": "base64"
            }
        ]
    });
    
    println!("\n🔍 Testing getAccountInfo...");
    match test_http_request(&client, rpc_url, &account_request, "getAccountInfo").await {
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
        match test_http_request(&client, rpc_url, &balance_request, "getBalance").await {
            Ok(duration) => {
                balance_times.push(duration);
                println!("getBalance #{}: {:.2}μs", i + 1, duration.as_micros());
            }
            Err(e) => {
                println!("getBalance #{} failed: {}", i + 1, e);
            }
        }
        
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Test getAccountInfo
        match test_http_request(&client, rpc_url, &account_request, "getAccountInfo").await {
            Ok(duration) => {
                account_times.push(duration);
                println!("getAccountInfo #{}: {:.2}μs", i + 1, duration.as_micros());
            }
            Err(e) => {
                println!("getAccountInfo #{} failed: {}", i + 1, e);
            }
        }
        
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Calculate statistics
    println!("\n📈 HTTP Performance Summary:");
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
    
    Ok(())
}

async fn test_http_request(
    client: &reqwest::Client,
    url: &str,
    request: &Value,
    method_name: &str,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?;
    
    let response_text = response.text().await?;
    let duration = start.elapsed();
    
    // Parse and validate JSON response
    if let Ok(json_response) = serde_json::from_str::<Value>(&response_text) {
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
        return Err(format!("Invalid JSON response for {}: {}", method_name, response_text).into());
    }
    
    Ok(duration)
} 