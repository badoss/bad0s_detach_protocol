pub async fn detect_sflow(packet: &[u8]) -> bool {
    if packet.len() < 16 {
        return false;
    }

    let version = u32::from_be_bytes([packet[0], packet[1], packet[2], packet[3]]);
    if version != 5 {
        return false;
    }

    println!("sFlow v5 Packet Detected:");
    println!("Version: {}", version);
    println!(
        "IP Version: {}",
        u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]])
    );
    println!("Agent Address: {:?}", &packet[8..12]);
    println!(
        "Sub Agent ID: {}",
        u32::from_be_bytes([packet[12], packet[13], packet[14], packet[15]])
    );

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_sflow() {
        let packet = [0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(detect_sflow(&packet).await);
    }
}
