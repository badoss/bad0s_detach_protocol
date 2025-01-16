pub async fn detect_stp(packet: &[u8]) -> bool {
    // STP packets typically start with a destination MAC address of 01:80:C2:00:00:00
    // and have an Ethernet type of 0x0026 (for IEEE 802.1D)
    if packet.len() >= 14 {
        let dest_mac = &packet[0..6];
        let eth_type = &packet[12..14];
        if dest_mac == [0x01, 0x80, 0xC2, 0x00, 0x00, 0x00] && eth_type == [0x00, 0x26] {
            println!("STP packet detected: {:?}", packet);
            // show detail of STP packet
            let stp_data = &packet[14..];
            println!("Protocol Identifier: {:?}", &stp_data[0..2]);
            println!("Protocol Version Identifier: {:?}", &stp_data[2..3]);
            return true;
        }
    }
    println!("Not an STP packet: {:?}", packet);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_stp() {
        let stp_packet = [
            0x01, 0x80, 0xC2, 0x00, 0x00, 0x00, // Destination MAC
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source MAC (dummy)
            0x00, 0x26, // Ethernet type (STP)
            // STP data (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(detect_stp(&stp_packet).await);
    }

    #[tokio::test]
    async fn test_detect_non_stp() {
        let non_stp_packet = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Destination MAC (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source MAC (dummy)
            0x08, 0x00, // Ethernet type (IPv4)
            // IPv4 data (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(!detect_stp(&non_stp_packet).await);
    }
}
