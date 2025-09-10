// Example: Sandbox Security Demonstration
// This script shows what operations are BLOCKED by the sandbox

print("=== Sandbox Security Demo ===\n");

// ✅ ALLOWED: Safe device operations
print("✅ Allowed operations:");

let devices = list_devices();
print("  - List devices: " + devices);

let device = get_device("arduino_uno");
print("  - Get device: OK");

let value = read(device, "gpio/2");
print("  - Read from device: " + value);

print("  - Basic math: " + (10 * 20 + 30));
print("  - String operations: " + ("Hello" + " " + "World"));

// Sleep is allowed but limited
sleep_ms(100);
print("  - Sleep (limited): OK\n");

// ❌ BLOCKED: Dangerous operations
print("❌ Blocked operations (these would fail):");
print("  - File system access: BLOCKED");
print("  - Network access: BLOCKED");
print("  - System commands: BLOCKED");
print("  - Eval/dynamic code: BLOCKED");
print("  - Import/require: BLOCKED\n");

// Resource limits demonstration
print("📊 Resource Limits:");
print("  - Max operations: 100,000 (default)");
print("  - Max execution time: 10 seconds");
print("  - Max memory: 10 MB");
print("  - Max string size: 1 MB");
print("  - Max array size: 10,000 elements\n");

// Demonstrate loop limits
print("Testing loop limits (max 10,000 iterations in strict mode):");
let count = 0;
for i in 0..100 {
    count += 1;
    // In strict mode, this would stop at configured limit
}
print("  - Completed " + count + " iterations\n");

// Demonstrate safe error handling
print("Safe error handling:");
let missing = get_device("nonexistent_device");
if missing == () {
    print("  - Device not found handled gracefully");
}

// Memory safety
print("\nMemory safety enforced:");
print("  - No raw pointers");
print("  - No unsafe operations");
print("  - Automatic cleanup");
print("  - Reference counting\n");

print("=== Sandbox Demo Complete ===");
print("All operations were safely contained within sandbox limits");