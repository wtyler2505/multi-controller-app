// Example: Blink LED on Arduino
// This script demonstrates basic device control with safety limits

// Get the Arduino device
let arduino = get_device("arduino_uno");

// Check if device is available
if arduino == () {
    print("Error: Arduino not found!");
    return;
}

print("Starting LED blink sequence...");

// Blink LED 10 times (resource limited)
for i in 0..10 {
    // Turn LED on (pin 13)
    let result = write(arduino, "gpio/13", 1);
    if result != "OK" {
        print("Error writing to pin 13: " + result);
        break;
    }
    
    print("LED ON - iteration " + (i + 1));
    sleep_ms(500);
    
    // Turn LED off
    write(arduino, "gpio/13", 0);
    print("LED OFF");
    sleep_ms(500);
}

print("Blink sequence complete!");