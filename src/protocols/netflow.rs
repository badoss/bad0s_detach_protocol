use crate::surrealdb::connection::_connect_to_db;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct NetFlowV5Record {
    pub src_addr: String,
    pub dst_addr: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub tos: u8,
    pub tcp_flags: u8,
    pub packet_size: u16,
}

#[derive(Serialize)]
pub struct NetFlowV5 {
    pub version: u16,
    pub count: u16,
    pub system_uptime: u32,
    pub unix_seconds: u32,
    pub unix_nanoseconds: u32,
    pub flow_sequence: u32,
    pub engine_type: u8,
    pub engine_id: u8,
    pub sampling_interval: u16,
    pub records: Vec<NetFlowV5Record>,
    pub timestamp: String,
    pub packet_details: Vec<u8>,
    pub packet_size: usize,
}

#[derive(Serialize)]
pub struct NetFlowV9 {
    pub version: u16,
    pub count: u16,
    pub system_uptime: u32,
    pub unix_seconds: u32,
    pub flow_sequence: u32,
    pub source_id: u32,
    pub packet_size: usize,
}

pub async fn detect_netflow(packet: &[u8]) -> bool {
    if packet.len() < 24 {
        return false;
    }

    let version = u16::from_be_bytes([packet[0], packet[1]]);
    if version != 5 {
        return false;
    }

    let count = u16::from_be_bytes([packet[2], packet[3]]);
    let mut records = Vec::new();
    let record_size = 48;
    for i in 0..count {
        let start = 24 + (i as usize) * record_size;
        let end = start + record_size;
        if end > packet.len() {
            break;
        }
        let record = &packet[start..end];
        let src_addr = u32::from_be_bytes([record[0], record[1], record[2], record[3]]);
        let dst_addr = u32::from_be_bytes([record[4], record[5], record[6], record[7]]);
        let src_port = u16::from_be_bytes([record[32], record[33]]);
        let dst_port = u16::from_be_bytes([record[34], record[35]]);
        let protocol = record[38];
        let tos = record[39];
        let tcp_flags = record[40];
        let packet_size = u16::from_be_bytes([record[42], record[43]]);

        records.push(NetFlowV5Record {
            src_addr: std::net::Ipv4Addr::from(src_addr).to_string(),
            dst_addr: std::net::Ipv4Addr::from(dst_addr).to_string(),
            src_port,
            dst_port,
            protocol,
            tos,
            tcp_flags,
            packet_size,
        });
    }

    let netflow_v5 = NetFlowV5 {
        version,
        count,
        system_uptime: u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]]),
        unix_seconds: u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]]),
        unix_nanoseconds: u32::from_be_bytes([packet[12], packet[13], packet[14], packet[15]]),
        flow_sequence: u32::from_be_bytes([packet[16], packet[17], packet[18], packet[19]]),
        engine_type: packet[20],
        engine_id: packet[21],
        sampling_interval: u16::from_be_bytes([packet[22], packet[23]]),
        records,
        timestamp: Utc::now().to_rfc3339(),
        packet_details: packet.to_vec(),
        packet_size: packet.len(),
    };

    let db = _connect_to_db().await;
    let query = format!(
        "CREATE netflow_v5 CONTENT {}",
        serde_json::to_string(&netflow_v5).unwrap()
    );
    db.query(query).await.unwrap();

    true
}

pub async fn detect_netflow_v9(packet: &[u8]) -> bool {
    if packet.len() < 20 {
        return false;
    }

    let version = u16::from_be_bytes([packet[0], packet[1]]);
    if version != 9 {
        return false;
    }

    let netflow_v9 = NetFlowV9 {
        version,
        count: u16::from_be_bytes([packet[2], packet[3]]),
        system_uptime: u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]]),
        unix_seconds: u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]]),
        flow_sequence: u32::from_be_bytes([packet[12], packet[13], packet[14], packet[15]]),
        source_id: u32::from_be_bytes([packet[16], packet[17], packet[18], packet[19]]),
        packet_size: packet.len(),
    };

    let db = _connect_to_db().await;
    let netflow_v9_value = serde_json::to_value(&netflow_v9).unwrap();
    let query = format!("CREATE netflow_v9 CONTENT {}", netflow_v9_value);
    db.query(query).await.unwrap();

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_netflow() {
        let packet = [
            0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(detect_netflow(&packet).await);
    }

    #[tokio::test]
    async fn test_detect_netflow_v9() {
        let packet = [0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(detect_netflow_v9(&packet).await);
    }
}
