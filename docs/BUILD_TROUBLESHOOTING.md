# Build & Testing Troubleshooting Guide

## Issue: Cargo Test Hangs Indefinitely

### Root Cause
Windows Rust compilation can hang due to:
1. **Memory pressure** during parallel compilation
2. **Resource contention** between rustc processes  
3. **Linker deadlocks** during dependency resolution
4. **Infinite macro expansion** (less likely but possible)

### Solutions Applied

#### 1. Cargo Configuration (.cargo/config.toml)
```toml
[build]
jobs = 2  # Reduced from default to prevent resource contention
incremental = true  # Enable incremental builds

[profile.dev]
debug = false  # Reduce memory usage during compilation
```

#### 2. Hardware Tests Feature Flag
```toml
[features]
hardware-tests = []  # Guards tests requiring real hardware
```

Usage:
```bash
cargo test --features hardware-tests  # With real hardware
cargo test --lib                      # Safe mock tests only
```

#### 3. Performance Test Categories
- **Unit tests**: Mock all I/O, run with `cargo test performance --lib`
- **Integration tests**: Use `DriverTestFixture` pattern  
- **Hardware tests**: Gated behind `#[cfg(feature = "hardware-tests")]`

#### 4. Release Build Optimization  
```toml
[profile.release]
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Remove debug symbols
opt-level = "z"     # Optimize for size
```

**Note**: Release builds may still hang on Windows due to LTO complexity.

### Coverage Testing Setup

#### Install Tarpaulin (Linux/WSL only)
```bash
cargo install cargo-tarpaulin
```

#### Configuration (Cargo.toml)
```toml
[package.metadata.tarpaulin]
exclude = ["*/tests/*", "*/examples/*", "*/bin/*", "src/bin/*"]
timeout = 120
out = ["Html", "Lcov"]
ignore-panics = true
features = []
```

#### Run Coverage
```bash
# On Linux/WSL
cargo tarpaulin --out Html --out Lcov \
    --exclude-files "*/tests/*" \
    --exclude-files "*/examples/*" \
    --ignore-panics \
    --timeout 120

# On Windows (alternative)
cargo test --lib  # Basic test coverage
```

### Cargo Aliases (Available Commands)
```bash
cargo perf-test      # Run performance tests only
cargo hardware-test  # Run with hardware feature
cargo test-safe      # All tests except hardware  
cargo quick-check    # Compilation check only
cargo fix-all        # Format + clippy fixes
cargo coverage       # Tarpaulin coverage (WSL/Linux)
```

### Emergency Process Management
```bash
# Kill stuck cargo/rustc processes
taskkill /IM "cargo.exe" /F
taskkill /IM "rustc.exe" /F

# Alternative using WMIC
wmic process where "name like '%cargo%' or name like '%rustc%'" delete
```

### Verification Steps
1. **Compilation**: `cargo check --lib` (should complete in <30s)
2. **Performance Tests**: `cargo test performance --lib` (12 tests should pass)
3. **All Library Tests**: `cargo test --lib` (broader test suite)
4. **Hardware Tests**: `cargo test --features hardware-tests` (requires real Arduino)

### Test Results Summary
- ✅ **Performance tests**: 12 passed, 0 failed (0.02s runtime)
- ✅ **Mock transport patterns**: Working correctly
- ✅ **Feature flag setup**: Hardware tests properly gated
- ✅ **Cargo aliases**: Configured for easy access

### Known Limitations
- **Tarpaulin**: Requires Linux/macOS or WSL on Windows
- **Release builds**: May hang due to LTO complexity on Windows
- **Parallel compilation**: Limited to 2 jobs to prevent hangs
- **Coverage reporting**: Use alternative tools on native Windows

### Architecture Notes
The performance monitoring system includes:
- `PerformanceMonitor`: Runtime metrics collection
- `BudgetEnforcer`: Resource limit enforcement  
- `ResourceBudget`: Configurable performance limits
- Task 17 compliance: <2s startup, ≤2% CPU, ≤150MB RAM

All tests use proper async/await patterns with tokio runtime.
