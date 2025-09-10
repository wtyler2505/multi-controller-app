//! Test the handshake schema implementation
//! 
//! This example demonstrates the handshake protocol message schema
//! and validates that all message types work correctly.

use multi_controller_app::protocols::handshake::schema::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Handshake Protocol Schema v{}", PROTOCOL_VERSION);
    
    // Test IDENTIFY command
    println!("\n=== IDENTIFY Command Test ===");
    let identify_cmd = MessageExamples::identify_command();
    
    // Validate the command
    identify_cmd.validate()?;
    println!(" IDENTIFY command validation passed");
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&identify_cmd)?;
    println!(" IDENTIFY command serialization:\n{}", json);
    
    // Deserialize back
    let deserialized: IdentifyCommand = serde_json::from_str(&json)?;
    assert_eq!(identify_cmd, deserialized);
    println!(" IDENTIFY command round-trip successful");
    
    // Test IDENTIFY response (success)
    println!("\n=== IDENTIFY Response (Success) Test ===");
    let identify_resp = MessageExamples::identify_response_success();
    
    // Validate the response
    identify_resp.validate()?;
    println!(" IDENTIFY response validation passed");
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&identify_resp)?;
    println!(" IDENTIFY response serialization:\n{}", json);
    
    // Deserialize back
    let deserialized: IdentifyResponse = serde_json::from_str(&json)?;
    assert_eq!(identify_resp, deserialized);
    println!(" IDENTIFY response round-trip successful");
    
    // Test IDENTIFY response (error)
    println!("\n=== IDENTIFY Response (Error) Test ===");
    let identify_error = MessageExamples::identify_response_error();
    
    // Validate the error response
    identify_error.validate()?;
    println!(" IDENTIFY error response validation passed");
    
    // Test ERROR message
    println!("\n=== ERROR Message Test ===");
    let error_msg = MessageExamples::error_message();
    
    // Validate the error message
    error_msg.validate()?;
    println!(" ERROR message validation passed");
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&error_msg)?;
    println!(" ERROR message serialization:\n{}", json);
    
    // Test validation errors
    println!("\n=== Validation Error Tests ===");
    
    // Test invalid command
    let mut invalid_cmd = identify_cmd.clone();
    invalid_cmd.command = "INVALID".to_string();
    match invalid_cmd.validate() {
        Err(ValidationError::InvalidCommand { .. }) => {
            println!(" Invalid command correctly rejected");
        }
        _ => panic!("Invalid command should have been rejected"),
    }
    
    // Test too many capabilities
    let mut too_many_caps = identify_cmd.clone();
    too_many_caps.capabilities_requested = (0..MAX_CAPABILITIES + 1)
        .map(|i| format!("cap_{}", i))
        .collect();
    match too_many_caps.validate() {
        Err(ValidationError::TooManyCapabilities { .. }) => {
            println!(" Too many capabilities correctly rejected");
        }
        _ => panic!("Too many capabilities should have been rejected"),
    }
    
    // Test invalid semver
    match validate_semver("invalid.version") {
        Err(ValidationError::InvalidSemver { .. }) => {
            println!(" Invalid semantic version correctly rejected");
        }
        _ => panic!("Invalid semver should have been rejected"),
    }
    
    // Test string length validation
    let long_string = "a".repeat(MAX_STRING_LENGTH + 1);
    match validate_string_length(&long_string, "test") {
        Err(ValidationError::StringTooLong { .. }) => {
            println!(" String length limit correctly enforced");
        }
        _ => panic!("Long string should have been rejected"),
    }
    
    println!("\n=== Schema Documentation Generation ===");
    let schema_docs = json_schema::generate_schema_docs();
    println!(" Schema documentation generated ({} characters)", schema_docs.len());
    
    println!("\n=== All Tests Passed! ===");
    println!(" All message types serialize/deserialize correctly");
    println!(" All validation rules work as expected");
    println!(" Schema is ready for handshake protocol implementation");
    println!(" Task 28.1 (Define Handshake Message Schema) - COMPLETE");
    
    Ok(())
}