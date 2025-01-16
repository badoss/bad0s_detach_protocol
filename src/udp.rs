use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub async fn handle_udp(
    udp_socket: UdpSocket,
    tx: mpsc::Sender<(&'static str, std::net::SocketAddr, Vec<u8>)>,
) {
    let mut buf = vec![0; 1024];
    loop {
        let (n, addr) = udp_socket.recv_from(&mut buf).await.unwrap();
        // println!("Received UDP packet from {}: {:?}", addr, &buf[..n]);
        // println!("Received port: {}", addr.port());
        // println!("Received packet detail: {:?}", &buf[..n]);
        tx.send(("UDP", addr, buf[..n].to_vec())).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_udp() {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let addr = socket.local_addr().unwrap();
        let (tx, mut rx) = mpsc::channel(1);

        tokio::spawn(handle_udp(socket, tx));

        let send_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        send_socket.send_to(b"test", addr).await.unwrap();

        let (protocol, _addr, packet) = rx.recv().await.unwrap();
        assert_eq!(protocol, "UDP");
        assert_eq!(packet, b"test");
    }
}
