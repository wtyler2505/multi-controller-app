# Multi-Controller App - Scripting Guide

## Overview

The Multi-Controller App provides a safe, sandboxed scripting environment using the Rhai scripting language. Scripts can automate device operations while being protected by strict security controls and resource limits.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Script Syntax](#script-syntax)
3. [Available API](#available-api)
4. [Security & Sandboxing](#security--sandboxing)
5. [Resource Limits](#resource-limits)
6. [Examples](#examples)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## Getting Started

### Running a Script

Scripts can be executed through the Scripts tab in the UI or programmatically:

```rust
// Load and run a script file
let script = std::fs::read_to_string("scripts/my_automation.rhai")?;
engine.eval(&script).await?;
```

### Basic Script Structure

```rhai
// Get a device
let device = get_device("arduino_uno");

// Check if device exists
if device == () {
    print("Device not found!");
    return;
}

// Perform operations
let value = read(device, "sensor/temperature");
print("Temperature: " + value);

// Control outputs
write(device, "gpio/13", 1);  // Set pin HIGH
sleep_ms(1000);
write(device, "gpio/13", 0);  // Set pin LOW
```

## Script Syntax

Rhai uses a JavaScript-like syntax with Rust influences:

### Variables
```rhai
let x = 42;                    // Immutable by default
let mut counter = 0;           // Mutable variable
const MAX_VALUE = 100;         // Constant
```

### Data Types
```rhai
let number = 42;               // Integer
let decimal = 3.14;           // Float
let text = "Hello";           // String
let flag = true;              // Boolean
let list = [1, 2, 3];        // Array
let map = #{ x: 10, y: 20 }; // Object map
```

### Control Flow
```rhai
// If-else
if temperature > 25 {
    print("Too hot!");
} else if temperature < 20 {
    print("Too cold!");
} else {
    print("Just right!");
}

// Loops
for i in 0..10 {
    print("Count: " + i);
}

while condition {
    // Do something
}

// Loop with break
loop {
    if done { break; }
}
```

### Functions
```rhai
fn calculate_average(values) {
    let sum = 0;
    for v in values {
        sum += v;
    }
    return sum / values.len();
}

let avg = calculate_average([10, 20, 30]);
```

## Available API

### Device Management

#### `list_devices() -> Array<String>`
Returns a list of available device IDs.

```rhai
let devices = list_devices();
for device_id in devices {
    print("Found: " + device_id);
}
```

#### `get_device(id: String) -> Device`
Gets a handle to a specific device.

```rhai
let arduino = get_device("arduino_uno");
```

### Device Operations

#### `read(device: Device, endpoint: String) -> String`
Reads data from a device endpoint.

```rhai
let value = read(device, "analog/A0");
let state = read(device, "gpio/2");
```

#### `write(device: Device, endpoint: String, value: Dynamic) -> String`
Writes data to a device endpoint.

```rhai
write(device, "gpio/13", 1);      // Digital write
write(device, "pwm/9", 128);      // PWM write
write(device, "serial", "Hello"); // Serial write
```

#### `send_command(device: Device, command: String) -> Dynamic`
Sends a command to the device.

```rhai
let result = send_command(device, "reset");
```

#### `wait_for_event(device: Device, event: String, timeout_ms: Int) -> Dynamic`
Waits for a device event with timeout.

```rhai
let event = wait_for_event(device, "button_press", 5000);
```

### Utility Functions

#### `print(message: String)`
Outputs a message to the log.

```rhai
print("Script started");
```

#### `sleep_ms(milliseconds: Int)`
Pauses execution for specified milliseconds.

```rhai
sleep_ms(1000);  // Wait 1 second
```

#### `timestamp() -> Int`
Returns current timestamp in milliseconds.

```rhai
let start = timestamp();
// ... do work ...
let elapsed = timestamp() - start;
print("Took " + elapsed + "ms");
```

## Security & Sandboxing

### What's Blocked

The sandbox prevents dangerous operations:

- ❌ **File System Access**: No reading/writing files
- ❌ **Network Access**: No HTTP requests or sockets
- ❌ **System Commands**: No executing shell commands
- ❌ **Dynamic Code**: No `eval()` or code generation
- ❌ **Module Loading**: No `import` or `require`
- ❌ **Raw Memory**: No pointer manipulation

### What's Allowed

Safe operations are permitted:

- ✅ Device read/write operations (with permissions)
- ✅ Mathematical calculations
- ✅ String manipulation
- ✅ Array and object operations
- ✅ Control flow (loops, conditions)
- ✅ Function definitions
- ✅ Limited sleep operations

### Permission Levels

Scripts operate under configurable permission levels:

1. **High Security** (Untrusted Scripts)
   - Read-only device access
   - Strict resource limits
   - Minimal operation set

2. **Default** (Normal Scripts)
   - Read/write device access
   - Standard resource limits
   - Full safe operation set

3. **Development** (Trusted Scripts)
   - Full device control
   - Relaxed limits
   - Debug output enabled

## Resource Limits

Scripts are constrained by configurable limits:

| Resource | Default | Strict | Relaxed |
|----------|---------|--------|---------|
| Max Operations | 100,000 | 10,000 | 1,000,000 |
| Max Execution Time | 10s | 1s | 60s |
| Max Memory | 10 MB | 1 MB | 100 MB |
| Max String Size | 1 MB | 10 KB | 10 MB |
| Max Array Size | 10,000 | 1,000 | 100,000 |
| Max Call Depth | 32 | 8 | 64 |
| Max Loop Iterations | 10,000 | 1,000 | 100,000 |

## Examples

### Example 1: Blink LED
```rhai
// Blink LED 10 times
let device = get_device("arduino_uno");

for i in 0..10 {
    write(device, "gpio/13", 1);
    sleep_ms(500);
    write(device, "gpio/13", 0);
    sleep_ms(500);
}
```

### Example 2: Read Sensor Data
```rhai
// Collect temperature readings
let device = get_device("arduino_uno");
let readings = [];

for i in 0..60 {
    let temp = read(device, "analog/A0");
    readings.push(temp);
    sleep_ms(1000);  // Once per second
}

print("Collected " + readings.len() + " readings");
```

### Example 3: Conditional Control
```rhai
// Temperature-based fan control
let device = get_device("arduino_uno");
const THRESHOLD = 25.0;

loop {
    let temp = read(device, "temperature");
    
    if temp > THRESHOLD {
        write(device, "fan", 1);  // Turn on fan
        print("Fan ON - Temp: " + temp);
    } else {
        write(device, "fan", 0);  // Turn off fan
        print("Fan OFF - Temp: " + temp);
    }
    
    sleep_ms(5000);  // Check every 5 seconds
}
```

### Example 4: Event Handling
```rhai
// Wait for button press
let device = get_device("arduino_uno");

print("Waiting for button press...");
let event = wait_for_event(device, "button_press", 30000);

if event != () {
    print("Button pressed!");
    write(device, "led", 1);
    sleep_ms(2000);
    write(device, "led", 0);
} else {
    print("Timeout - no button press");
}
```

## Best Practices

### 1. Always Check Device Availability
```rhai
let device = get_device("device_id");
if device == () {
    print("Device not found!");
    return;
}
```

### 2. Handle Errors Gracefully
```rhai
let result = write(device, "gpio/13", 1);
if result != "OK" {
    print("Write failed: " + result);
}
```

### 3. Use Constants for Configuration
```rhai
const SENSOR_PIN = "analog/A0";
const SAMPLE_RATE = 1000;  // ms
const MAX_SAMPLES = 100;
```

### 4. Add Logging for Debugging
```rhai
print("Starting operation...");
// ... operation code ...
print("Operation complete");
```

### 5. Clean Up on Exit
```rhai
// Ensure outputs are in safe state
write(device, "motor", 0);
write(device, "heater", 0);
print("Cleanup complete");
```

## Troubleshooting

### Common Issues

#### Script Won't Compile
- Check syntax errors (missing semicolons, brackets)
- Verify variable names are defined
- Ensure strings are properly quoted

#### Device Not Found
- Verify device ID is correct
- Ensure device is connected
- Check device permissions

#### Operation Timeout
- Script may exceed execution time limit
- Add `sleep_ms()` in tight loops
- Reduce operation count

#### Resource Limit Exceeded
- Reduce array sizes
- Limit loop iterations
- Free unused variables

### Debug Tips

1. **Enable debug output** in development mode
2. **Add print statements** to track execution
3. **Use smaller test cases** first
4. **Check resource usage** with monitoring
5. **Review sandbox configuration** for limits

## Advanced Topics

### Custom Functions
```rhai
fn map_value(x, in_min, in_max, out_min, out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

// Map 0-1023 analog input to 0-255 PWM output
let analog = read(device, "analog/A0");
let pwm = map_value(analog, 0, 1023, 0, 255);
write(device, "pwm/9", pwm);
```

### State Machines
```rhai
let state = "IDLE";
let device = get_device("arduino_uno");

loop {
    switch state {
        "IDLE" => {
            if read(device, "button") == "1" {
                state = "RUNNING";
                print("Starting...");
            }
        }
        "RUNNING" => {
            // Do work
            write(device, "led", 1);
            
            if read(device, "stop") == "1" {
                state = "IDLE";
                print("Stopping...");
            }
        }
    }
    
    sleep_ms(100);
}
```

## Security Considerations

1. **Never trust user scripts** - Always run in sandbox
2. **Validate all inputs** - Check ranges and types
3. **Limit resource usage** - Prevent DoS attacks
4. **Audit script access** - Log who runs what
5. **Review scripts** before running in production
6. **Use minimal permissions** required for task
7. **Monitor execution** for anomalies

## API Reference Summary

| Function | Description | Returns |
|----------|-------------|---------|
| `list_devices()` | Get all device IDs | Array<String> |
| `get_device(id)` | Get device handle | Device |
| `read(device, endpoint)` | Read from device | String |
| `write(device, endpoint, value)` | Write to device | String |
| `send_command(device, cmd)` | Send command | Dynamic |
| `wait_for_event(device, event, timeout)` | Wait for event | Dynamic |
| `print(message)` | Output to log | void |
| `sleep_ms(ms)` | Pause execution | void |
| `timestamp()` | Get current time | Int |

---

For more information, see the [examples](../scripts/examples/) directory or consult the main documentation.