# Dual-Mode System Architecture
**Multi-Controller App Performance Optimization**

## Problem Statement

The Multi-Controller App requires two fundamentally incompatible operational modes:
1. **Development Mode**: Rich tooling with 9 MCP servers (815MB RAM, 1-60s response times)
2. **Hardware Control Mode**: Real-time control (<50ms latency, <150MB RAM)

MCP server overhead makes it impossible to meet hardware control requirements while maintaining development tools active.

## Solution: Dual-Mode Architecture

### System Modes

```rust
pub enum SystemMode {
    Development {
        mcp_servers: Vec<MCPServer>,
        memory_limit: Option<usize>, // None = unlimited
        timeout_default: Duration,    // 60 seconds
    },
    HardwareControl {
        transport_layer: DirectTransport,
        memory_limit: usize,          // 150 MB hard limit
        max_latency: Duration,         // 50ms hard limit
        mcp_servers: Vec<MCPServer>,  // Empty - no MCP active
    }
}
```

## Implementation Strategy

### 1. Configuration-Based Mode Selection

Create `runtime-config.toml`:

```toml
[modes.development]
active = true
mcp_servers = ["cipher-memory", "taskmaster-ai", "desktop-commander", 
               "FileScopeMCP", "clear-thought", "context7", 
               "perplexity-ask", "memory", "time-server"]
memory_limit = "unlimited"
timeout_default = "60s"

[modes.hardware_control]
active = false
mcp_servers = []  # No MCP servers in hardware mode
memory_limit = "150MB"
max_latency = "50ms"
transport = "direct"  # Bypass MCP layer entirely
```

### 2. Runtime Mode Switching

```rust
use crate::config::SystemMode;

pub struct MultiControllerApp {
    mode: SystemMode,
    mcp_manager: Option<MCPManager>,
    transport: Box<dyn Transport>,
}

impl MultiControllerApp {
    pub fn switch_mode(&mut self, new_mode: SystemMode) -> Result<()> {
        match new_mode {
            SystemMode::Development { .. } => {
                // Start MCP servers
                self.mcp_manager = Some(MCPManager::new()?);
                self.mcp_manager.as_mut().unwrap().start_all_servers()?;
                info!("Switched to Development Mode - MCP servers active");
            }
            SystemMode::HardwareControl { .. } => {
                // Shutdown all MCP servers
                if let Some(manager) = self.mcp_manager.take() {
                    manager.shutdown_all()?;
                }
                // Initialize direct transport layer
                self.transport = Box::new(DirectTransport::new()?);
                info!("Switched to Hardware Control Mode - Direct transport active");
            }
        }
        self.mode = new_mode;
        Ok(())
    }
}
```

### 3. Performance Budget Enforcement

```rust
pub struct PerformanceBudget {
    memory_limit: usize,
    latency_limit: Duration,
}

impl PerformanceBudget {
    pub fn enforce(&self) -> Result<()> {
        // Memory check
        let current_memory = get_process_memory()?;
        if current_memory > self.memory_limit {
            return Err(BudgetViolation::Memory {
                current: current_memory,
                limit: self.memory_limit,
            });
        }
        
        // Latency check for hardware operations
        let test_latency = measure_transport_latency()?;
        if test_latency > self.latency_limit {
            return Err(BudgetViolation::Latency {
                current: test_latency,
                limit: self.latency_limit,
            });
        }
        
        Ok(())
    }
}
```

### 4. Transport Layer Abstraction

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> TransportResult<()>;
    async fn send(&mut self, data: &[u8]) -> TransportResult<()>;
    async fn receive(&mut self, timeout: Duration) -> TransportResult<Vec<u8>>;
    async fn cleanup_resources(&mut self) -> TransportResult<()>;
}

// Direct transport for hardware control (no MCP overhead)
pub struct DirectTransport {
    serial: Option<SerialPort>,
    tcp: Option<TcpStream>,
    ssh: Option<SshSession>,
}

impl DirectTransport {
    pub fn new() -> Self {
        Self {
            serial: None,
            tcp: None,
            ssh: None,
        }
    }
    
