// Multi-Controller App Test Sketch for Arduino
// This sketch implements a simple command protocol for testing
// the Rust driver implementation

const int LED_PIN = 13;  // Built-in LED on most Arduino boards
String inputBuffer = "";
bool echoMode = false;

void setup() {
  Serial.begin(115200);
  pinMode(LED_PIN, OUTPUT);
  digitalWrite(LED_PIN, LOW);
  
  // Send identification on startup
  delay(100);  // Small delay for serial stability
}

void loop() {
  // Check for incoming serial data
  while (Serial.available() > 0) {
    char inChar = Serial.read();
    
    // Check for command terminator
    if (inChar == '\n') {
      // Process the complete command
      processCommand(inputBuffer);
      inputBuffer = "";  // Clear buffer for next command
    } else if (inChar != '\r') {
      // Add character to buffer (ignore carriage return)
      inputBuffer += inChar;
    }
  }
}

void processCommand(String command) {
  command.trim();  // Remove any whitespace
  
  // Echo mode for debugging
  if (echoMode) {
    Serial.print("ECHO:");
    Serial.println(command);
  }
  
  // Parse and execute commands
  if (command == "PROBE") {
    Serial.println("ARDUINO_UNO");
  }
  else if (command.startsWith("LED_ON")) {
    digitalWrite(LED_PIN, HIGH);
    Serial.println("OK");
  }
  else if (command.startsWith("LED_OFF")) {
    digitalWrite(LED_PIN, LOW);
    Serial.println("OK");
  }
  else if (command.startsWith("LED_STATE")) {
    int state = digitalRead(LED_PIN);
    Serial.print("STATE:");
    Serial.println(state);
  }
  else if (command.startsWith("PIN_MODE ")) {
    // Format: PIN_MODE <pin> <mode>
    // Example: PIN_MODE 7 OUTPUT
    int spacePos = command.indexOf(' ', 9);
    if (spacePos > 0) {
      int pin = command.substring(9, spacePos).toInt();
      String mode = command.substring(spacePos + 1);
      
      if (mode == "OUTPUT") {
        pinMode(pin, OUTPUT);
        Serial.println("OK");
      } else if (mode == "INPUT") {
        pinMode(pin, INPUT);
        Serial.println("OK");
      } else if (mode == "INPUT_PULLUP") {
        pinMode(pin, INPUT_PULLUP);
        Serial.println("OK");
      } else {
        Serial.println("ERROR:Invalid mode");
      }
    } else {
      Serial.println("ERROR:Invalid format");
    }
  }
  else if (command.startsWith("DIGITAL_WRITE ")) {
    // Format: DIGITAL_WRITE <pin> <value>
    // Example: DIGITAL_WRITE 7 HIGH
    int firstSpace = command.indexOf(' ');
    int secondSpace = command.indexOf(' ', firstSpace + 1);
    
    if (secondSpace > 0) {
      int pin = command.substring(firstSpace + 1, secondSpace).toInt();
      String value = command.substring(secondSpace + 1);
      
      if (value == "HIGH" || value == "1") {
        digitalWrite(pin, HIGH);
        Serial.println("OK");
      } else if (value == "LOW" || value == "0") {
        digitalWrite(pin, LOW);
        Serial.println("OK");
      } else {
        Serial.println("ERROR:Invalid value");
      }
    } else {
      Serial.println("ERROR:Invalid format");
    }
  }
  else if (command.startsWith("DIGITAL_READ ")) {
    // Format: DIGITAL_READ <pin>
    // Example: DIGITAL_READ 7
    int pin = command.substring(13).toInt();
    int value = digitalRead(pin);
    Serial.print("VALUE:");
    Serial.println(value);
  }
  else if (command.startsWith("ANALOG_READ ")) {
    // Format: ANALOG_READ <pin>
    // Example: ANALOG_READ 0 (for A0)
    int pin = command.substring(12).toInt();
    int value = analogRead(pin);
    Serial.print("VALUE:");
    Serial.println(value);
  }
  else if (command.startsWith("PWM_WRITE ")) {
    // Format: PWM_WRITE <pin> <value>
    // Example: PWM_WRITE 9 128
    int firstSpace = command.indexOf(' ');
    int secondSpace = command.indexOf(' ', firstSpace + 1);
    
    if (secondSpace > 0) {
      int pin = command.substring(firstSpace + 1, secondSpace).toInt();
      int value = command.substring(secondSpace + 1).toInt();
      
      if (value >= 0 && value <= 255) {
        analogWrite(pin, value);
        Serial.println("OK");
      } else {
        Serial.println("ERROR:Value out of range");
      }
    } else {
      Serial.println("ERROR:Invalid format");
    }
  }
  else if (command == "ECHO_ON") {
    echoMode = true;
    Serial.println("OK");
  }
  else if (command == "ECHO_OFF") {
    echoMode = false;
    Serial.println("OK");
  }
  else if (command == "PING") {
    Serial.println("PONG");
  }
  else if (command == "VERSION") {
    Serial.println("VERSION:1.0.0");
  }
  else if (command == "RESET") {
    // Reset all pins to default state
    digitalWrite(LED_PIN, LOW);
    echoMode = false;
    Serial.println("OK");
  }
  else if (command.startsWith("DELAY ")) {
    // Format: DELAY <milliseconds>
    // Useful for testing timing
    int delayTime = command.substring(6).toInt();
    if (delayTime > 0 && delayTime <= 1000) {
      delay(delayTime);
      Serial.println("OK");
    } else {
      Serial.println("ERROR:Invalid delay");
    }
  }
  else if (command == "HELP") {
    Serial.println("Commands:");
    Serial.println("  PROBE - Identify device");
    Serial.println("  LED_ON/LED_OFF/LED_STATE - Control built-in LED");
    Serial.println("  PIN_MODE <pin> <OUTPUT|INPUT|INPUT_PULLUP>");
    Serial.println("  DIGITAL_WRITE <pin> <HIGH|LOW>");
    Serial.println("  DIGITAL_READ <pin>");
    Serial.println("  ANALOG_READ <pin>");
    Serial.println("  PWM_WRITE <pin> <0-255>");
    Serial.println("  ECHO_ON/ECHO_OFF - Toggle echo mode");
    Serial.println("  PING - Test connection");
    Serial.println("  VERSION - Get firmware version");
    Serial.println("  RESET - Reset to defaults");
    Serial.println("  DELAY <ms> - Delay response");
    Serial.println("  HELP - Show this help");
  }
  else {
    Serial.print("ERROR:Unknown command: ");
    Serial.println(command);
  }
}