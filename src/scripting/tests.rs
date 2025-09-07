#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripting::{ScriptEngine, SandboxConfig, DeviceApi};
    use std::sync::Arc;
    
    #[test]
    fn test_sandbox_config_creation() {
        let config = SandboxConfig::default();
        assert_eq!(config.allow_network, false);
        assert_eq!(config.allow_filesystem, false);
        
        let strict = SandboxConfig::high_security();
        assert_eq!(strict.limits.max_operations, 10_000);
        
        let dev = SandboxConfig::development();
        assert_eq!(dev.enable_debug, true);
    }
    
    #[test]
    fn test_resource_limits() {
        let limits = crate::scripting::ResourceLimits::default();
        assert_eq!(limits.max_operations, 100_000);
        assert_eq!(limits.max_memory, 10 * 1024 * 1024);
        
        let strict = crate::scripting::ResourceLimits::strict();
        assert_eq!(strict.max_operations, 10_000);
        assert_eq!(strict.max_execution_time.as_secs(), 1);
    }
    
    #[tokio::test]
    async fn test_simple_script_compilation() {
        // Create a mock device manager
        let manager = Arc::new(crate::device::DeviceManager::new());
        let device_api = Arc::new(DeviceApi::new(manager));
        let config = SandboxConfig::default();
        
        let engine = ScriptEngine::new(config, device_api).unwrap();
        
        // Test compiling a simple script
        let script = r#"
            let x = 1 + 1;
            print("Result: " + x);
        "#;
        
        let result = engine.compile_script("test_script", script).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_security_validation() {
        let manager = Arc::new(crate::device::DeviceManager::new());
        let device_api = Arc::new(DeviceApi::new(manager));
        let config = SandboxConfig::high_security();
        
        let engine = ScriptEngine::new(config, device_api).unwrap();
        
        // Test that dangerous patterns are rejected
        let dangerous_script = r#"
            let result = std::process::Command::new("ls");
        "#;
        
        let result = engine.compile_script("danger", dangerous_script).await;
        assert!(result.is_err());
        
        match result {
            Err(crate::scripting::ScriptError::Security(msg)) => {
                assert!(msg.contains("forbidden pattern"));
            }
            _ => panic!("Expected security error"),
        }
    }
    
    #[tokio::test]
    async fn test_simple_eval() {
        let manager = Arc::new(crate::device::DeviceManager::new());
        let device_api = Arc::new(DeviceApi::new(manager));
        let config = SandboxConfig::default();
        
        let engine = ScriptEngine::new(config, device_api).unwrap();
        
        // Test evaluating a simple expression
        let result = engine.eval("1 + 2 + 3").await;
        assert!(result.is_ok());
        
        if let Ok(value) = result {
            // Rhai returns values as Dynamic
            assert_eq!(value.as_int().unwrap(), 6);
        }
    }
}