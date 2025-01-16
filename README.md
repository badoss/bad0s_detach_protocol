# bad0s detach protocol

This project is a network packet analyzer that supports detecting various protocols including:

- NetFlow v5
- NetFlow v9
- sFlow
- JFlow
- RPC
- gRPC
- HTTP
- SNMP
- STP

## Project Structure
```
src/
├── main.rs 
├── tcp.rs 
├── udp.rs 
├── surrealdb/
└── protocols/ 
    ├── mod.rs 
    ├── netflow.rs 
    ├── sflow.rs 
    ├── jflow.rs 
    ├── rpc.rs 
    ├── grpc.rs
    ├── http.rs
    ├── stp.rs
    ├── span.rs
    └── snmp.rs
```

## Start DB

create .env file
```
DB_USER=root
DB_PASSWORD=root
```

### docker start
start serrealDB 
```
docker compose up -d
```

## Usage

To run the project, use the following command:

```sh
cargo run
```

### How to Test
```sh
cargo test
```

## Protocol Detection

### NetFlow v5
NetFlow v5 packets are detected and details such as version, count, system uptime, Unix seconds, Unix nanoseconds, flow sequence, engine type, engine ID, and sampling interval are printed. Additionally, flow records are parsed to show source and destination addresses and ports.

### NetFlow v9
NetFlow v9 packets are detected and details such as version, count, system uptime, Unix seconds, flow sequence, and source ID are printed.

### sFlow
sFlow v5 packets are detected and details such as version, IP version, agent address, and sub-agent ID are printed.

### JFlow
JFlow v5 packets are detected and details such as version, count, system uptime, Unix seconds, Unix nanoseconds, flow sequence, engine type, engine ID, and sampling interval are printed.

### RPC
RPC packets are detected based on a basic check. Real detection logic can be more complex.

### gRPC
gRPC packets are detected based on a basic check. Real detection logic can be more complex.

### HTTP
HTTP packets are detected by checking for common HTTP methods such as GET, POST, PUT, DELETE, HEAD, OPTIONS, and PATCH. Headers and body content are printed. WebSocket upgrade requests are also detected and logged.

### SNMP
SNMP packets are detected for versions v1, v2, and v3. Details such as version and specific packet content are printed.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License
This project is licensed under the MIT License.