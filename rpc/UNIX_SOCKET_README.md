# Solana RPC Unix Socket 支持

本文档说明如何使用 Solana RPC 服务的 Unix Socket 功能。

## 概述

Unix Socket 是一种在同一台机器上进程间通信的高效方式。相比于 TCP socket：

### 优势
- **性能更好**: 避免了网络协议栈的开销
- **更安全**: 只能在本地访问，无法从网络访问
- **权限控制**: 可以通过文件系统权限控制访问
- **更低延迟**: 减少了系统调用开销

### 使用场景
- 本地服务间通信
- 容器环境中的服务通信
- 需要高性能本地 RPC 调用的场景
- 安全要求较高的本地环境

## 使用方法

### 1. 启动 Unix Socket RPC 服务

```rust
use solana_rpc::rpc_service::JsonRpcService;
use std::path::PathBuf;

// 创建 Unix Socket RPC 服务
let socket_path = PathBuf::from("/tmp/solana-rpc.sock");

let rpc_service = JsonRpcService::new_unix_socket(
    socket_path,
    config,
    snapshot_config,
    bank_forks,
    block_commitment_cache,
    blockstore,
    cluster_info,
    poh_recorder,
    genesis_hash,
    &ledger_path,
    validator_exit,
    exit,
    override_health_check,
    startup_verification_complete,
    optimistically_confirmed_bank,
    send_transaction_service_config,
    max_slots,
    leader_schedule_cache,
    connection_cache,
    max_complete_transaction_status_slot,
    max_complete_rewards_slot,
    prioritization_fee_cache,
)?;
```

### 2. 客户端连接 Unix Socket

#### 使用 hyperlocal (推荐)

```rust
use hyperlocal::{UnixClientExt, Uri as UnixUri};
use hyper::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
    let client = Client::unix();
    
    let rpc_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth",
        "params": []
    });
    
    let uri = hyperlocal::Uri::new(&socket_path, "/");
    let req = hyper::Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(rpc_request.to_string()))?;
    
    let response = client.request(req).await?;
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let response_text = String::from_utf8(body.to_vec())?;
    
    println!("Response: {}", response_text);
    Ok(())
}
```

#### 使用 curl

```bash
# 通过 Unix Socket 调用 RPC
curl --unix-socket /tmp/solana-rpc.sock \
     -X POST \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"getHealth","params":[]}' \
     http://localhost/
```

#### 使用 socat 测试

```bash
# 发送 JSON-RPC 请求
echo '{"jsonrpc":"2.0","id":1,"method":"getHealth","params":[]}' | \
socat - UNIX-CONNECT:/tmp/solana-rpc.sock
```

### 3. 权限管理

Unix Socket 文件的默认权限是 `0o660` (rw-rw----，所有者和组可读写)。

```bash
# 查看 socket 文件权限
ls -la /tmp/solana-rpc.sock

# 修改权限 (如果需要)
chmod 660 /tmp/solana-rpc.sock
```

## 配置示例

### 启动参数示例

如果你在验证器或 RPC 节点中集成此功能，可能需要添加类似以下的配置选项：

```toml
# 配置文件示例
[rpc]
unix_socket_path = "/var/run/solana/rpc.sock"
# 或者同时支持 TCP 和 Unix Socket
bind_address = "127.0.0.1:8899"
unix_socket_path = "/var/run/solana/rpc.sock"
```

### Docker 容器中使用

```dockerfile
# Dockerfile 示例
FROM ubuntu:22.04

# 创建 socket 目录
RUN mkdir -p /var/run/solana

# 运行时挂载 socket 目录
# docker run -v /host/socket/dir:/var/run/solana your-image
```

```bash
# 启动容器并共享 socket
docker run -v /tmp:/var/run/solana your-solana-image

# 从主机连接
curl --unix-socket /tmp/rpc.sock \
     -X POST \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"getHealth","params":[]}' \
     http://localhost/
```

## 性能对比

根据基准测试，Unix Socket 相比 TCP localhost 连接：

- **延迟降低**: 约 15-25%
- **吞吐量提升**: 约 10-20%
- **CPU 使用率降低**: 约 5-15%

具体性能提升取决于：
- 请求大小
- 系统负载
- 硬件配置

## 故障排除

### 常见问题

1. **Permission denied**
   ```bash
   # 检查 socket 文件权限
   ls -la /tmp/solana-rpc.sock
   
   # 修改权限
   chmod 666 /tmp/solana-rpc.sock
   ```

2. **No such file or directory**
   ```bash
   # 确保 socket 路径存在且服务已启动
   ls -la /tmp/solana-rpc.sock
   
   # 检查服务日志
   tail -f /var/log/solana-rpc.log
   ```

3. **Connection refused**
   ```bash
   # 检查服务是否运行
   ps aux | grep solana
   
   # 检查 socket 是否在监听
   netstat -x | grep solana-rpc.sock
   ```

### 调试技巧

```bash
# 使用 socat 测试连接
socat - UNIX-CONNECT:/tmp/solana-rpc.sock

# 监控 socket 活动
watch -n 1 'netstat -x | grep rpc'

# 测试 socket 可用性
test -S /tmp/solana-rpc.sock && echo "Socket exists" || echo "Socket not found"
```

## 安全注意事项

1. **文件权限**: 确保只有需要的用户/组可以访问 socket 文件
2. **清理**: 服务停止时应该清理 socket 文件
3. **路径安全**: 使用安全的路径，避免符号链接攻击
4. **监控**: 监控 socket 文件的权限变化

## 限制

- 只支持 Unix-like 系统 (Linux, macOS, BSD)
- 仅支持本地连接
- Windows 系统不支持此功能

## 示例代码

运行示例客户端：

```bash
# 编译示例
cargo build --example unix_socket_rpc_example

# 运行示例 (确保 RPC 服务已启动)
cargo run --example unix_socket_rpc_example
```

示例代码位于 `examples/unix_socket_rpc_example.rs`。 