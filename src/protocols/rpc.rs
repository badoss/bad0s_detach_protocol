pub async fn detect_rpc(packet: &[u8]) -> bool {
    // Basic check for RPC protocol (this is a placeholder, real detection logic will be more complex)
    if packet.len() >= 4 && &packet[0..4] == b"\x80\x00\x00\x00" {
        println!("RPC Packet Detected");
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_rpc() {
        let packet = b"\x80\x00\x00\x00";
        assert!(detect_rpc(packet).await);

        let invalid_packet = b"\x00\x00\x00\x00";
        assert!(!detect_rpc(invalid_packet).await);
    }
}
