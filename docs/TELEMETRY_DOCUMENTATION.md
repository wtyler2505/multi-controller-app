# Telemetry System Documentation

## Overview

The Multi-Controller App includes a comprehensive telemetry system for capturing, storing, and analyzing real-time data from device I/O and events. The system is designed for high performance, thread safety, and memory efficiency.

## Architecture

### Core Components

1. **Ring Buffer** (`src/telemetry/ring_buffer.rs`)
   - Lock-free, thread-safe circular buffer
   - Fixed-size with automatic overwrite of oldest data
   - Minimum capacity of 2,000 samples per requirements
   - Atomic operations for write position tracking
   - Support for concurrent readers and single writer

2. **Sample Types** (`src/telemetry/sample.rs`)
   - Multiple data types: Float32, Float64, Int32, UInt32, Bool, String, Bytes, Vector
   - Rich metadata support (source, unit, quality, tags)
   - Automatic timestamping in milliseconds
   - Statistics calculation (min, max, mean, std dev)

3. **Channels** (`src/telemetry/channel.rs`)
   - Named telemetry streams with independent buffers
   - Configurable sample rates with rate limiting
   - Type safety enforcement
   - Memory usage tracking
   - Data decimation for visualization

4. **Export/Import** (`src/telemetry/export.rs`)
   - Multiple formats: JSON, CSV, Binary, MessagePack
   - Optional compression with GZIP
   - Batch export for multiple channels
   - Full round-trip import capability

5. **System Manager** (`src/telemetry/mod.rs`)
   - Central coordination of multiple channels
   - Global memory management
   - Automatic pruning to enforce memory limits
   - System-wide statistics and monitoring

## Usage Examples

### Basic Usage

```rust
use multi_controller_app::telemetry::{TelemetrySystem, TelemetrySample};

// Create telemetry system
let system = TelemetrySystem::new();

// Create a channel for temperature data
let temp_channel = system.create_channel(
    "temperature".to_string(),
    Some(ChannelConfig {
        name: "temperature".to_string(),
        buffer_size: 5000,  // 5000 samples
        sample_rate: 10.0,  // 10 Hz
        sample_type: SampleType::Float32,
    })
);

// Add samples
temp_channel.add_sample(TelemetrySample::new_f32(23.5));
temp_channel.add_sample(TelemetrySample::with_metadata(
    SampleValue::Float32(24.1),
    SampleMetadata::default()
        .source("Arduino".to_string())
        .unit("°C".to_string())
        .quality(0.95)
));

// Get data for charting
let chart_data = temp_channel.chart_data(100);  // Max 100 points

// Export data
let exporter = TelemetryExporter::new().with_compression(true);
let json_data = exporter.export(
    &temp_channel.export_data(),
    ExportFormat::JsonPretty
)?;
```

### Advanced Features

#### Memory Management

```rust
// Configure with memory limits
let config = TelemetryConfig {
    default_buffer_size: 2000,
    max_memory_bytes: 10 * 1024 * 1024,  // 10MB
    auto_memory_management: true,
    default_sample_rate: 30.0,
};

let system = TelemetrySystem::with_config(config);

// System automatically prunes oldest data when limit exceeded
system.enforce_memory_limits();
```

#### Batch Operations

```rust
// Add multiple samples efficiently
let samples = vec![
    TelemetrySample::new_f32(1.0),
    TelemetrySample::new_f32(2.0),
    TelemetrySample::new_f32(3.0),
];
channel.add_samples(samples);

// Export all channels
let all_data = system.export_all(ExportFormat::Csv)?;
```

#### Statistics and Analysis

```rust
// Get channel statistics
let stats = channel.get_stats();
println!("Total samples: {}", stats.total_samples);
println!("Drop rate: {:.2}%", stats.drop_rate());
println!("Effective rate: {:.2} Hz", stats.effective_sample_rate().unwrap_or(0.0));

// Calculate sample statistics
let samples = channel.snapshot();
let statistics = SampleStatistics::from_samples(&samples);
println!("Mean: {:.2}", statistics.mean.unwrap_or(0.0));
println!("Std Dev: {:.2}", statistics.std_dev.unwrap_or(0.0));
```

## Performance Characteristics

### Memory Usage

- **Per Sample Overhead**: ~40 bytes (timestamp + value + metadata pointer)
- **Ring Buffer**: Pre-allocated, no dynamic allocation during operation
- **Channel Overhead**: ~200 bytes + buffer size
- **Compression**: Typically 50-70% reduction for exports

