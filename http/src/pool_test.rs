#[cfg(test)]
#[cfg(feature = "tokio-runtime")]
mod tests {
    use crate::async_http::AsyncClient;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_connection_reuse() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let url = format!("http://{}", addr);

        // 启动一个简单的 mock 服务器
        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    // 处理第一个请求
                    let _ = socket.read(&mut buf).await;
                    let resp = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nhi";
                    socket.write_all(resp.as_bytes()).await.unwrap();
                    
                    // 保持连接，等待第二个请求（如果复用了）
                    let _ = socket.read(&mut buf).await;
                    socket.write_all(resp.as_bytes()).await.unwrap();
                });
            }
        });

        let mut client = AsyncClient::New();

        // 第一次请求
        let resp1 = client.Get(&url).await?;
        assert_eq!(resp1.reused, false);
        assert_eq!(resp1.Body.unwrap().to_vec(), b"hi");

        // 第二次请求 (应该复用)
        let resp2 = client.Get(&url).await?;
        assert_eq!(resp2.reused, true);
        assert_eq!(resp2.Body.unwrap().to_vec(), b"hi");

        Ok(())
    }
}
