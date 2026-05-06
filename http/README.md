gostd_http  等价于 gostd::net::http

`gostd_http` 是 Go 标准库 `net/http` 的 Rust 仿真实现，旨在为 Rust 开发者提供与 Go 语言一致的 HTTP 客户端开发体验，同时利用 Rust 的所有权模型和并发原语进行了深度优化。

# 核心特性

- **高性能连接池 (Connection Pooling)**：
    - **Actor 模型架构**：底层连接管理采用基于消息传递的 Actor 模型，消除了高并发下的锁竞争瓶颈。
    - **自动复用 (Keep-Alive)**：支持对 TCP 和 TLS 连接的持久化存储与自动复用。
    - **HTTPS 深度优化**：支持复用完整的 TLS 会话，大幅减少 HTTPS 请求中的重复握手开销。
- **智能化生命周期管理**：
    - **空闲超时清理**：后台自动扫描并清理超过 90 秒未使用的闲置连接。
    - **存活探测 (Health Check)**：在连接重用前进行主动探测，确保返回给用户的始终是可用的活跃连接。
- **极致性能设计**：
    - **零分配 Cookie 校验**：使用静态查找表加速字段合法性校验，彻底消除堆内存分配。
    - **零拷贝头部读取**：`Header::Get` 直接返回借用（`&str`），显著降低内存克隆压力。
- **健壮性与可观测性**：
    - **强类型错误处理**：系统性移除 `unwrap`，通过精细化的错误枚举处理各种协议异常。
    - **复用状态感知**：`Response` 结构体新增 `reused` 字段，方便开发者监控连接复用效率。
- **高度仿真 API**：完美适配 Go 语言 `http.Client`、`http.Request` 的接口习惯。

# 使用例子

## http模块

### Async 异步http
  默认不启用异步方式

    features =["async-std-runtime"] // 使用async_std 异步运行时
    或者 features =["tokio-runtime"] // 使用 tokio 异步运行时

#### 使用async_std

 Cargo.toml配置：

    async-std = {version = "1.13" ,features = ["attributes"]}
    gostd = { version = "0.4" ,features =["async-std-rt"]}
    或者 gostd_http = { version = "1" ,features =["async-std-runtime"]}

1. POST

```rust

use gostd::net::http::async_http; 
// 或者用 use gostd_http::async_http

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet";
    let postbody = r#"{"id":0,"category":{"id":0,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":0,"name":"string"}],"status":"available"}"#
   .as_bytes()
   .to_vec();
    let response = async_http::Post(url, "application/json", Some(postbody.into())).await?;

    println!(
        "{}",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap()
    );

    Ok(())
}

```
或者 

```rust
use gostd::net::http::{async_http::AsyncClient, Method, Request};
// 或者用 use gostd_http::{async_http::AsyncClient, Method, Request};
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet";

    let postbody = r#"{
      "id": 0,
      "category": {
        "id": 0,
        "name": "string"
      },
      "name": "doggie",
      "photoUrls": [
        "string"
      ],
      "tags": [
        {
          "id": 0,
          "name": "string"
        }
      ],
      "status": "available"
    }"#
    .as_bytes()
    .to_vec();

    let mut req = Request::New(Method::Post, url, Some(postbody.into()))?;

    req.Header.Set("accept", "application/json");
    req.Header.Set("Content-Type", "application/json");
    let mut client = AsyncClient::New();
    let response = client.Do(&mut req).await?;

    println!(
        "{} (connection reused: {})",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap(),
        response.reused
    );

    Ok(())
}

```

2. GET

```rust
use gostd::net::http::async_http;
// 或者用 use gostd_http::async_http;
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let response = async_http::Get(url).await?;

    println!(
        "{}",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap()
    );

    Ok(())
}

``` 

#### 使用tokio

 Cargo.toml配置：

    tokio = { version = "1.44", features = ["full"] }
    gostd = { version = "0.4" ,features =["tokio-rt"]}
    或者 gostd_http = { version = "1" ,features =["tokio-runtime"]}

1. POST

```rust

use gostd::net::http::async_http; 
// 或者用 use gostd_http::async_http;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet";
    let postbody = r#"{"id":0,"category":{"id":0,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":0,"name":"string"}],"status":"available"}"#
   .as_bytes()
   .to_vec();
    let response = async_http::Post(url, "application/json", Some(postbody.into())).await?;

    println!(
        "{}",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap()
    );

    Ok(())
}

```

### Sync 同步http

### client客户端

1. POST

```rust

use gostd::net::http;
fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet";
    let postbody = r#"{"id":0,"category":{"id":0,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":0,"name":"string"}],"status":"available"}"#
   .as_bytes()
   .to_vec();
    let response = http::Post(url, "application/json", Some(postbody))?;

    println!(
        "{}",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap()
    );

    Ok(())
}

```

2. GET

```rust
use gostd::net::http;

fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let response = http::Get(url)?;

    println!(
        "{} (reused: {})",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap(),
        response.reused
    );

    Ok(())
}

``` 

## multipart模块

### form-data Body (文本字段)

```rust
use gostd::bytes;
use gostd::mime::multipart::Writer;
use gostd::net::http::{Client, Method, Request};
fn main() -> Result<(), std::io::Error> {
    let mut body = bytes::Buffer::new();
    let mut w = Writer::new(&mut body);
    w.WriteField("requestId", "12121231231")?;
    w.WriteField("name", "刘xxx")?;
    w.Close()?;
    let contentType = w.FormDataContentType();
    let url = "http://www.baidu.com";
    let mut req = Request::New(Method::Post, url, Some(body.Bytes().into()))?;
    req.Header.Set("Content-Type", contentType.as_str());
    let mut client = Client::New();
    let response = client.Do(&mut req)?;

    println!(
        "{}",
        String::from_utf8(response.Body.unwrap().to_vec()).unwrap()
    );

    Ok(())
}
```

### form-data Body (带文件附件)

```rust
use gostd::bytes;
use gostd::mime::multipart::Writer;
use gostd::net::http::{Client, Method, Request};
use std::fs;

fn main() -> anyhow::Result<()> {
    let mut body = bytes::Buffer::new();
    let mut w = Writer::new(&mut body);

    // 1. 添加普通文本字段
    w.WriteField("title", "测试上传附件")?;

    // 2. 创建文件表单部分
    // CreateFormFile 返回底层 buffer 的可变引用，可直接写入文件字节
    let mut file_part = w.CreateFormFile("file_field_name", "example.txt")?;

    // 3. 读取本地文件并写入
    let content = fs::read("test_file.txt")?; 
    file_part.Write(content)?;

    // 4. 关闭 Writer 以写入最后的 boundary 边界
    w.Close()?;

    let contentType = w.FormDataContentType();
    let url = "http://example.com/api/upload";

    let mut req = Request::New(Method::Post, url, Some(body.Bytes().into()))?;
    req.Header.Set("Content-Type", contentType.as_str());

    let mut client = Client::New();
    let response = client.Do(&mut req)?;

    println!("Response status: {}", response.Status);
    Ok(())
}
```

