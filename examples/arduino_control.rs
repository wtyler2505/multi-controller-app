//! Arduino Control Example
//! 
//! This example demonstrates direct control of an Arduino using the SerialTransport.
//! Make sure to upload the test_sketch.ino to your Arduino first.
//!
//! Usage:
//!   cargo run --example arduino_control COM3

use multi_controller_app::transport::{Transport, TransportConfig, TransportType};
use multi_controller_app::transport::serial::SerialTransport;
use std::time::Duration;
use tokio::time::sleep;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    // Get port from command line or default to COM3
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "COM3".to_string());
    
    println!("Arduino Control Example");
    println!("======================");
    println!("Using port: {}", port);
    println!();
    println!("Make sure:");
    println!("1. Arduino is connected to {}", port);
    println!("2. test_sketch.ino is uploaded to the Arduino");
    println!();
    
    // Configure serial transport
    let config = TransportConfig {
        transport_type: TransportType::Serial,
        address: port.clone(),
        port: 0, // Not used for serial
        baud_rate: Some(115200),
        timeout_ms: 500,
        retry_count: 3,
        latency_ms: Some(50),
        auto_reconnect: true,
        reconnect_delay_ms: 1000,
        max_reconnect_attempts: 5,
    };
    
    // Create and connect transport
    let mut transport = SerialTransport::new(config)?;
    
    println!("Connecting to Arduino...");
    transport.connect().await?;
    println!("Connected!");
    
    // Wait a moment for Arduino to initialize
    sleep(Duration::from_millis(1000)).await;
    
    // Helper function to send command and get response
    async fn send_command(
        transport: &mut SerialTransport,
        command: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("→ Sending: {}", command);
        
        // Send command with line ending
        let cmd_bytes = format!("{}\r\n", command).into_bytes();
        transport.send(&cmd_bytes).await?;
        
        // Receive response
        let response_bytes = transport.receive(Duration::from_millis(500)).await?;
        let response = String::from_utf8_lossy(&response_bytes)
            .trim()
            .to_string();
        
        println!("← Response: {}", response);
        Ok(response)
    }
    
    println!("\n=== Testing Arduino Communication ===\n");
    
    // Test 1: Probe for Arduino
    println!("Test 1: Probe Arduino");
    let response = send_command(&mut transport, "PROBE").await?;
    assert_eq!(response, "ARDUINO_UNO", "Arduino identification failed");
    println!("✓ Arduino identified\n");
    
    // Test 2: Get version
    println!("Test 2: Get firmware version");
    let response = send_command(&mut transport, "VERSION").await?;
    assert!(response.starts_with("VERSION:"), "Version query failed");
    println!("✓ Firmware version: {}\n", response);
    
    // Test 3: Ping test
    println!("Test 3: Ping test");
    let response = send_command(&mut transport, "PING").await?;
    assert_eq!(response, "PONG", "Ping test failed");
    println!("✓ Ping successful\n");
    
    // Test 4: Control built-in LED
    println!("Test 4: LED Control (Pin 13)");
    
    println!("  Turning LED ON...");
    let response = send_command(&mut transport, "LED_ON").await?;
    assert_eq!(response, "OK", "LED ON failed");
    sleep(Duration::from_millis(1000)).await;
    
    println!("  Checking LED state...");
    let response = send_command(&mut transport, "LED_STATE").await?;
    assert_eq!(response, "STATE:1", "LED state check failed");
    
    println!("  Turning LED OFF...");
    let response = send_command(&mut transport, "LED_OFF").await?;
    assert_eq!(response, "OK", "LED OFF failed");
    sleep(Duration::from_millis(500)).await;
    
    println!("  Checking LED state...");
    let response = send_command(&mut transport, "LED_STATE").await?;
    assert_eq!(response, "STATE:0", "LED state check failed");
    println!("✓ LED control working\n");
    
    // Test 5: Blink LED
    println!("Test 5: LED Blink Pattern");
    for i in 0..5 {
        print!("  Blink {} of 5...", i + 1);
        io::stdout().flush()?;
        
        send_command(&mut transport, "LED_ON").await?;
        sleep(Duration::from_millis(200)).await;
        send_command(&mut transport, "LED_OFF").await?;
        sleep(Duration::from_millis(200)).await;
        
        println!(" done");
    }
    println!("✓ Blink pattern complete\n");
    
    // Test 6: Read analog value
    println!("Test 6: Read Analog Pin A0");
    let response = send_command(&mut transport, "ANALOG_READ 0").await?;
    if response.starts_with("VALUE:") {
        let value: i32 = response[6..].parse().unwrap_or(0);
        println!("✓ Analog A0 value: {} (0-1023 range)\n", value);
    }
    
    // Test 7: Digital I/O on pin 7
    println!("Test 7: Digital I/O (Pin 7)");
    
    println!("  Setting pin 7 as OUTPUT...");
    let response = send_command(&mut transport, "PIN_MODE 7 OUTPUT").await?;
    assert_eq!(response, "OK", "PIN_MODE failed");
    
    println!("  Writing HIGH to pin 7...");
    let response = send_command(&mut transport, "DIGITAL_WRITE 7 HIGH").await?;
    assert_eq!(response, "OK", "DIGITAL_WRITE HIGH failed");
    sleep(Duration::from_millis(500)).await;
    
    println!("  Writing LOW to pin 7...");
    let response = send_command(&mut transport, "DIGITAL_WRITE 7 LOW").await?;
    assert_eq!(response, "OK", "DIGITAL_WRITE LOW failed");
    println!("✓ Digital I/O working\n");
    
    // Test 8: PWM on pin 9
    println!("Test 8: PWM Control (Pin 9)");
    println!("  Setting pin 9 as OUTPUT...");
    send_command(&mut transport, "PIN_MODE 9 OUTPUT").await?;
    
    for value in [0, 64, 128, 192, 255] {
        println!("  PWM value: {}/255", value);
        let cmd = format!("PWM_WRITE 9 {}", value);
        let response = send_command(&mut transport, &cmd).await?;
        assert_eq!(response, "OK", "PWM_WRITE failed");
        sleep(Duration::from_millis(300)).await;
    }
    
    // Turn off PWM
    send_command(&mut transport, "PWM_WRITE 9 0").await?;
    println!("✓ PWM control working\n");
    
    // Measure latency
    println!("=== Performance Test ===");
    println!("Measuring command latency (50 iterations)...");
    
    let mut total_time = Duration::ZERO;
    for _ in 0..50 {
        let start = std::time::Instant::now();
        send_command(&mut transport, "PING").await?;
        total_time += start.elapsed();
    }
    
    let avg_latency = total_time / 50;
    println!("Average latency: {:?}", avg_latency);
    
    if avg_latency < Duration::from_millis(50) {
        println!("✓ Latency under 50ms target!");
    } else {
        println!("⚠ Latency exceeds 50ms target");
    }
    
    // Interactive mode
    println!("\n=== Interactive Mode ===");
    println!("Enter commands to send to Arduino (or 'quit' to exit):");
    println!("Try: HELP, LED_ON, LED_OFF, PING, etc.");
    println!();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();
        
        if command.eq_ignore_ascii_case("quit") || command.eq_ignore_ascii_case("exit") {
            break;
        }
        
        if !command.is_empty() {
            match send_command(&mut transport, command).await {
                Ok(response) => {
                    // Response already printed by send_command
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
    
    // Clean up
    println!("\nDisconnecting...");
    transport.disconnect().await?;
    println!("Goodbye!");
    
    Ok(())
}