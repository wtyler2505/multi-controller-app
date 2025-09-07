@echo off
echo Multi-Controller App - Cargo Build Helper
echo.
echo Available Commands:
echo   quick-test    - Run performance tests only
echo   safe-build    - Build without optimization  
echo   hardware-test - Run tests with hardware feature
echo   coverage      - Generate coverage report (WSL/Linux only)
echo.
echo Performance tests can be run with:
cargo perf-test
echo.
echo For WSL/Linux coverage, install tarpaulin:
echo cargo install cargo-tarpaulin
echo Then run: cargo coverage
echo.
echo Release builds may hang on Windows - use incremental builds instead

