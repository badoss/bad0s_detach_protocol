mod protocols;
mod surrealdb;
mod tcp;
mod udp;

use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let udp_socket = UdpSocket::bind("0.0.0.0:8080").await.unwrap();

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(tcp::handle_tcp(tcp_listener, tx.clone()));
    tokio::spawn(udp::handle_udp(udp_socket, tx));

    while let Some((protocol, addr, packet)) = rx.recv().await {
        println!("From IP: {}", addr.ip());
        // println!("Spanning port connect: {}", addr.port());
        // println!("Spanning packet detail: {:?}", packet);
        println!("Protocol: {}", protocol);

        match protocol {
            "TCP" => {
                match_packet_type(&packet).await;
            }
            "UDP" => {
                match_packet_type(&packet).await;
                // println!("UDP Protocol Detected");
            }
            _ => println!("Unknown Protocol Detected"),
        }

        async fn match_packet_type(packet: &[u8]) {
            if protocols::netflow::detect_netflow(packet).await {
                // println!("NetFlow v5 Protocol Detected");
            } else if protocols::netflow::detect_netflow_v9(packet).await {
                // println!("NetFlow v9 Protocol Detected");
            } else if protocols::sflow::detect_sflow(packet).await {
                println!("sFlow Protocol Detected");
            } else if protocols::jflow::detect_jflow(packet).await {
                println!("JFlow Protocol Detected");
            } else if protocols::rpc::detect_rpc(packet).await {
                println!("RPC Protocol Detected");
            } else if protocols::grpc::detect_grpc(packet).await {
                println!("gRPC Protocol Detected");
            } else if protocols::http::detect_http(packet).await {
                println!("HTTP Protocol Detected");
            } else if protocols::snmp::detect_snmp_v1(packet).await {
                println!("SNMP v1 Protocol Detected");
            } else if protocols::snmp::detect_snmp_v2(packet).await {
                println!("SNMP v2 Protocol Detected");
            } else if protocols::snmp::detect_snmp_v3(packet).await {
                println!("SNMP v3 Protocol Detected");
            } else if protocols::stp::detect_stp(packet).await {
                println!("STP Protocol Detected");
            } else if protocols::span::detect_span(packet).await {
                println!("SPAN Protocol Detected");
            } else if protocols::span::detect_rspan(packet).await {
                println!("RSPAN Protocol Detected");
            } else {
                println!("Unknown Packet Type Detected");
            }
        }
    }
}
