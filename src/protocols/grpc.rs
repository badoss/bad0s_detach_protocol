pub async fn detect_grpc(packet: &[u8]) -> bool {
    // Basic check for gRPC protocol (this is a placeholder, real detection logic will be more complex)
    if packet.len() >= 5 && &packet[0..5] == b"\x00\x00\x00\x00\x00" {
        println!("gRPC Packet Detected");
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_grpc() {
        let packet = b"\x00\x00\x00\x00\x00";
        assert!(detect_grpc(packet).await);

        let invalid_packet = b"\x01\x00\x00\x00\x00";
        assert!(!detect_grpc(invalid_packet).await);
    }
}
