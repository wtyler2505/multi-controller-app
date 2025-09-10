// Example: Read multiple sensors and log data
// Demonstrates safe reading and data collection

// Get device handle
let device = get_device("arduino_uno");

if device == () {
    print("Error: Device not connected");
    return;
}

// Sensor endpoints to monitor
let sensors = [
    "analog/A0",  // Temperature sensor
    "analog/A1",  // Light sensor  
    "analog/A2",  // Humidity sensor
];

// Collection parameters
let samples = 10;
let interval_ms = 1000;

print("Starting sensor data collection...");
print("Collecting " + samples + " samples at " + interval_ms + "ms intervals");

// Collect data (resource limited by sandbox)
let data = [];

for sample in 0..samples {
    print("\nSample " + (sample + 1) + ":");
    
    let readings = #{};
    
    for sensor in sensors {
        let value = read(device, sensor);
        readings[sensor] = value;
        print("  " + sensor + ": " + value);
    }
    
    // Store timestamp with readings
    readings["timestamp"] = timestamp();
    data.push(readings);
    
    // Wait before next sample
    if sample < samples - 1 {
        sleep_ms(interval_ms);
    }
}

print("\nData collection complete!");
print("Collected " + data.len() + " samples");

// Calculate averages (if numeric values)
// Note: In production, values would be actual numbers
print("\nSummary statistics would be calculated here");