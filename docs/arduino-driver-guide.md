# Arduino Driver Guide

## Overview

The Multi-Controller App Arduino driver provides support for Arduino Uno, Mega, and Nano boards through a lightweight custom protocol. This driver enables digital I/O, analog reading, PWM control, and hall sensor integration.

## Supported Boards

- **Arduino Uno**: Official and clone boards (CH340, FTDI chips)
- **Arduino Mega**: Extended I/O capabilities  
- **Arduino Nano**: Compact form factor

## Features

### Core Capabilities
- Digital I/O (read/write)
- Analog input (10-bit ADC, 0-1023)
- PWM output (8-bit, 0-255)
- Hall sensor support (RPM and pulse counting)
- 115200 baud serial communication
- USB VID/PID auto-detection

### USB Detection
The driver automatically detects Arduino devices by USB vendor/product IDs:
- `0x2341`: Official Arduino boards
- `0x1A86`: CH340-based clones
- `0x0403`: FTDI-based boards

## Protocol Specification

### Command Format
```
COMMAND [ARGS]\r\n
```

### Response Format
```
OK\r\n                    # Success
ERROR: message\r\n        # Error with description
VALUE:data\r\n           # Data response
RPM:float\r\n            # RPM reading
COUNT:integer\r\n        # Counter value
```

### Commands

| Command | Arguments | Response | Description |
|---------|-----------|----------|-------------|
| `PROBE` | none | `ARDUINO_UNO_V1` | Identify device |
| `PIN_MODE` | pin, mode | `OK` | Set pin mode (INPUT/OUTPUT/PWM/ANALOG) |
| `DIGITAL_WRITE` | pin, value | `OK` | Write digital value (0/1) |
| `DIGITAL_READ` | pin | `VALUE:0/1` | Read digital input |
| `ANALOG_READ` | pin | `VALUE:0-1023` | Read analog value |
| `PWM_WRITE` | pin, duty | `OK` | Set PWM duty cycle (0-255) |
| `HALL_CONFIG` | pin, mode | `OK` | Configure hall sensor |
| `HALL_READ` | pin | `RPM:float` | Read hall sensor RPM |
| `HALL_COUNT` | pin | `COUNT:int` | Read pulse count |
| `HALL_RESET` | pin | `OK` | Reset pulse counter |

## API Usage

### Session Endpoints

The driver exposes the following endpoints through the DeviceSession interface:

```rust
// Pin configuration
"pinMode" -> args: [pin: u8, mode: "INPUT"|"OUTPUT"|"PWM"|"ANALOG"]

// Digital I/O
"digitalWrite" -> args: [pin: u8, value: bool]
"digitalRead" -> args: [pin: u8] -> returns: {value: bool}

// Analog I/O
"analogRead" -> args: [pin: u8] -> returns: {value: u16}
"pwmWrite" -> args: [pin: u8, duty: u8]

// Hall sensor
"configureHallSensor" -> args: [pin: u8, mode: "RISING"|"FALLING"|"BOTH"]
"readHallRPM" -> args: [pin: u8] -> returns: {rpm: f32}
"readHallCounter" -> args: [pin: u8] -> returns: {count: u32}
"resetHallCounter" -> args: [pin: u8]
```

## Pin Mappings

### Arduino Uno
- **Digital I/O**: Pins 0-13
- **PWM**: Pins 3, 5, 6, 9, 10, 11
- **Analog Input**: A0-A5 (pins 0-5 in analog context)
- **Interrupts**: Pins 2, 3 (for hall sensors)

### Arduino Mega
- **Digital I/O**: Pins 0-53
- **PWM**: Pins 2-13, 44-46
- **Analog Input**: A0-A15
- **Interrupts**: Pins 2, 3, 18, 19, 20, 21

## Performance

- **Latency**: 50ms typical for command/response
- **Baud Rate**: 115200 (8N1)
- **Max Update Rate**: ~20 Hz for continuous operations
- **Response Timeout**: 500ms

## Current Limitations

⚠️ **Transport Architecture Issue**: The current implementation simulates responses due to a design limitation in the Transport trait. The trait expects `Arc<dyn Transport>` but requires `&mut self` for send/receive operations. This architectural issue needs to be addressed in the transport layer for full functionality.

## Future Enhancements

### Firmata Protocol Support
The driver is structured to support Firmata protocol in future versions:
- Protocol abstraction layer ready for Firmata integration
- Maintains backward compatibility with custom protocol
- Planned support for StandardFirmata v2.5+

### Additional Features
- I2C communication
- SPI support
- Servo control
- Interrupt handling
- Multi-device coordination

## Troubleshooting

### Device Not Detected
1. Check USB cable connection
2. Verify Arduino appears in Device Manager (Windows) or `/dev/tty*` (Linux/Mac)
3. Ensure correct drivers installed for clone boards (CH340/FTDI)
4. Try different USB port

### Communication Errors
1. Verify 115200 baud rate setting
2. Check Arduino firmware supports custom protocol
3. Ensure no other application using serial port
4. Reset Arduino board

### Performance Issues
1. Reduce command frequency for continuous operations
2. Use PWM for smooth analog outputs instead of rapid digital writes
3. Implement local buffering for high-frequency data

## Example Arduino Firmware

To use this driver, upload the following sketch to your Arduino:

```cpp
void setup() {
  Serial.begin(115200);
  Serial.setTimeout(50);
}

void loop() {
  if (Serial.available()) {
    String command = Serial.readStringUntil('\n');
    command.trim();
    
    if (command == "PROBE") {
      Serial.println("ARDUINO_UNO_V1");
    }
    else if (command.startsWith("PIN_MODE ")) {
      // Parse and set pin mode
      Serial.println("OK");
    }
    else if (command.startsWith("DIGITAL_WRITE ")) {
      // Parse and write digital value
      Serial.println("OK");
    }
    else if (command.startsWith("DIGITAL_READ ")) {
      // Parse and read digital value
      int value = digitalRead(pin);
      Serial.print("VALUE:");
      Serial.println(value);
    }
    // ... implement other commands
  }
}
```

## Reference

- [Arduino Serial Communication](https://www.arduino.cc/reference/en/language/functions/communication/serial/)
- [USB VID/PID Database](https://devicehunt.com/all-usb-vendors)
- [Firmata Protocol](https://github.com/firmata/protocol)