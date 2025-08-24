# Multi-Controller App - Architecture Reference

## System Architecture

The application follows a modular, plugin-based architecture with clear separation of concerns between UI, device management, transport, and control logic.

## Core Components

### 1. UI Layer

- **Technology**: WPF/WinUI 3 (C#) or egui/Win32 (Rust)
- **Structure**: Single-window application with sidebar and tabs
- **Tabs**: Devices, Manual Controls, Scripts, Telemetry, Logs, Profiles
- **Interaction**: Message bus pattern for UI-to-core communication
- **Performance**: Must maintain 30 FPS for telemetry updates

### 2. Device Manager

- **Purpose**: Central orchestrator for device lifecycle
- **Responsibilities**:
  - Device discovery and enumeration
  - Connection state management
  - Session registry maintenance
  - Hot-plug event handling
  - Driver coordination

### 3. Driver Registry

- **Location**: `/drivers/<driver-name>/`
- **Interface**: `IDeviceDriver` and `IDeviceSession`
- **Manifest**: JSON/TOML describing capabilities
- **Loading**: Dynamic plugin loading at runtime
- **Examples**: Arduino, ESP32, RioRand drivers

### 4. Transport Layer

- **Protocols**: Serial (COM), TCP/UDP, SSH
- **Features**:
  - Async I/O operations
  - Exponential backoff reconnection
  - Connection pooling
  - Latency monitoring
- **Libraries**:
  - C#: System.IO.Ports, System.Net.Sockets, SSH.NET
  - Rust: serialport-rs, tokio, thrussh

### 5. Scripting Engine

- **Runtime**: JavaScript (preferred), Lua, or Python
- **Sandbox**: Restricted file/network access
- **API Surface**:
  ```javascript
  devices.list(); // Get available devices
  dev.call(endpoint, args); // Send control command
  dev.subscribe(stream); // Subscribe to telemetry
  dev.unsubscribe(stream); // Unsubscribe from telemetry
  ```

### 6. Telemetry Engine

- **Buffer**: Fixed-size ring buffer (2000+ samples)
- **Decimation**: Automatic data reduction for display
- **Performance**: 30 FPS rendering target
- **Memory**: Bounded growth with automatic pruning

### 7. Profile Manager

- **Format**: JSON or TOML configuration files
- **Features**:
  - Save/load device configurations
  - Hot-reload support
  - Schema validation
  - Version migration

### 8. Logger

- **Types**: Device I/O logs, Application events
- **Storage**: Rolling file buffers
- **Export**: One-click log export
- **Retention**: Configurable size/time limits

## Key Interfaces

### IDeviceDriver

```csharp
interface IDeviceDriver {
    string Name { get; }
    string[] SupportedTransports { get; }
    Task<bool> ProbeAsync(ITransport transport);
    Task<IDeviceSession> OpenAsync(ITransport transport);
}
```

### IDeviceSession

```csharp
interface IDeviceSession {
    Task<object> InvokeAsync(string endpoint, object[] args);
    Task<IDisposable> SubscribeAsync(string stream, Action<byte[]> handler);
    Task CloseAsync();
}
```

## Communication Patterns

### Device Discovery Flow

1. UI requests discovery start
2. Device Manager enumerates available ports
3. For each port, Driver Registry attempts probe
4. Successful probes create device sessions
5. Device list updates in UI

### Control Command Flow

1. User interaction in Manual Controls tab
2. UI sends command via message bus
3. Device Manager routes to appropriate session
4. Session sends via transport layer
5. Response flows back through chain
6. UI updates with result/status

### Telemetry Stream Flow

1. Device emits telemetry data
2. Transport layer receives bytes
3. Session parses and forwards to Telemetry Engine
4. Telemetry Engine decimates and buffers
5. UI renders at 30 FPS

## Safety Architecture

### Global Stop Mechanism

- **Location**: Prominent UI button
- **Action**: Immediate cessation of all outputs
- **Implementation**: Direct hardware reset where possible
- **Recovery**: Graceful state restoration

### Rate Limiting

- **Per-Device**: Configurable command rate limits
- **Global**: System-wide throughput caps
- **Ramp Rates**: Gradual value changes for safety

### Connection Resilience

- **Hot-Plug**: Automatic detection and recovery
- **Backoff**: Exponential retry with jitter
- **Cleanup**: Proper resource disposal on disconnect

## Performance Considerations

### Memory Management

- **Ring Buffers**: Fixed-size for telemetry
- **Object Pooling**: Reuse for frequent allocations
- **Weak References**: For cache management
- **Native AOT**: Reduced runtime overhead (C# path)

### Async Operations

- **Transport I/O**: All async/await or tokio
- **UI Updates**: Dispatcher/message queue
- **Script Execution**: Separate thread/context
- **Telemetry Processing**: Background workers

### Resource Budgets

- **Startup**: Resource loading deferred where possible
- **Idle**: Event-driven, no polling loops
- **Active**: Throttled update rates, bounded buffers

## Extension Points

### Adding New Drivers

1. Create `/drivers/<name>/` directory
2. Implement IDeviceDriver interface
3. Add manifest.json with metadata
4. Driver auto-discovered on startup

### Custom Scripts

1. Place in `/scripts/` directory
2. Access via Scripts tab in UI
3. Use provided API for device control

### UI Customization

1. Modify XAML/UI definitions
2. Add custom controls to tabs
3. Extend message bus for new commands