    // Direct system calls, no JSON serialization
    pub async fn send_hardware_command(&mut self, cmd: &[u8]) -> Result<()> {
        let start = Instant::now();
        
        // Direct write to hardware
        if let Some(serial) = &mut self.serial {
            serial.write_all(cmd)?;
            serial.flush()?;
        }
        
        let latency = start.elapsed();
        assert!(latency < Duration::from_millis(50), "Latency budget exceeded");
        
        Ok(())
    }
}
```

## Mode Transition Workflow

### Development → Hardware Control

1. **Save Development State**
   ```rust
   let dev_state = mcp_manager.save_state()?;
   storage::persist("dev_state.json", &dev_state)?;
   ```

2. **Shutdown MCP Servers**
   ```rust
   mcp_manager.shutdown_all_gracefully(Duration::from_secs(5))?;
   ```

3. **Free Memory**
   ```rust
   memory::force_gc()?;
   memory::trim_working_set()?;
   ```

4. **Initialize Hardware Mode**
   ```rust
   let transport = DirectTransport::new()?;
   transport.initialize_hardware()?;
   ```

5. **Verify Performance**
   ```rust
   assert!(memory::current() < 150_000_000); // 150MB
   assert!(transport.test_latency()? < Duration::from_millis(50));
   ```

### Hardware Control → Development

1. **Safe Hardware Shutdown**
   ```rust
   transport.safe_shutdown_sequence()?;
   ```

2. **Restore Development State**
   ```rust
   let dev_state = storage::load("dev_state.json")?;
   ```

3. **Start MCP Servers**
   ```rust
   mcp_manager = MCPManager::from_state(dev_state)?;
   mcp_manager.start_all_servers()?;
   ```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_mode_switching() {
    let mut app = MultiControllerApp::new();
    
    // Start in development
    app.switch_mode(SystemMode::Development { .. }).unwrap();
    assert!(app.mcp_manager.is_some());
    
    // Switch to hardware
    app.switch_mode(SystemMode::HardwareControl { .. }).unwrap();
    assert!(app.mcp_manager.is_none());
    assert!(memory::current() < 150_000_000);
}

#[test]
fn test_latency_enforcement() {
    let mut transport = DirectTransport::new();
    let start = Instant::now();
    transport.send_hardware_command(b"TEST").unwrap();
    assert!(start.elapsed() < Duration::from_millis(50));
}
```

### Integration Tests
```rust
#[test]
fn test_full_mode_cycle() {
    // Development work
    let mut app = MultiControllerApp::new_development();
    app.run_development_tasks()?;
    
    // Switch to hardware control
    app.switch_to_hardware_mode()?;
    app.control_hardware_device()?;
    
    // Return to development
    app.switch_to_development_mode()?;
    app.continue_development()?;
}
```

## Monitoring & Telemetry

### Mode-Specific Metrics

```rust
pub struct ModeMetrics {
    pub mode: SystemMode,
    pub memory_usage: usize,
    pub active_servers: Vec<String>,
    pub average_latency: Duration,
    pub mode_switches: u32,
    pub budget_violations: Vec<BudgetViolation>,
}

impl ModeMetrics {
    pub fn report(&self) {
        match self.mode {
            SystemMode::Development { .. } => {
                info!("Development Mode: {} servers, {}MB RAM", 
                      self.active_servers.len(), 
                      self.memory_usage / 1_000_000);
            }
            SystemMode::HardwareControl { .. } => {
                info!("Hardware Mode: {}ms latency, {}MB RAM",
                      self.average_latency.as_millis(),
                      self.memory_usage / 1_000_000);
            }
        }
    }
}
```

## Benefits

1. **Performance Compliance**: Hardware control meets <50ms latency, <150MB RAM
2. **Developer Experience**: Full MCP tooling available during development
3. **Clear Separation**: No performance interference between modes
4. **Predictable Behavior**: Deterministic timing in hardware mode
5. **Scalability**: Easy to add new modes or adjust budgets

## Implementation Timeline

- **Phase 1** (Immediate): Configuration files and mode enum
- **Phase 2** (This Week): Mode switching logic and MCP management
- **Phase 3** (Next Week): Direct transport implementation
- **Phase 4** (Testing): Comprehensive testing and validation

## Success Criteria

- [ ] Hardware mode uses <150MB RAM consistently
- [ ] Hardware operations complete in <50ms
- [ ] Mode switching takes <5 seconds
- [ ] No MCP servers active in hardware mode
- [ ] Development mode maintains all current capabilities
- [ ] 8+ hour soak test passes in both modes

---

*This architecture ensures the Multi-Controller App can excel at both development productivity and real-time hardware control without compromise.*