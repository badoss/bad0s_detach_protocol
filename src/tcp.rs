use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub async fn handle_tcp(
    tcp_listener: TcpListener,
    tx: mpsc::Sender<(&'static str, std::net::SocketAddr, Vec<u8>)>,
) {
    loop {
        let (mut socket, addr) = tcp_listener.accept().await.unwrap();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                println!("Received TCP packet from {}: {:?}", addr, &buf[..n]);
                println!("Received port: {}", addr.port());
                println!("Received packet detail: {:?}", &buf[..n]);

                tx.send(("TCP", addr, buf[..n].to_vec())).await.unwrap();
            }
        });
    }
}

// pub async fn get_tcp_stream(addr: SocketAddr) -> Option<TcpStream> {
//     match TcpStream::connect(addr).await {
//         Ok(stream) => Some(stream),
//         Err(_) => None,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_handle_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(handle_tcp(listener, tx));

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

        let (protocol, _addr, packet) = rx.recv().await.unwrap();
        assert_eq!(protocol, "TCP");
        assert_eq!(packet, b"GET / HTTP/1.1\r\n\r\n");
    }

    #[tokio::test]
    async fn test_handle_tcp_large_packet() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(handle_tcp(listener, tx));

        let mut stream = TcpStream::connect(addr).await.unwrap();
        let large_packet = vec![0u8; 2048];
        stream.write_all(&large_packet).await.unwrap();

        let (protocol, _addr, packet) = rx.recv().await.unwrap();
        assert_eq!(protocol, "TCP");
        assert_eq!(packet, large_packet[..1024].to_vec());
    }
}
