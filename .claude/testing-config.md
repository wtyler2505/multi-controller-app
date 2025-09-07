---
framework: cargo
test_command: cargo test
created: 2025-01-06T17:14:00Z
---

# Testing Configuration - Multi-Controller App

## Framework
- **Type**: Cargo (Rust's built-in test framework)
- **Version**: Cargo 1.88.0 / Rustc 1.88.0
- **Config File**: Cargo.toml
- **Platform**: Windows (win32)

## Test Structure
- **Test Directory**: `tests/` (integration tests)
- **Test Files**: 34 integration test files found
- **Unit Tests**: Embedded in source files with `#[cfg(test)]` modules
- **Naming Patterns**: 
  - Integration: `*_test.rs`, `*_tests.rs`
  - Unit: `test_*` functions in `#[cfg(test)]` modules

## Commands

### Run All Tests
```bash
cargo test -- --nocapture
```

### Run Specific Test File
```bash
# Integration test
cargo test --test profile_test -- --nocapture

# All tests in a module
cargo test profile:: -- --nocapture
```

### Run with Categories
```bash
# Unit tests only
cargo test --lib -- --nocapture

# Integration tests only
cargo test --tests -- --nocapture

# Doc tests only
cargo test --doc -- --nocapture
```

### Run with Features
```bash
# With hardware tests (requires real Arduino)
cargo test --features hardware-tests -- --nocapture

# Without default features
cargo test --no-default-features -- --nocapture
```

### Run with Debugging
```bash
# Very verbose output
cargo test -- --nocapture --test-threads=1

# With backtrace
RUST_BACKTRACE=1 cargo test -- --nocapture

# Full backtrace
RUST_BACKTRACE=full cargo test -- --nocapture
```

### Run Specific Test
```bash
# Exact match
cargo test test_profile_manager_creation -- --nocapture --exact

# Pattern match
cargo test profile -- --nocapture
```

## Environment
- **Required ENV vars**: None for basic tests
- **Optional ENV vars**:
  - `RUST_BACKTRACE`: 1 or full for stack traces
  - `RUST_LOG`: debug/info/warn/error for logging
- **Test Database**: Not required
- **Test Servers**: Not required
- **Hardware**: MockTransport simulates hardware

## Test Categories

### Unit Tests (95+ tests)
Located in `src/**/*.rs` files:
- Transport tests (serial, TCP, UDP, SSH)
- Telemetry system tests
- Profile management tests
- Device driver tests
- UI component tests

### Integration Tests (34 files)
Located in `tests/` directory:
- `arduino_driver_tests.rs` - Arduino driver integration
- `profile_test.rs` - Profile management integration
- `telemetry_chart_integration_test.rs` - Telemetry charts
- `telemetry_ring_buffer_tests.rs` - Ring buffer tests
- `loopback_tests.rs` - Hardware loopback tests
- `performance_tests.rs` - Performance benchmarks
- Transport integration tests
- Device manager tests

## Test Runner Agent Configuration
- **Verbose Output**: Always use `--nocapture`
- **Sequential Execution**: Use `--test-threads=1` for debugging
- **No Real Hardware**: Use MockTransport for hardware simulation
- **Full Stack Traces**: Set `RUST_BACKTRACE=1`
- **Complete Output**: Capture all test output including println!

## Common Issues & Solutions

### Compilation Slow
- **Issue**: Tests take long to compile
- **Solution**: Use `cargo check --tests` first, then run specific tests

### Tests Filtered Out
- **Issue**: "95 filtered out" message
- **Solution**: Use correct test path or --test flag for integration tests

### Hardware Tests Fail
- **Issue**: Hardware tests fail without Arduino
- **Solution**: Don't use `--features hardware-tests` unless hardware connected

### Parallel Test Conflicts
- **Issue**: Tests fail when run in parallel
- **Solution**: Use `--test-threads=1` for sequential execution

### Missing Output
- **Issue**: println! statements don't show
- **Solution**: Always use `--nocapture` flag

## Performance Considerations
- Full test suite compilation: ~2 minutes first time
- Incremental compilation: 10-30 seconds
- Test execution: <5 seconds for all unit tests
- Integration tests: Variable based on test complexity

## MockTransport Pattern
For hardware simulation, use the MockTransport pattern:
```rust
use crate::transport::mock::{MockTransport, MockConfig};

let config = MockConfig {
    simulate_errors: false,
    latency_ms: Some(50),
    connection_behavior: ConnectionBehavior::Stable,
};
let transport = Arc::new(MockTransport::new(config));
```

## Tips for Test Development
1. Always use MockTransport for hardware tests
2. Keep unit tests in `#[cfg(test)]` modules
3. Put integration tests in `tests/` directory
4. Use descriptive test names
5. Test both success and failure paths
6. Use `--nocapture` to see println! output
7. Run with `--test-threads=1` when debugging race conditions