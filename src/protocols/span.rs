pub async fn detect_span(packet: &[u8]) -> bool {
    if packet.len() >= 14 {
        let dest_mac = &packet[0..6];
        let src_mac = &packet[6..12];
        let eth_type = &packet[12..14];
        println!("SPAN packet detected:");
        println!("Destination MAC: {:02x?}", dest_mac);
        println!("Source MAC: {:02x?}", src_mac);
        println!("Ethernet Type: {:02x?}", eth_type);
        return true;
    }
    println!("Not a SPAN packet: {:?}", packet);
    false
}

pub async fn detect_rspan(packet: &[u8]) -> bool {
    if packet.len() >= 18 {
        let dest_mac = &packet[0..6];
        let src_mac = &packet[6..12];
        let eth_type = &packet[12..14];
        let vlan_id = ((packet[14] as u16 & 0x0F) << 8) | packet[15] as u16;
        println!("RSPAN packet detected:");
        println!("Destination MAC: {:02x?}", dest_mac);
        println!("Source MAC: {:02x?}", src_mac);
        println!("Ethernet Type: {:02x?}", eth_type);
        println!("VLAN ID: {}", vlan_id);
        if vlan_id == 4095 {
            return true;
        }
    }
    println!("Not an RSPAN packet: {:?}", packet);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_span() {
        let span_packet = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Destination MAC (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source MAC (dummy)
            0x08, 0x00, // Ethernet type (IPv4)
            // IPv4 data (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(detect_span(&span_packet).await);
    }

    #[tokio::test]
    async fn test_detect_rspan() {
        let rspan_packet = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Destination MAC (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source MAC (dummy)
            0x81, 0x00, 0x0F, 0xFF, // VLAN tag (RSPAN VLAN ID 4095)
            0x08, 0x00, // Ethernet type (IPv4)
            // IPv4 data (dummy)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(detect_rspan(&rspan_packet).await);
    }
}
