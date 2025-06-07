# Solana RPC 性能测试示例

这个目录包含了用于测试 Solana RPC 性能的示例代码，特别是对比 HTTP 和 Unix Socket 两种连接方式的性能差异。

## unix_socket_rpc_example.rs

这个示例会同时测试 HTTP 和 Unix Socket 连接到 Solana RPC 服务器的性能，并提供详细的性能对比报告。

### 功能特性

- ✅ 同时测试 HTTP 和 Unix Socket 连接
- ✅ 多种 RPC 方法测试 (getHealth, getVersion, getSlot)
- ✅ 多次迭代测试以获得可靠的性能数据
- ✅ 详细的统计信息 (平均值、最小值、最大值)
- ✅ 性能提升计算和对比
- ✅ 兼容 macOS 和 Linux

### 前置条件

1. **启动 RPC 服务器** - 确保你有一个运行中的 Solana RPC 服务器，它同时支持 HTTP 和 Unix Socket：

```bash
# 使用新的 new_with_both() 方法同时启动 HTTP 和 Unix Socket
# HTTP: http://127.0.0.1:8899
# Unix Socket: /tmp/solana-rpc.sock
```

2. **安装依赖** - 确保系统上已安装必要的依赖：

```bash
cargo build --example unix_socket_rpc_example
```

### 使用方法

```bash
# 编译示例
cargo build --example unix_socket_rpc_example

# 运行性能测试
cargo run --example unix_socket_rpc_example
```

### 示例输出

```
🔥 Solana RPC Performance Comparison: HTTP vs Unix Socket
============================================================
⏳ Waiting for RPC services to be available...

🚀 Testing RPC method: getHealth (100 iterations)
============================================================
📡 Testing HTTP RPC...
   ✅ Response: "ok"
🔌 Testing Unix Socket RPC...
   ✅ Response: "ok"

📊 Performance Comparison:
------------------------------------------------------------
HTTP        | Avg:   1250.3μs | Min:      890μs | Max:     2100μs | Success: 100/100
Unix Socket | Avg:    320.7μs | Min:      280μs | Max:      450μs | Success: 100/100
------------------------------------------------------------
🏆 Unix Socket is 3.90x faster (74.3% improvement)

🚀 Testing RPC method: getVersion (100 iterations)
============================================================
📡 Testing HTTP RPC...
   ✅ Response: {
     "feature-set": 4206508270,
     "solana-core": "2.2.15"
   }
🔌 Testing Unix Socket RPC...
   ✅ Response: {
     "feature-set": 4206508270,
     "solana-core": "2.2.15"
   }

📊 Performance Comparison:
------------------------------------------------------------
HTTP        | Avg:   1180.5μs | Min:      920μs | Max:     1800μs | Success: 100/100
Unix Socket | Avg:    310.2μs | Min:      270μs | Max:      420μs | Success: 100/100
------------------------------------------------------------
🏆 Unix Socket is 3.81x faster (73.7% improvement)

✨ Performance testing completed!
```

### 配置选项

你可以修改示例代码中的以下参数：

- `iterations`: 每个测试的迭代次数 (默认: 100)
- `http_url`: HTTP RPC 服务器地址 (默认: "http://127.0.0.1:8899")
- `socket_path`: Unix Socket 路径 (默认: "/tmp/solana-rpc.sock")
- `test_cases`: 要测试的 RPC 方法列表

### 性能分析

根据测试结果，Unix Socket 通常比 HTTP 连接有以下优势：

1. **更低的延迟** - 避免了 TCP/IP 协议栈的开销
2. **更高的吞吐量** - 减少了网络层的处理时间
3. **更少的系统调用** - 直接在内核空间进行数据传输
4. **更好的可预测性** - 避免了网络拥塞和路由影响

### 故障排除

**问题**: Unix Socket 测试失败
```
⚠️ Unix Socket not available at: "/tmp/solana-rpc.sock"
```

**解决方案**: 
1. 确保 RPC 服务器启用了 Unix Socket 支持
2. 检查 socket 文件权限 (应该是 660)
3. 确认 socket 路径正确

**问题**: HTTP 测试失败
```
❌ HTTP request failed: Connection refused
```

**解决方案**:
1. 确保 RPC 服务器在 127.0.0.1:8899 上运行
2. 检查防火墙设置
3. 确认端口没有被其他进程占用

### 兼容性

- ✅ macOS (Darwin)
- ✅ Linux
- ❌ Windows (Unix Socket 功能不可用，但 HTTP 测试仍然可用)

### 扩展

你可以扩展这个示例来测试：

- 更多的 RPC 方法
- 不同的负载模式
- 并发连接测试
- 内存使用分析
- 错误率统计 