### Throughput

- **Write Performance**: >100,000 samples/second (single thread)
- **Read Performance**: Snapshot creation is O(n) where n = buffer size
- **Export Performance**: ~1MB/second for JSON, ~10MB/second for binary

### Thread Safety

- **Multiple Readers**: Supported via RwLock
- **Single Writer**: Lock-free using atomic operations
- **Channel Operations**: All operations are thread-safe

## Configuration

### System Configuration

```rust
pub struct TelemetryConfig {
    /// Default buffer size for new channels (minimum 2000)
    pub default_buffer_size: usize,
    
    /// Maximum total memory allowed for telemetry (bytes)
    pub max_memory_bytes: usize,
    
    /// Enable automatic memory management
    pub auto_memory_management: bool,
    
    /// Default sample rate (Hz) for channels
    pub default_sample_rate: f32,
}
```

### Channel Configuration

```rust
pub struct ChannelConfig {
    /// Channel name
    pub name: String,
    
    /// Buffer size (minimum 2000 samples)
    pub buffer_size: usize,
    
    /// Expected sample rate (Hz) for rate limiting
    pub sample_rate: f32,
    
    /// Sample type for this channel
    pub sample_type: SampleType,
}
```

## Integration with UI

The telemetry system integrates with the existing `TelemetryPanel` in the UI:

```rust
// In UI update loop
let telemetry_channel = system.get_channel("device_telemetry").unwrap();
let chart_data = telemetry_channel.chart_data(200);  // Get last 200 points

// Update UI chart
for (timestamp, value) in chart_data {
    telemetry_panel.add_data_point(timestamp, value);
}

// Display statistics
let stats = telemetry_channel.get_stats();
ui.label(format!("Buffer: {:.0}%", stats.buffer_fill_ratio * 100.0));
ui.label(format!("Rate: {:.1} Hz", stats.effective_sample_rate().unwrap_or(0.0)));
```

## Testing

The telemetry system includes comprehensive unit tests:

### Ring Buffer Tests
- Basic operations (push, snapshot, clear)
- Wraparound behavior
- Concurrent access
- Batch operations
- Pruning functionality

### Channel Tests
- Sample addition and retrieval
- Rate limiting
- Type checking
- Chart data generation

### Export/Import Tests
- All format types (JSON, CSV, Binary, MessagePack)
- Compression/decompression
- Round-trip integrity
- Multi-channel export

Run tests with:
```bash
cargo test telemetry
```

## Best Practices

1. **Buffer Sizing**: Choose buffer sizes based on expected data rate and retention needs
   - High-frequency data: Larger buffers (10,000+ samples)
   - Low-frequency data: Smaller buffers (2,000-5,000 samples)

2. **Rate Limiting**: Set appropriate sample rates to prevent overwhelming the system
   - UI updates: 30-60 Hz
   - Sensor data: Match sensor capabilities
   - Event logging: No rate limiting

3. **Memory Management**: Monitor memory usage and configure limits appropriately
   - Enable auto-management for production
   - Set reasonable max_memory_bytes
   - Use pruning for long-running applications

4. **Export Strategy**: Choose export formats based on use case
   - JSON: Human-readable, debugging
   - CSV: Spreadsheet analysis
   - Binary/MessagePack: Efficient storage and transmission

5. **Metadata Usage**: Include relevant metadata for analysis
   - Source identification
   - Units of measurement
   - Quality indicators
   - Contextual tags

## Troubleshooting

### Common Issues

1. **High Memory Usage**
   - Enable auto memory management
   - Reduce buffer sizes
   - Increase pruning frequency
   - Use compression for exports

2. **Sample Loss**
   - Check rate limiting settings
   - Monitor drop statistics
   - Increase buffer size if needed
   - Consider batch operations

3. **Performance Issues**
   - Use batch operations for multiple samples
   - Decimate data for visualization
   - Export in binary format for large datasets
   - Enable compression for network transfers

## Future Enhancements

Potential improvements for future versions:

1. **Advanced Algorithms**
   - LTTB decimation for better visualization
   - Real-time anomaly detection
   - Predictive analytics

2. **Storage Backend**
   - Time-series database integration
   - Persistent storage options
   - Cloud telemetry services

3. **Visualization**
   - Multiple chart types
   - Real-time streaming updates
   - Interactive zoom and pan

4. **Analysis Tools**
   - FFT for frequency analysis
   - Correlation analysis
   - Pattern recognition

---

*Last Updated: 2025-09-05*  
*Version: 1.0.0*