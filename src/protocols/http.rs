pub async fn detect_http(packet: &[u8]) -> bool {
    if let Ok(packet_str) = std::str::from_utf8(packet) {
        if ["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH"]
            .iter()
            .any(|&method| packet_str.starts_with(method))
        {
            // println!("HTTP packet detected: {:?}", packet_str);
            if let Some(header_end) = packet_str.find("\r\n\r\n") {
                println!("Headers: {}", &packet_str[..header_end]);
                println!("Body: {}", &packet_str[header_end + 4..]);

                // Check for WebSocket upgrade request
                if packet_str.contains("Upgrade: websocket") {
                    println!("WebSocket upgrade request detected");
                    println!("message: {:?}", packet_str);
                }
            }
            return true;
        }
    }
    println!("Not an HTTP packet: {:?}", packet);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_detect_http() {
        let packet = b"GET / HTTP/1.1\r\n\r\n";
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            assert!(detect_http(&buffer[..n]).await);
            let response = b"HTTP/1.1 200 OK\r\n\r\n";
            socket.write_all(response).await.unwrap();
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(packet).await.unwrap();
        let mut response = vec![0; 1024];
        let _n = stream.read(&mut response).await.unwrap();
        assert!(response.starts_with(b"HTTP/1.1 200 OK"));
    }

    #[tokio::test]
    async fn test_detect_websocket() {
        let packet = b"GET / HTTP/1.1\r\nUpgrade: websocket\r\n\r\n";
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            assert!(detect_http(&buffer[..n]).await);
            let response = b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n";
            socket.write_all(response).await.unwrap();
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(packet).await.unwrap();
        let mut response = vec![0; 1024];
        let _n = stream.read(&mut response).await.unwrap();
        assert!(response.starts_with(b"HTTP/1.1 101 Switching Protocols"));
    }
}
