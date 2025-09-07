// Example: Temperature Control System
// Demonstrates conditional logic, events, and safety limits

// Configuration
const TARGET_TEMP = 25.0;  // Target temperature in Celsius
const TOLERANCE = 1.0;      // Acceptable deviation
const MAX_RUNTIME = 60000;  // Maximum runtime in milliseconds (1 minute)

// Get device
let controller = get_device("arduino_uno");

if controller == () {
    print("Error: Temperature controller not found");
    return;
}

print("Temperature Control System");
print("Target: " + TARGET_TEMP + "°C ± " + TOLERANCE + "°C");

// Control loop (limited by sandbox timeout)
let start_time = timestamp();
let iterations = 0;
let max_iterations = 100;  // Safety limit

while iterations < max_iterations {
    iterations += 1;
    
    // Check runtime limit
    if (timestamp() - start_time) > MAX_RUNTIME {
        print("Maximum runtime reached - shutting down");
        break;
    }
    
    // Read current temperature
    let temp_reading = read(controller, "analog/A0");
    
    // In production, this would be actual temperature conversion
    // For demo, we simulate with mock data
    let current_temp = 20.0 + (iterations * 0.5);  // Simulated warming
    
    print("Iteration " + iterations + ": Current temp = " + current_temp + "°C");
    
    // Control logic
    if current_temp < (TARGET_TEMP - TOLERANCE) {
        // Too cold - turn on heater
        write(controller, "gpio/8", 1);  // Heater ON
        write(controller, "gpio/9", 0);  // Cooler OFF
        print("  → Heating...");
        
    } else if current_temp > (TARGET_TEMP + TOLERANCE) {
        // Too hot - turn on cooler
        write(controller, "gpio/8", 0);  // Heater OFF
        write(controller, "gpio/9", 1);  // Cooler ON
        print("  → Cooling...");
        
    } else {
        // Within range - maintain
        write(controller, "gpio/8", 0);  // Heater OFF
        write(controller, "gpio/9", 0);  // Cooler OFF
        print("  ✓ Temperature OK");
        
        // In production, might wait here
        // For demo, we exit after reaching target
        print("\nTarget temperature achieved!");
        break;
    }
    
    // Control loop delay
    sleep_ms(500);
}

// Ensure everything is turned off at exit
write(controller, "gpio/8", 0);  // Heater OFF
write(controller, "gpio/9", 0);  // Cooler OFF

print("Temperature control ended after " + iterations + " iterations");
print("Runtime: " + (timestamp() - start_time) + "ms");