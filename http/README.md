
 gostd_http  等价于 gostd::net::http

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
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
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
        "{}",
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
    );

    Ok(())
}
// output
// {"id":92233723685477587,"category":{"id":,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":,"name":"string"}],"status":"available"}

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
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
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
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let mut req = Request::New(Method::Get, url, None)?;
    req.Header.Set("Content-Type", "application/json");

    let mut client = AsyncClient::New();

    let response = client.Do(&mut req).await?;
    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
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
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
    );

    Ok(())
}

```
或者 

```rust
use gostd::net::http::{async_http::AsyncClient, Method, Request};
// 或者用 use gostd_http::{async_http::AsyncClient, Method, Request};
#[tokio::main]
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
        "{}",
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
    );

    Ok(())
}
// output
// {"id":92233723685477587,"category":{"id":,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":,"name":"string"}],"status":"available"}

```

2. GET

```rust
use gostd::net::http::async_http;
// 或者用 use gostd_http::async_http;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let response = async_http::Get(url).await?;

    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
    );

    Ok(())
}

``` 
或者 

```rust
use gostd::net::http::{async_http::AsyncClient, Method, Request};
// 或者用 use gostd_http::{async_http::AsyncClient, Method, Request};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let mut req = Request::New(Method::Get, url, None)?;
    req.Header.Set("Content-Type", "application/json");

    let mut client = AsyncClient::New();

    let response = client.Do(&mut req).await?;
    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error").to_vec()).unwrap()
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
        String::from_utf8(response.Body.expect("return body error")).unwrap()
    );

    Ok(())
}

```
或者 

```rust
use gostd::net::http::{Client, Method, Request};

fn main() -> anyhow::Result<()> {

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

    let mut req = Request::New(Method::Post, url, Some(postbody))?;

    req.Header.Set("accept", "application/json");
    req.Header.Set("Content-Type", "application/json");
    let mut client = Client::New();
    let response = client.Do(&mut req)?;

    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error")).unwrap()
    );

    Ok(())
}

// output
// {"id":92233723685477587,"category":{"id":,"name":"string"},"name":"doggie","photoUrls":["string"],"tags":[{"id":,"name":"string"}],"status":"available"}

```

2. GET

```rust
use gostd::net::http;

fn main() -> anyhow::Result<()> {
    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let response = http::Get(url)?;

    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error")).unwrap()
    );

    Ok(())
}

``` 
或者 

```rust
use gostd::net::http::{Client, Method, Request};

fn main() -> anyhow::Result<()> {

    let url = "https://petstore.swagger.io/v2/pet/findByStatus?status=available";
    let mut req = Request::New(Method::Get, url, None)?;
    req.Header.Set("Content-Type", "application/json");

    let mut client = Client::New();

    let response = client.Do(&mut req)?;
    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error")).unwrap()
    );

    Ok(())
}

```
## multipart模块

### form-data Body

```rust
use gostd::bytes;
use gostd::mime::multipart::Writer;
use gostd::net::http::{Client, Method, Request};
fn main() -> Result<(), std::io::Error> {
    let mut body = bytes::Buffer::new();
    let mut w = Writer::new(&mut body);
    w.WriteField("requestId", "12121231231")?;
    w.WriteField("testTime", "2022-01-22 18:00:00")?;
    w.WriteField("checkTime", "2022-01-22 22:00:00")?;
    w.WriteField("auditTime", "2022-01-22 23:00:00")?;
    w.WriteField("tubeCode", "QCGD99SDF")?;
    w.WriteField("testRatio", "1")?;
    w.WriteField("name", "刘xxx")?;
    w.WriteField("sex", "1")?;
    w.WriteField("birthdate", "20003-07-02")?;
    w.WriteField("address", "北京市丰台区")?;
    w.WriteField("phoneNumber", "1881xxxx")?;
    w.WriteField("cardType", "身份证")?;
    w.WriteField("cardNumber", "xxxx")?;
    w.WriteField("testResult", "0")?;
    w.WriteField("testUserName", "xxx")?;
    w.WriteField("checkUserName", "xxx")?;
    w.Close()?;
    let contentType = w.FormDataContentType();
    let url = "http://www.baidu.com";
    let mut req = Request::New(Method::Post, url, Some(body.Bytes()))?;
    req.Header.Set("Content-Type", contentType.as_str());
    let mut client = Client::New();
    let response = client.Do(&mut req)?;

    println!(
        "{}",
        String::from_utf8(response.Body.expect("return body error")).unwrap()
    );

    Ok(())
}
```


