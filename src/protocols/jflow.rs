pub async fn detect_jflow(packet: &[u8]) -> bool {
    if packet.len() < 24 {
        return false;
    }

    let version = u16::from_be_bytes([packet[0], packet[1]]);
    if version != 5 {
        return false;
    }

    println!("JFlow v5 Packet Detected:");
    println!("Version: {}", version);
    println!("Count: {}", u16::from_be_bytes([packet[2], packet[3]]));
    println!(
        "System Uptime: {}",
        u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]])
    );
    println!(
        "Unix Seconds: {}",
        u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]])
    );
    println!(
        "Unix Nanoseconds: {}",
        u32::from_be_bytes([packet[12], packet[13], packet[14], packet[15]])
    );
    println!(
        "Flow Sequence: {}",
        u32::from_be_bytes([packet[16], packet[17], packet[18], packet[19]])
    );
    println!("Engine Type: {}", packet[20]);
    println!("Engine ID: {}", packet[21]);
    println!(
        "Sampling Interval: {}",
        u16::from_be_bytes([packet[22], packet[23]])
    );

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_jflow() {
        let packet = [
            0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(detect_jflow(&packet).await);
    }
}
