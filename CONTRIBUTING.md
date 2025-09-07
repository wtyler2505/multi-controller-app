# Contributing to Multi-Controller App

Thank you for your interest in contributing to the Multi-Controller App! This document provides guidelines and instructions for contributing.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)

## 📜 Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Accept responsibility for mistakes
- Prioritize the project's best interests

## 🚀 Getting Started

1. **Fork the repository**
2. **Clone your fork**
```bash
git clone https://github.com/yourusername/multi-controller-app.git
cd multi-controller-app
```

3. **Run setup script**
```powershell
.\scripts\setup-windows.ps1
```

4. **Create a feature branch**
```bash
git checkout -b feature/your-feature-name
```

## 💻 Development Process

### MANDATORY: Verification-First Development

**Before ANY work:**
1. Never claim implementation without proof
2. Always check TaskMaster for next task
3. Never create files unless explicitly requested
4. Always validate performance budgets
5. Always use file:line format for code references

### 1. Task Management Protocol (REQUIRED)

```bash
# ALWAYS run before starting work
npx task-master-ai next                    # Get next available task
npx task-master-ai get-task --id=<id>     # Review task details
npx task-master-ai validate-dependencies   # Check dependencies

# During work
npx task-master-ai set-status --id=<id> --status=in-progress
npx task-master-ai update-subtask --id=<id> --prompt="progress notes"

# After completion
npx task-master-ai set-status --id=<id> --status=review
# After review
npx task-master-ai set-status --id=<id> --status=done
```

### 2. Check existing issues
- Look for existing issues or create a new one
- Comment on the issue to indicate you're working on it
- Reference Task ID in issue comments

### 3. Follow TDD approach
- Write tests first
- Implement the feature
- Ensure all tests pass

### 4. Performance validation (MANDATORY)
- **Startup**: Must be < 2s
- **Idle CPU**: Must be ≤ 2%
- **Base RAM**: Must be ≤ 150MB
- **Serial latency**: Must be ≤ 50ms
- Run performance tests and document results

## 📝 Coding Standards

### TypeScript/JavaScript

- Use ESLint and Prettier configurations
- Follow functional programming principles where appropriate
- Prefer `const` over `let`
- Use meaningful variable names
- Add JSDoc comments for public APIs

```typescript
/**
 * Connects to a hardware device
 * @param config - Device configuration
 * @returns Promise resolving to device session
 */
export async function connectDevice(config: DeviceConfig): Promise<DeviceSession> {
    // Implementation
}
```

### C# / .NET

- Follow Microsoft C# coding conventions
- Use nullable reference types
- Implement proper disposal patterns
- Add XML documentation comments

```csharp
/// <summary>
/// Manages device communication sessions
/// </summary>
public class DeviceManager : IDisposable
{
    // Implementation
}
```

### Performance Guidelines

- Keep startup time under 2 seconds
- Monitor memory allocations
- Use object pooling for frequently created objects
- Implement proper async/await patterns
- Profile before optimizing

## 📦 Commit Guidelines

We follow conventional commits specification:

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions or changes
- `chore`: Build process or auxiliary tool changes

### Examples
```bash
feat(driver): add ESP32 driver implementation
fix(serial): resolve timeout issue on Windows
docs(readme): update installation instructions
perf(telemetry): optimize data decimation algorithm
```

## 🔄 Pull Request Process

1. **Before submitting**
   - Ensure all tests pass: `npm test`
   - Run linting: `npm run lint`
   - Format code: `npm run format`
   - Update documentation if needed

2. **PR title format**
   - Use conventional commit format
   - Reference issue number: `feat(driver): add Arduino support (#123)`

3. **PR description template**
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Performance budgets met

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

4. **Review process**
   - At least one approval required
   - CI checks must pass
   - No merge conflicts

## 🧪 Testing Requirements

### Unit Tests
- Minimum 80% code coverage
- Test edge cases
- Mock external dependencies

### Integration Tests
- Test actual hardware when possible
- Use mock services for CI/CD
- Validate protocol implementations

### Performance Tests
- Measure against budgets
- Profile memory usage
- Check for memory leaks

### Test Structure
```typescript
describe('DeviceDriver', () => {
    describe('connect', () => {
        it('should establish connection within timeout', async () => {
            // Test implementation
        });
        
        it('should handle connection errors gracefully', async () => {
            // Test implementation
        });
    });
});
```

## 📚 Documentation

### Code Documentation
- Add JSDoc/XML comments for public APIs
- Include usage examples
- Document complex algorithms

### README Updates
- Update feature list for new capabilities
- Maintain accurate setup instructions
- Keep dependency list current

### Architecture Documentation
- Update `/docs/architecture/` for significant changes
- Add decision records for architectural choices
- Include diagrams when helpful

## 🏗️ Project Structure Guidelines

### Adding a New Driver
1. Create driver folder: `/drivers/your-driver/`
2. Include `manifest.json`
3. Implement `IDeviceDriver` interface
4. Add unit tests
5. Update supported devices in README

### Adding a New Transport
1. Create transport in `/transports/`
2. Implement `ITransport` interface
3. Add reconnection logic
4. Include performance monitoring
5. Add integration tests

## 🐛 Reporting Issues

### Bug Reports
Include:
- System information (Windows version, .NET version)
- Steps to reproduce
- Expected vs actual behavior
- Error messages/logs
- Screenshots if applicable

### Feature Requests
Include:
- Use case description
- Proposed solution
- Alternative solutions considered
- Impact on existing features

## 💡 Getting Help

- Check [documentation](docs/)
- Search existing issues
- Ask in discussions
- Contact maintainers

## 🎉 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Given credit in commit messages

Thank you for contributing to Multi-Controller App!