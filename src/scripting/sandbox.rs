use std::time::Duration;

/// Resource limits for script execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum number of operations before forcing termination
    pub max_operations: u64,
    
    /// Maximum execution time
    pub max_execution_time: Duration,
    
    /// Maximum memory usage in bytes
    pub max_memory: usize,
    
    /// Maximum string length
    pub max_string_size: usize,
    
    /// Maximum array size
    pub max_array_size: usize,
    
    /// Maximum call depth
    pub max_call_depth: usize,
    
    /// Maximum loop iterations
    pub max_iterations: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_execution_time: Duration::from_secs(10),
            max_memory: 10 * 1024 * 1024, // 10MB
            max_string_size: 1_000_000,    // 1MB strings
            max_array_size: 10_000,
            max_call_depth: 32,
            max_iterations: 10_000,
        }
    }
}

impl ResourceLimits {
    /// Create strict limits for untrusted scripts
    pub fn strict() -> Self {
        Self {
            max_operations: 10_000,
            max_execution_time: Duration::from_secs(1),
            max_memory: 1024 * 1024, // 1MB
            max_string_size: 10_000,
            max_array_size: 1_000,
            max_call_depth: 8,
            max_iterations: 1_000,
        }
    }
    
    /// Create relaxed limits for trusted scripts
    pub fn relaxed() -> Self {
        Self {
            max_operations: 1_000_000,
            max_execution_time: Duration::from_secs(60),
            max_memory: 100 * 1024 * 1024, // 100MB
            max_string_size: 10_000_000,    // 10MB
            max_array_size: 100_000,
            max_call_depth: 64,
            max_iterations: 100_000,
        }
    }
}

/// Sandbox configuration for script execution
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Resource limits
    pub limits: ResourceLimits,
    
    /// Allow network access
    pub allow_network: bool,
    
    /// Allow file system access
    pub allow_filesystem: bool,
    
    /// Allow system commands
    pub allow_system_commands: bool,
    
    /// Allowed device operations
    pub allowed_device_ops: Vec<String>,
    
    /// Denied API functions
    pub denied_functions: Vec<String>,
    
    /// Enable debug output
    pub enable_debug: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            limits: ResourceLimits::default(),
            allow_network: false,
            allow_filesystem: false,
            allow_system_commands: false,
            allowed_device_ops: vec![
                "read".to_string(),
                "write".to_string(),
                "subscribe".to_string(),
            ],
            denied_functions: vec![
                "eval".to_string(),
                "import".to_string(),
            ],
            enable_debug: false,
        }
    }
}

impl SandboxConfig {
    /// Create a high-security sandbox for untrusted scripts
    pub fn high_security() -> Self {
        Self {
            limits: ResourceLimits::strict(),
            allow_network: false,
            allow_filesystem: false,
            allow_system_commands: false,
            allowed_device_ops: vec!["read".to_string()], // Read-only
            denied_functions: vec![
                "eval".to_string(),
                "import".to_string(),
                "load".to_string(),
                "require".to_string(),
            ],
            enable_debug: false,
        }
    }
    
    /// Create a development sandbox with relaxed restrictions
    pub fn development() -> Self {
        Self {
            limits: ResourceLimits::relaxed(),
            allow_network: false, // Still no network in dev
            allow_filesystem: true,
            allow_system_commands: false,
            allowed_device_ops: vec![
                "read".to_string(),
                "write".to_string(),
                "subscribe".to_string(),
                "control".to_string(),
            ],
            denied_functions: vec![],
            enable_debug: true,
        }
    }
}