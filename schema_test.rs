//! Focused handshake schema validation test
//! 
//! This test validates only the handshake schema implementation
//! without depending on the full project compilation.

use std::collections::HashMap;
use uuid::Uuid;

// Import the specific handshake schema types we want to test
use multi_controller_app::protocols::handshake::schema::*;

fn main() {
    println!("Testing Handshake Schema Implementation...");

    // Test 1: Basic IDENTIFY command creation and validation
    let identify = IdentifyCommand {
        command: "IDENTIFY".to_string(),
        protocol_version: "1.0.0".to_string(),
        session_id: Uuid::new_v4(),
        capabilities_requested: vec!["basic_io".to_string(), "telemetry".to_string()],
        timestamp: Some("2025-01-06T12:00:00Z".to_string()),
        client_info: Some(ClientInfo {
            name: "Multi-Controller App".to_string(),
            version: "0.1.0".to_string(),
            platform: "Windows".to_string(),
            metadata: HashMap::new(),
        }),
        auth_token: None,
        custom_params: HashMap::new(),
    };

    // Validate the command
    match identify.validate() {
        Ok(()) => println!("✓ IDENTIFY command validation passed"),
        Err(e) => {
            eprintln!("✗ IDENTIFY command validation failed: {}", e);
            return;
        }
    }

    // Test JSON serialization
    match serde_json::to_string(&identify) {
        Ok(json) => {
            println!("✓ IDENTIFY command JSON serialization successful");
            
            // Test deserialization
            match serde_json::from_str::<IdentifyCommand>(&json) {
                Ok(_) => println!("✓ IDENTIFY command JSON deserialization successful"),
                Err(e) => {
                    eprintln!("✗ IDENTIFY command JSON deserialization failed: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("✗ IDENTIFY command JSON serialization failed: {}", e);
            return;
        }
    }

    // Test 2: IDENTIFY response validation
    let response = IdentifyResponse {
        status: "OK".to_string(),
        device_id: "arduino_uno_001".to_string(),
        device_type: "Arduino_Uno".to_string(),
        firmware_version: "2.1.0".to_string(),
        protocol_version: "1.0.0".to_string(),
        capabilities: vec![
            Capability {
                name: "basic_io".to_string(),
                version: "1.0.0".to_string(),
                description: "Basic digital and analog I/O operations".to_string(),
                enabled_by_default: true,
                parameters: HashMap::new(),
                dependencies: vec![],
                min_protocol_version: Some("1.0.0".to_string()),
            }
        ],
        session_accepted: true,
        error_message: None,
        error_code: None,
        device_info: None,
        timestamp: Some("2025-01-06T12:00:01Z".to_string()),
        session_id: Some(identify.session_id),
        custom_params: HashMap::new(),
    };

    match response.validate() {
        Ok(()) => println!("✓ IDENTIFY response validation passed"),
        Err(e) => {
            eprintln!("✗ IDENTIFY response validation failed: {}", e);
            return;
        }
    }

    // Test 3: Error message validation
    let error_msg = ErrorMessage {
        message_type: "ERROR".to_string(),
        session_id: Some(identify.session_id),
        severity: ErrorSeverity::Error,
        category: ErrorCategory::Protocol,
        message: "Test error message".to_string(),
        error_code: Some("TEST_ERROR".to_string()),
        details: None,
        timestamp: Some("2025-01-06T12:00:02Z".to_string()),
        retryable: true,
    };

    match error_msg.validate() {
        Ok(()) => println!("✓ Error message validation passed"),
        Err(e) => {
            eprintln!("✗ Error message validation failed: {}", e);
            return;
        }
    }

    // Test 4: Semantic version validation
    let valid_versions = vec!["1.0.0", "2.1.5", "10.20.30"];
    for version in valid_versions {
        if validate_semver(version).is_ok() {
            println!("✓ Semver validation passed for {}", version);
        } else {
            eprintln!("✗ Semver validation failed for {}", version);
            return;
        }
    }

    let invalid_versions = vec!["1.0", "1.0.0.1", "v1.0.0"];
    for version in invalid_versions {
        if validate_semver(version).is_err() {
            println!("✓ Semver validation correctly rejected {}", version);
        } else {
            eprintln!("✗ Semver validation incorrectly accepted {}", version);
            return;
        }
    }

    // Test 5: String length validation  
    let normal_string = "This is a normal string";
    if validate_string_length(&normal_string, "test").is_ok() {
        println!("✓ String length validation passed for normal string");
    } else {
        eprintln!("✗ String length validation failed for normal string");
        return;
    }

    let too_long_string = "a".repeat(MAX_STRING_LENGTH + 1);
    match validate_string_length(&too_long_string, "test") {
        Err(ValidationError::StringTooLong { .. }) => println!("✓ String length validation correctly rejected oversized string"),
        _ => {
            eprintln!("✗ String length validation should have rejected oversized string");
            return;
        }
    }

    // Test 6: Protocol constants
    println!("✓ Protocol version: {}", PROTOCOL_VERSION);
    println!("✓ Max string length: {}", MAX_STRING_LENGTH);
    println!("✓ Max capabilities: {}", MAX_CAPABILITIES);
    println!("✓ Max parameters: {}", MAX_PARAMETERS);

    // Test 7: Message examples functionality
    let example_cmd = MessageExamples::identify_command();
    if example_cmd.validate().is_ok() {
        println!("✓ Example IDENTIFY command is valid");
    } else {
        eprintln!("✗ Example IDENTIFY command is invalid");
        return;
    }

    let example_response = MessageExamples::identify_response_success();
    if example_response.validate().is_ok() {
        println!("✓ Example IDENTIFY response is valid");
    } else {
        eprintln!("✗ Example IDENTIFY response is invalid");
        return;
    }

    println!("\n🎉 All handshake schema tests passed!");
    println!("Task 28.1: Define Handshake Message Schema - COMPLETE");
}
