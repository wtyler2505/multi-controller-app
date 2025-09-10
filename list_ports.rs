fn main() {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("No serial ports found!");
            } else {
                println!("Available serial ports:");
                for p in ports {
                    println!("  {} - {:?}", p.port_name, p.port_type);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing ports: {}", e);
        }
    }
}