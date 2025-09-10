# Multi-Controller App - Coding Standards

## Overview

This document defines the coding standards and conventions for the Multi-Controller App project. All contributors must follow these guidelines to ensure consistency, maintainability, and quality across the codebase.

## Table of Contents

1. [File Management Rules](#file-management-rules)
2. [Code References Format](#code-references-format)
3. [Performance Validation](#performance-validation)
4. [Naming Conventions](#naming-conventions)
5. [Type Safety](#type-safety)
6. [Code Organization](#code-organization)
7. [Error Handling](#error-handling)
8. [Async Patterns](#async-patterns)
9. [Performance Guidelines](#performance-guidelines)
10. [PowerShell Script Standards](#powershell-script-standards)
11. [Documentation](#documentation)
12. [Testing Standards](#testing-standards)
13. [Security Practices](#security-practices)

## File Management Rules

### Absolute Prohibitions

- **NEVER create new files unless EXPLICITLY requested with exact path by user**
- **NEVER create documentation files (*.md, *.txt, *.rst) proactively**
- **NEVER create README files without explicit user request**
- **NEVER create example/sample files without explicit instruction**
- **NEVER create test files unless user specifically asks for tests**

### Required Practices

- **ALWAYS edit existing files rather than creating new ones**
- **ALWAYS ask "Should I create <filename>?" before ANY file creation**
- **ALWAYS use Edit or MultiEdit tools for modifications**
- **ALWAYS check if a similar file exists before proposing new file**

### File Operation Workflow

1. User requests feature → Check existing files first
2. Find closest existing file → Propose editing it
3. If truly new file needed → Request explicit permission with full path
4. After permission → Create with minimal content
5. Log file creation in decision log

## Code References Format

### Mandatory Format

**EVERY code reference MUST use**: `file_path:line_number`

- **Path**: Always from project root, forward slashes
- **Line**: Exact line number, update if code moves
- **Pattern**: `path/to/file.ext:123`
- **Never**: Use relative paths or omit line numbers

### Usage Examples

```typescript
// In error messages
throw new Error('Serial connection failed at transports/serial/src/index.ts:32');

// In TODO comments
// TODO: Refactor this to use the pattern at packages/core/src/utils.ts:145

// In cross-references
// See performance validation: apps/desktop/Program.cs:48-61
```

### Git Commit Standards

#### Automated Commit Generation

Use the smart commit tool for consistent, context-aware commits:

```bash
# Recommended approach
npm run task:commit

# This tool automatically:
# - Analyzes staged files to determine commit type
# - Fetches task context from Task Master
# - Enforces conventional commit format
# - Adds task reference
```

#### Manual Commit Format

When automation is unavailable, follow this format:

```bash
<type>(<scope>): <subject> (task <id>)

<body>

<footer>
```

Example:
```bash
fix(serial): resolve timeout issue in transports/serial/src/index.ts:44 (task 11)

The retry logic was not properly handling connection timeouts.
Fixed by implementing the pattern from packages/core/src/retry.ts:23
```

#### Commit Types

- `feat`: New feature
- `fix`: Bug fix  
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions or corrections
- `chore`: Routine tasks, maintenance

## Performance Validation

### Required Validation

All code changes must validate against performance budgets:

- **Startup time**: < 2s (measure with `Program.ValidateStartupPerformance()`)
- **Idle CPU**: ≤ 2% (monitor with Performance Profiler)
- **Base RAM**: ≤ 150 MB (≤ 220 MB with charts active)
- **Serial latency**: ≤ 50 ms (enforce in transport layer)
- **Network latency**: ≤ 100 ms (TCP/UDP operations)

### Performance Comments

```csharp
// ALWAYS include performance impact comment
// Performance Impact: Stays within 150MB RAM budget
// Startup Impact: Adds ~50ms to initialization
```

### Measurement Patterns

```typescript
// Measure operation latency
const start = performance.now();
await operation();
const latency = performance.now() - start;
if (latency > 50) {
  logger.warn(`Operation exceeded 50ms budget: ${latency}ms at operations.ts:42`);
}
```

## Naming Conventions

### Files and Directories

```typescript
// ✅ Good
device-manager.ts       // kebab-case for regular files
IDeviceDriver.ts       // PascalCase for interface files
DeviceError.ts         // PascalCase for class files
serial-transport/      // kebab-case for directories

// ❌ Bad
deviceManager.ts       // camelCase files
device_manager.ts      // snake_case files
DEVICE-MANAGER.ts     // UPPER-CASE files
```

### Variables and Functions

```typescript
// ✅ Good
const deviceId: string = generateId();
const MAX_RECONNECT_ATTEMPTS = 5;  // UPPER_SNAKE_CASE for constants
let isConnected = false;           // boolean prefixed with is/has/should

function calculateLatency(start: number, end: number): number {
  return end - start;
}

// ❌ Bad
const device_id = generateId();    // snake_case
const maxReconnectAttempts = 5;    // not uppercase for constant
let connected = false;              // ambiguous boolean name
```

### Classes and Interfaces

```typescript
// ✅ Good
interface ITransport {              // Interfaces prefixed with 'I'
  connect(): Promise<void>;
}

class DeviceManager {               // PascalCase for classes
  private devices: Map<string, IDevice>;
}

type DeviceId = string;             // Type aliases in PascalCase
enum TransportType {                // Enums in PascalCase
  Serial = 'serial',                // Enum values in PascalCase
  TCP = 'tcp'
}

// ❌ Bad
interface Transport {}              // Missing 'I' prefix
class deviceManager {}              // Not PascalCase
type deviceId = string;             // Not PascalCase
```

### Async Methods

```typescript
// ✅ Good - No 'Async' suffix, let async/await pattern indicate async
async connect(): Promise<void> {
  await this.transport.connect();
}

// ❌ Bad - Redundant 'Async' suffix
async connectAsync(): Promise<void> {
  await this.transport.connect();
}
```

## Type Safety

### No Any Types

```typescript
// ✅ Good
function processData(data: unknown): ProcessedData {
  if (isValidData(data)) {
    return transformData(data as ValidData);
  }
  throw new Error('Invalid data');
}

interface ConfigOptions {
  port: number;
  host: string;
  timeout?: number;
}

// ❌ Bad
function processData(data: any): any {
  return transformData(data);
}

const config: any = { port: 3000 };
```

### Use Enums for Fixed Values

```typescript
// ✅ Good
enum LogLevel {
  Error = 'error',
  Warn = 'warn',
  Info = 'info',
  Debug = 'debug'
}

enum DeviceStatus {
  Disconnected = 0,
  Connecting = 1,
  Connected = 2,
  Error = 3
}

// ❌ Bad
type LogLevel = 'error' | 'warn' | 'info' | 'debug';
const DEVICE_STATUS = {
  DISCONNECTED: 0,
  CONNECTING: 1,
  CONNECTED: 2
};
```

### Strict Configuration Types

```typescript
// ✅ Good
interface SerialConfig {
  port: string;
  baudRate: number;
  dataBits: 5 | 6 | 7 | 8;
  stopBits: 1 | 1.5 | 2;
  parity: 'none' | 'even' | 'odd';
}

// ❌ Bad
interface SerialConfig {
  port: string;
  baudRate: number;
  options?: Record<string, any>;
}
```

## Code Organization

### File Structure

```typescript
// ✅ Good - Organized by feature/responsibility
/**
 * Device Manager - Orchestrates device discovery and lifecycle
 * @module DeviceManager
 */

// External imports first
import { EventEmitter } from 'events';
import type { Logger } from 'winston';

// Internal imports second
import type { IDevice, ITransport } from './interfaces';
import { DeviceError, ErrorCode } from './errors';
import { generateId, delay } from './utils';

// Constants
const DEFAULT_TIMEOUT_MS = 5000;
const MAX_DEVICES = 100;

// Types/Interfaces local to this file
interface DeviceState {
  status: DeviceStatus;
  lastSeen: number;
}

// Main class/function
export class DeviceManager extends EventEmitter {
  // Static members first
  static readonly VERSION = '1.0.0';
  
  // Private fields
  private devices: Map<string, IDevice>;
  private states: Map<string, DeviceState>;
  
  // Constructor
  constructor(private logger: Logger) {
    super();
    this.devices = new Map();
    this.states = new Map();
  }
  
  // Public methods
  public async connect(id: string): Promise<void> {
    // Implementation
  }
  
  // Protected methods
  protected validateDevice(device: IDevice): boolean {
    // Implementation
  }
  
  // Private methods
  private updateState(id: string, status: DeviceStatus): void {
    // Implementation
  }
}
```

### Method Organization

```typescript
// ✅ Good - Single responsibility, clear naming
class TelemetryBuffer {
  private buffer: number[] = [];
  private readonly maxSize: number;
  
  constructor(maxSize: number) {
    this.maxSize = maxSize;
  }
  
  add(value: number): void {
    if (this.buffer.length >= this.maxSize) {
      this.buffer.shift();
    }
    this.buffer.push(value);
  }
  
  getLatest(count: number): number[] {
    return this.buffer.slice(-count);
  }
  
  clear(): void {
    this.buffer = [];
  }
}

// ❌ Bad - Multiple responsibilities, unclear methods
class DataHandler {
  handleData(data: any): any {
    // Does too many things
    if (typeof data === 'string') {
      // Parse string
    } else if (Array.isArray(data)) {
      // Process array
    }
    // Store to database
    // Send to telemetry
    // Update UI
  }
}
```

## Error Handling

### Custom Error Classes

```typescript
// ✅ Good
export class DeviceError extends Error {
  constructor(
    public readonly code: ErrorCode,
    message: string,
    public readonly deviceId?: string,
    public readonly cause?: Error
  ) {
    super(message);
    this.name = 'DeviceError';
    Error.captureStackTrace(this, DeviceError);
  }
}

export enum ErrorCode {
  ConnectionFailed = 'CONNECTION_FAILED',
  Timeout = 'TIMEOUT',
  InvalidResponse = 'INVALID_RESPONSE',
  DeviceNotFound = 'DEVICE_NOT_FOUND'
}

// Usage
throw new DeviceError(
  ErrorCode.ConnectionFailed,
  `Failed to connect to device ${deviceId}`,
  deviceId,
  originalError
);
```

### Error Handling Patterns

```typescript
// ✅ Good - Explicit error handling
async function connectDevice(id: string): Promise<void> {
  try {
    const device = await findDevice(id);
    await device.connect();
    logger.info(`Connected to device ${id}`);
  } catch (error) {
    if (error instanceof DeviceError) {
      logger.error(`Device error: ${error.message}`, { 
        code: error.code,
        deviceId: error.deviceId 
      });
      throw error;
    }
    
    // Wrap unexpected errors
    throw new DeviceError(
      ErrorCode.UnknownError,
      'Unexpected error during connection',
      id,
      error as Error
    );
  }
}

// ❌ Bad - Silent failures, generic catches
async function connectDevice(id: string): Promise<void> {
  try {
    const device = await findDevice(id);
    await device.connect();
  } catch {
    // Silently ignoring error
    console.log('Failed');
  }
}
```

## Async Patterns

### Promise Handling

```typescript
// ✅ Good - Proper async/await usage
class TransportManager {
  async connectAll(transports: ITransport[]): Promise<void> {
    const results = await Promise.allSettled(
      transports.map(t => this.connectTransport(t))
    );
    
    const failures = results.filter(r => r.status === 'rejected');
    if (failures.length > 0) {
      logger.warn(`${failures.length} transports failed to connect`);
    }
  }
  
  private async connectTransport(transport: ITransport): Promise<void> {
    const timeout = new Promise<never>((_, reject) => 
      setTimeout(() => reject(new Error('Connection timeout')), 5000)
    );
    
    return Promise.race([transport.connect(), timeout]);
  }
}

// ❌ Bad - Mixing callbacks and promises
function connectDevice(callback: (err?: Error) => void): void {
  findDevice().then(device => {
    device.connect((err) => {
      if (err) callback(err);
      else callback();
    });
  });
}
```

### Resource Cleanup

```typescript
// ✅ Good - Proper cleanup with finally
async function processWithDevice(id: string): Promise<void> {
  let device: IDevice | null = null;
  
  try {
    device = await connectDevice(id);
    await device.process();
  } finally {
    if (device) {
      await device.disconnect();
    }
  }
}

// Using disposable pattern
class DeviceSession implements AsyncDisposable {
  async [Symbol.asyncDispose](): Promise<void> {
    await this.cleanup();
  }
  
  private async cleanup(): Promise<void> {
    await this.device.disconnect();
  }
}
```

## Performance Guidelines

### Memory Management

```typescript
// ✅ Good - Efficient memory usage
class TelemetryBuffer {
  private buffer: Float32Array;
  private index = 0;
  
  constructor(size: number) {
    this.buffer = new Float32Array(size);
  }
  
  add(value: number): void {
    this.buffer[this.index] = value;
    this.index = (this.index + 1) % this.buffer.length;
  }
}

// ❌ Bad - Unbounded growth
class TelemetryBuffer {
  private buffer: number[] = [];
  
  add(value: number): void {
    this.buffer.push(value); // No size limit
  }
}
```

### Efficient Collections

```typescript
// ✅ Good - Use appropriate data structures
class DeviceRegistry {
  private devices = new Map<string, IDevice>();      // O(1) lookup
  private deviceTypes = new Set<string>();           // O(1) uniqueness check
  private weakRefs = new WeakMap<object, IDevice>(); // Allows GC
  
  findByType(type: string): IDevice[] {
    return Array.from(this.devices.values())
      .filter(d => d.type === type);
  }
}

// ❌ Bad - Inefficient lookups
class DeviceRegistry {
  private devices: IDevice[] = [];
  
  findById(id: string): IDevice | undefined {
    return this.devices.find(d => d.id === id); // O(n) lookup
  }
}
```

## PowerShell Script Standards

### Character Encoding Requirements

PowerShell scripts in this project MUST use ASCII-only characters to ensure compatibility across all Windows environments.

#### Prohibited Characters

- **NO emojis**: ✅, ❌, ⚠️, ℹ️, 🚀, etc.
- **NO box-drawing**: ╔, ╗, ╚, ╝, ═, ║, etc.
- **NO special Unicode**: Any character outside ASCII range (0-127)

#### Required Replacements

Use these ASCII alternatives:

| Prohibited | Use Instead |
|------------|-------------|
| ✅ | [OK] or [SUCCESS] |
| ❌ | [ERROR] or [FAIL] |
| ⚠️ | [WARNING] or [WARN] |
| ℹ️ | [INFO] |
| ╔══╗ | +==+ or ---- |
| ║ | \| |
| → | -> or => |

#### Validation Process

```bash
# Validate any PowerShell script before committing
npm run validate:ps1 scripts/my-script.ps1

# Manual validation
powershell -File scripts/my-script.ps1
```

#### Example PowerShell Script

```powershell
# Good - ASCII only
Write-Host "[OK] Process completed successfully" -ForegroundColor Green
Write-Host "[ERROR] Failed to connect" -ForegroundColor Red
Write-Host "[WARNING] High memory usage detected" -ForegroundColor Yellow

# Bad - Contains emojis/Unicode
Write-Host "✅ Process completed successfully" -ForegroundColor Green
Write-Host "❌ Failed to connect" -ForegroundColor Red
Write-Host "⚠️ High memory usage detected" -ForegroundColor Yellow
```

### Error Handling in PowerShell

```powershell
# Always use try-catch for robust error handling
try {
    $result = Get-Process "MultiControllerApp"
    Write-Host "[OK] Process found: $($result.Id)" -ForegroundColor Green
}
catch {
    Write-Host "[ERROR] Process not found: $_" -ForegroundColor Red
    exit 1
}
```

### Output Formatting

```powershell
# Use consistent status prefixes
function Write-Status {
    param(
        [string]$Message,
        [string]$Type = "INFO"
    )
    
    $prefix = switch ($Type) {
        "SUCCESS" { "[OK]"; $color = "Green" }
        "ERROR"   { "[ERROR]"; $color = "Red" }
        "WARNING" { "[WARNING]"; $color = "Yellow" }
        "INFO"    { "[INFO]"; $color = "Cyan" }
        default   { "[$Type]"; $color = "White" }
    }
    
    Write-Host "$prefix $Message" -ForegroundColor $color
}
```

## Documentation

### JSDoc Standards

```typescript
/**
 * Manages device connections and lifecycle
 * @class DeviceManager
 * @extends EventEmitter
 */
export class DeviceManager extends EventEmitter {
  /**
   * Connects to a device with the specified ID
   * @param {string} deviceId - The unique device identifier
   * @param {ConnectionOptions} [options] - Optional connection parameters
   * @returns {Promise<void>} Resolves when connection is established
   * @throws {DeviceError} When device not found or connection fails
   * @example
   * ```typescript
   * await manager.connect('device-123', { timeout: 5000 });
   * ```
   */
  async connect(deviceId: string, options?: ConnectionOptions): Promise<void> {
    // Implementation
  }
}
```

### Inline Comments

```typescript
// ✅ Good - Explains WHY, not WHAT
function calculateDecimation(dataRate: number, displayRate: number): number {
  // Use Nyquist theorem to prevent aliasing while maintaining visual quality
  const minSampleRate = displayRate * 2;
  
  // Apply additional factor for smoother visualization
  const targetRate = minSampleRate * 1.5;
  
  return Math.ceil(dataRate / targetRate);
}

// ❌ Bad - Obvious comments
function calculateDecimation(dataRate: number, displayRate: number): number {
  // Multiply by 2
  const minSampleRate = displayRate * 2;
  
  // Multiply by 1.5
  const targetRate = minSampleRate * 1.5;
  
  // Return the ceiling
  return Math.ceil(dataRate / targetRate);
}
```

## Testing Standards

### Test Organization

```typescript
// ✅ Good - Descriptive, organized tests
describe('DeviceManager', () => {
  let manager: DeviceManager;
  let mockTransport: jest.Mocked<ITransport>;
  
  beforeEach(() => {
    mockTransport = createMockTransport();
    manager = new DeviceManager(mockTransport);
  });
  
  describe('connect', () => {
    it('should establish connection with valid device ID', async () => {
      // Arrange
      const deviceId = 'test-device';
      mockTransport.probe.mockResolvedValue(true);
      
      // Act
      await manager.connect(deviceId);
      
      // Assert
      expect(mockTransport.connect).toHaveBeenCalledWith(deviceId);
      expect(manager.isConnected(deviceId)).toBe(true);
    });
    
    it('should throw DeviceError when device not found', async () => {
      // Arrange
      mockTransport.probe.mockResolvedValue(false);
      
      // Act & Assert
      await expect(manager.connect('invalid'))
        .rejects
        .toThrow(DeviceError);
    });
  });
});
```

## Security Practices

### Input Validation

```typescript
// ✅ Good - Validate all inputs
class CommandProcessor {
  private readonly VALID_COMMANDS = new Set(['read', 'write', 'reset']);
  private readonly MAX_PAYLOAD_SIZE = 1024;
  
  processCommand(command: string, payload: Buffer): void {
    if (!this.VALID_COMMANDS.has(command)) {
      throw new Error(`Invalid command: ${command}`);
    }
    
    if (payload.length > this.MAX_PAYLOAD_SIZE) {
      throw new Error('Payload too large');
    }
    
    // Process validated command
  }
}
```

### Sensitive Data

```typescript
// ✅ Good - Never log sensitive data
logger.info('User authenticated', { 
  userId: user.id,
  timestamp: Date.now()
  // Never log: password, apiKey, tokens
});

// ❌ Bad - Exposing sensitive data
logger.info('Login attempt', {
  username: user.email,
  password: user.password, // NEVER DO THIS
  apiKey: config.apiKey    // NEVER DO THIS
});
```

## Enforcement

These standards are enforced through:

1. **ESLint** - Automated linting rules
2. **Prettier** - Code formatting
3. **TypeScript** - Type checking
4. **Pre-commit hooks** - Validation before commits
5. **CI/CD pipeline** - Automated validation in PRs
6. **Code reviews** - Manual verification

## Exceptions

Exceptions to these standards require:
1. Documentation in code comments explaining why
2. Team approval in PR review
3. Plan for future refactoring if temporary

---

*Last updated: January 2025*
*Version: 1.0.0*