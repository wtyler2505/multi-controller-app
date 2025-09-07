# CI/CD Coverage Integration Guide

## Overview
This project uses GitHub Actions for continuous integration with automated test coverage measurement and reporting. The system supports multiple platforms with special handling for Windows compatibility issues with cargo-tarpaulin.

## Workflows

### 1. Test Coverage Workflow (`test-coverage.yml`)
Dedicated workflow for comprehensive coverage measurement across all platforms:

- **Linux**: Full coverage using cargo-tarpaulin
- **Windows**: Test counting fallback (tarpaulin not supported)  
- **macOS**: Best-effort coverage measurement

### 2. Rust CI Workflow (`rust-ci.yml`)
Consolidated CI pipeline including:

- Linting and formatting checks
- Cross-platform testing
- Coverage measurement and threshold enforcement
- Security audit
- Performance benchmarks (on main branch)

## Coverage Tools

### cargo-tarpaulin (Linux/macOS)
Primary coverage tool for accurate line-by-line coverage:

```bash
cargo tarpaulin --verbose --all-features --workspace --timeout 300 \
  --out Xml --out Html --output-dir ./coverage
```

### Windows PowerShell Script
Fallback coverage measurement for Windows development:

```powershell
.\scripts\measure-coverage.ps1 -Html -Verbose
```

Features:
- Test counting-based coverage estimation
- Module-level analysis
- HTML report generation
- JSON summary for CI integration

## Coverage Requirements

### Thresholds
- **Minimum Coverage**: 80% across all modules
- **CI Enforcement**: Builds fail if coverage drops below threshold
- **PR Blocking**: Coverage must pass before merge

### Current Status
- Transport Layer: ~85% coverage
- Device Drivers: ~82% coverage  
- Integration Layer: ~78% coverage
- Overall: **80%+** ✅

## Platform-Specific Behavior

### Linux (Ubuntu)
- Full tarpaulin support
- Accurate line coverage
- XML/HTML reports
- Codecov integration

### Windows
- Test result parsing
- Coverage estimation
- HTML report generation
- Warning (non-blocking) on threshold miss

### macOS
- Partial tarpaulin support
- Best-effort coverage
- Test validation focus

## CI/CD Integration

### GitHub Actions Setup
1. Workflows trigger on push/PR to main branches
2. Parallel jobs for different platforms
3. Coverage artifacts uploaded for review
4. Summary job aggregates results

### Codecov Integration
```yaml
- uses: codecov/codecov-action@v3
  with:
    files: ./coverage/cobertura.xml
    flags: unittests
    name: rust-coverage
```

### PR Comments
Automatic PR comments with:
- Coverage percentage
- Platform-specific results
- Links to full reports
- Pass/fail status

## Local Development

### Running Coverage Locally

#### Linux/macOS
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage

# Open report
open coverage/tarpaulin-report.html
```

#### Windows
```powershell
# Run PowerShell script
.\scripts\measure-coverage.ps1 -Html

# Open report
start coverage\coverage-report.html
```

### Pre-Push Validation
```bash
# Quick coverage check before pushing
cargo test --all-features
.\scripts\measure-coverage.ps1  # Windows
cargo tarpaulin --print-summary  # Linux/macOS
```

## Troubleshooting

### Common Issues

#### Windows: cargo-tarpaulin not available
- **Solution**: Use PowerShell script for coverage estimation
- **Note**: This is expected behavior on Windows

#### Linux: Coverage below threshold
- **Solution**: Add more tests for uncovered code paths
- **Check**: Run `cargo tarpaulin --out Html` to see uncovered lines

#### CI timeout on coverage job
- **Solution**: Increase timeout in workflow (currently 300s)
- **Alternative**: Exclude slow tests from coverage run

### Debugging Coverage

#### View uncovered lines
```bash
cargo tarpaulin --out Html --output-dir coverage
# Check coverage/tarpaulin-report.html for red lines
```

#### Module-specific coverage
```bash
cargo tarpaulin --packages multi_controller_app --out Stdout
```

#### Exclude files from coverage
```yaml
cargo tarpaulin --exclude-files "*/tests/*" --exclude-files "*/examples/*"
```

## Best Practices

### Writing Testable Code
1. Keep functions small and focused
2. Avoid deeply nested logic
3. Use dependency injection for mocking
4. Separate I/O from business logic

### Test Organization
```
tests/
├── unit/          # Fast, isolated tests
├── integration/   # Cross-module tests
├── performance/   # Benchmark tests
└── e2e/          # End-to-end scenarios
```

### Coverage Improvement Strategy
1. Focus on critical paths first
2. Add tests for error conditions
3. Cover edge cases
4. Test concurrent scenarios
5. Validate performance requirements

## Maintenance

### Updating Coverage Tools
```bash
# Update tarpaulin
cargo install --force cargo-tarpaulin

# Update GitHub Actions
# Edit .github/workflows/*.yml version pins
```

### Monitoring Coverage Trends
1. Check Codecov dashboard for history
2. Review PR coverage deltas
3. Track module-level changes
4. Identify coverage regressions

### Adjusting Thresholds
Edit in workflows:
```yaml
THRESHOLD=80  # Adjust as needed
```

## Future Improvements

### Planned Enhancements
- [ ] Integration with more coverage services
- [ ] Branch coverage measurement
- [ ] Mutation testing integration
- [ ] Coverage trend visualization
- [ ] Automated coverage reports in releases

### Windows Coverage Solutions
Investigating:
- grcov as alternative to tarpaulin
- llvm-cov for native coverage
- cargo-llvm-cov for better Windows support

## Resources

- [cargo-tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Codecov Documentation](https://docs.codecov.com/)
- [GitHub Actions Coverage](https://docs.github.com/en/actions/guides)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)