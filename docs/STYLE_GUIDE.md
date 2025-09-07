# Multi-Controller App - Style Guide

## Visual Code Examples

This guide provides side-by-side comparisons of good and bad code patterns specific to the Multi-Controller App project.

## Table of Contents

1. [Device Management](#device-management)
2. [Transport Handling](#transport-handling)
3. [Error Handling](#error-handling)
4. [Telemetry & Metrics](#telemetry--metrics)
5. [Configuration](#configuration)
6. [Testing](#testing)

## Device Management

### Device Registration

```typescript
// ❌ BAD: Loose typing, no validation
class DeviceManager {
  devices: any[] = [];
  
  addDevice(device: any) {
    this.devices.push(device);
  }
}

// ✅ GOOD: Strong typing, validation, error handling
class DeviceManager {
  private readonly devices = new Map<DeviceId, IDevice>();
  private readonly logger: Logger;
  
  async registerDevice(device: IDevice): Promise<void> {
    if (this.devices.has(device.id)) {
      throw new DeviceError(
        ErrorCode.DeviceAlreadyRegistered,
        `Device ${device.id} is already registered`,
        device.id
      );
    }
    
    this.validateDevice(device);
    this.devices.set(device.id, device);
    this.logger.info('Device registered', { deviceId: device.id });
  }
  
  private validateDevice(device: IDevice): void {
    if (!device.id || !device.name) {
      throw new ValidationError('Invalid device', 'device', device);
    }
  }
}
```

### Device Discovery

```typescript
// ❌ BAD: No timeout, poor error handling
async function findDevices() {
  const devices = [];
  for (const port of getPorts()) {
    try {
      const device = await probePort(port);
      if (device) devices.push(device);
    } catch {}
  }
  return devices;
}

// ✅ GOOD: Timeout, parallel execution, proper error handling
async function discoverDevices(
  transports: ITransport[],
  options: DiscoveryOptions = {}
): Promise<DiscoveryResult[]> {
  const { timeout = 5000, parallel = true } = options;
  
  const discoveryPromises = transports.map(async (transport) => {
    const timeoutPromise = new Promise<never>((_, reject) =>
      setTimeout(() => reject(new TimeoutError(
        'Discovery timeout',
        'device-discovery',
        timeout
      )), timeout)
    );
    
    try {
      const result = await Promise.race([
        this.probeTransport(transport),
        timeoutPromise
      ]);
      
      return { transport, device: result, status: 'found' };
    } catch (error) {
      this.logger.warn('Discovery failed for transport', {
        transportId: transport.id,
        error: toError(error).message
      });
      return { transport, device: null, status: 'not-found', error };
    }
  });
  
  return parallel
    ? Promise.all(discoveryPromises)
    : sequentialExecute(discoveryPromises);
}
```

## Transport Handling

### Connection Management

```typescript
// ❌ BAD: No reconnection, no state management
class SerialTransport {
  port: any;
  
  connect(portName: string) {
    this.port = new SerialPort(portName);
  }
  
  send(data: string) {
    this.port.write(data);
  }
}

// ✅ GOOD: Reconnection logic, state management, error handling
class SerialTransport implements ITransport {
  private port: SerialPort | null = null;
  private state = DeviceStatus.Disconnected;
  private reconnectAttempts = 0;
  
  async connect(params: SerialConnectionParams): Promise<void> {
    this.state = DeviceStatus.Connecting;
    
    try {
      this.port = await this.createPort(params);
      await this.configurePort(params);
      
      this.port.on('close', () => this.handleDisconnection());
      this.port.on('error', (error) => this.handleError(error));
      
      this.state = DeviceStatus.Connected;
      this.reconnectAttempts = 0;
      
      this.logger.info('Serial port connected', {
        port: params.port,
        baudRate: params.baudRate
      });
    } catch (error) {
      this.state = DeviceStatus.Error;
      throw new TransportError(
        ErrorCode.ConnectionFailed,
        `Failed to connect to ${params.port}`,
        this.id,
        'serial',
        this.shouldRetry(),
        error as Error
      );
    }
  }
  
  private async handleDisconnection(): Promise<void> {
    this.state = DeviceStatus.Disconnected;
    
    if (this.shouldRetry()) {
      await this.attemptReconnection();
    }
  }
  
  private async attemptReconnection(): Promise<void> {
    this.state = DeviceStatus.Reconnecting;
    this.reconnectAttempts++;
    
    const delay = this.calculateBackoff();
    await sleep(delay);
    
    try {
      await this.connect(this.lastParams);
    } catch (error) {
      if (this.reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
        this.emit('reconnectFailed', error);
      }
    }
  }
  
  private calculateBackoff(): number {
    const base = RECONNECT_BASE_DELAY_MS;
    const multiplier = Math.pow(BACKOFF_MULTIPLIER, this.reconnectAttempts);
    const jitter = Math.random() * BACKOFF_JITTER_FACTOR;
    
    return Math.min(
      base * multiplier * (1 + jitter),
      RECONNECT_MAX_DELAY_MS
    );
  }
}
```

## Error Handling

### Command Execution

```typescript
// ❌ BAD: Generic errors, no context
async function sendCommand(cmd: string): Promise<any> {
  try {
    const result = await device.send(cmd);
    return result;
  } catch (err) {
    console.error('Command failed');
    throw err;
  }
}

// ✅ GOOD: Specific errors, rich context, metrics
async function executeCommand(
  deviceId: DeviceId,
  command: Command
): Promise<CommandResult> {
  const startTime = Date.now();
  const commandId = generateId();
  
  this.logger.debug('Executing command', {
    deviceId,
    commandId,
    command: command.type
  });
  
  try {
    const device = this.getDevice(deviceId);
    const result = await device.execute(command);
    
    const duration = Date.now() - startTime;
    
    this.metrics.recordCommandSent(device.type, command.type);
    this.metrics.recordSerialLatency(device.type, 'command', duration);
    
    this.logger.info('Command executed successfully', {
      deviceId,
      commandId,
      duration
    });
    
    return {
      commandId,
      status: CommandStatus.Success,
      data: result,
      duration,
      timestamp: Date.now()
    };
  } catch (error) {
    const duration = Date.now() - startTime;
    const typedError = toError(error);
    
    this.metrics.recordCommandFailed(
      device?.type || 'unknown',
      command.type,
      typedError.name
    );
    
    this.logger.error('Command execution failed', typedError, {
      deviceId,
      commandId,
      command: command.type,
      duration
    });
    
    throw new CommandError(
      ErrorCode.CommandFailed,
      `Command ${command.type} failed for device ${deviceId}`,
      command.type,
      deviceId,
      typedError
    );
  }
}
```

## Telemetry & Metrics

### Buffer Management

```typescript
// ❌ BAD: Unbounded growth, no type safety
class TelemetryBuffer {
  data = [];
  
  add(value) {
    this.data.push(value);
  }
  
  get() {
    return this.data;
  }
}

// ✅ GOOD: Fixed size, type safe, efficient
class TelemetryBuffer<T extends TelemetryPoint> {
  private readonly buffer: T[];
  private index = 0;
  private count = 0;
  private readonly maxSize: number;
  
  constructor(maxSize: number = TELEMETRY_BUFFER_SIZE) {
    this.maxSize = maxSize;
    this.buffer = new Array(maxSize);
  }
  
  add(point: T): void {
    this.buffer[this.index] = point;
    this.index = (this.index + 1) % this.maxSize;
    this.count = Math.min(this.count + 1, this.maxSize);
    
    if (this.count === this.maxSize) {
      this.metrics.recordTelemetryBufferOverflow(
        point.deviceType,
        point.metric
      );
    }
  }
  
  getLatest(n: number): T[] {
    const result: T[] = [];
    const start = (this.index - Math.min(n, this.count) + this.maxSize) % this.maxSize;
    
    for (let i = 0; i < Math.min(n, this.count); i++) {
      result.push(this.buffer[(start + i) % this.maxSize]!);
    }
    
    return result;
  }
  
  clear(): void {
    this.index = 0;
    this.count = 0;
  }
  
  get size(): number {
    return this.count;
  }
  
  get isFull(): boolean {
    return this.count === this.maxSize;
  }
}
```

### Decimation

```typescript
// ❌ BAD: Naive downsampling, loses important features
function decimate(data: number[], target: number): number[] {
  const ratio = Math.floor(data.length / target);
  return data.filter((_, i) => i % ratio === 0);
}

// ✅ GOOD: LTTB algorithm, preserves visual features
function decimateLTTB(
  data: TelemetryPoint[],
  targetPoints: number
): TelemetryPoint[] {
  if (data.length <= targetPoints) {
    return data;
  }
  
  const bucketSize = (data.length - 2) / (targetPoints - 2);
  const decimated: TelemetryPoint[] = [];
  
  // Always include first point
  decimated.push(data[0]!);
  
  let prevSelectedIndex = 0;
  
  for (let i = 1; i < targetPoints - 1; i++) {
    const bucketStart = Math.floor((i - 1) * bucketSize) + 1;
    const bucketEnd = Math.floor(i * bucketSize) + 1;
    
    // Calculate average for next bucket (for triangle area calculation)
    const nextBucketStart = bucketEnd;
    const nextBucketEnd = Math.min(
      Math.floor((i + 1) * bucketSize) + 1,
      data.length
    );
    
    let avgX = 0;
    let avgY = 0;
    
    for (let j = nextBucketStart; j < nextBucketEnd; j++) {
      avgX += data[j]!.timestamp;
      avgY += data[j]!.value as number;
    }
    
    avgX /= (nextBucketEnd - nextBucketStart);
    avgY /= (nextBucketEnd - nextBucketStart);
    
    // Find point in current bucket with largest triangle area
    let maxArea = -1;
    let selectedIndex = bucketStart;
    
    const prevPoint = data[prevSelectedIndex]!;
    
    for (let j = bucketStart; j < bucketEnd; j++) {
      const point = data[j]!;
      const area = Math.abs(
        (prevPoint.timestamp - avgX) * (point.value as number - prevPoint.value as number) -
        (prevPoint.timestamp - point.timestamp) * (avgY - prevPoint.value as number)
      );
      
      if (area > maxArea) {
        maxArea = area;
        selectedIndex = j;
      }
    }
    
    decimated.push(data[selectedIndex]!);
    prevSelectedIndex = selectedIndex;
  }
  
  // Always include last point
  decimated.push(data[data.length - 1]!);
  
  return decimated;
}
```

## Configuration

### Type-Safe Configuration

```typescript
// ❌ BAD: No validation, any types
function loadConfig(path: string): any {
  const config = JSON.parse(fs.readFileSync(path, 'utf8'));
  return config;
}

// ✅ GOOD: Validation, type safety, defaults
function loadConfig<T extends BaseConfig>(
  path: string,
  schema: ConfigSchema<T>,
  defaults: Partial<T> = {}
): T {
  const rawConfig = this.readConfigFile(path);
  const merged = this.mergeWithDefaults(rawConfig, defaults);
  const validated = this.validateConfig(merged, schema);
  const resolved = this.resolveEnvironmentVariables(validated);
  
  return resolved;
}

private validateConfig<T>(
  config: unknown,
  schema: ConfigSchema<T>
): T {
  const errors: ValidationError[] = [];
  
  for (const [key, validator] of Object.entries(schema)) {
    const value = (config as any)[key];
    
    if (validator.required && value === undefined) {
      errors.push(new ValidationError(
        `Missing required field: ${key}`,
        key,
        undefined
      ));
      continue;
    }
    
    if (value !== undefined && !validator.validate(value)) {
      errors.push(new ValidationError(
        `Invalid value for ${key}`,
        key,
        value,
        validator.constraints
      ));
    }
  }
  
  if (errors.length > 0) {
    throw new AggregateError('Configuration validation failed', errors);
  }
  
  return config as T;
}
```

## Testing

### Unit Tests

```typescript
// ❌ BAD: No setup/teardown, unclear assertions
test('device connects', async () => {
  const device = new Device();
  await device.connect();
  expect(device.connected).toBe(true);
});

// ✅ GOOD: Proper setup/teardown, clear assertions, edge cases
describe('DeviceManager', () => {
  let manager: DeviceManager;
  let mockTransport: jest.Mocked<ITransport>;
  let mockLogger: jest.Mocked<Logger>;
  
  beforeEach(() => {
    mockTransport = createMockTransport();
    mockLogger = createMockLogger();
    manager = new DeviceManager(mockTransport, mockLogger);
  });
  
  afterEach(() => {
    jest.clearAllMocks();
  });
  
  describe('connect', () => {
    it('should establish connection with valid device', async () => {
      // Arrange
      const deviceId = 'test-device-123';
      const mockDevice = createMockDevice({ id: deviceId });
      
      mockTransport.probe.mockResolvedValue(true);
      mockTransport.connect.mockResolvedValue(undefined);
      
      // Act
      await manager.connect(deviceId);
      
      // Assert
      expect(mockTransport.connect).toHaveBeenCalledWith(
        expect.objectContaining({ deviceId })
      );
      expect(manager.isConnected(deviceId)).toBe(true);
      expect(mockLogger.info).toHaveBeenCalledWith(
        'Device connected',
        expect.objectContaining({ deviceId })
      );
    });
    
    it('should throw DeviceError when device not found', async () => {
      // Arrange
      mockTransport.probe.mockResolvedValue(false);
      
      // Act & Assert
      await expect(manager.connect('invalid-device'))
        .rejects
        .toThrow(DeviceError);
      
      expect(mockLogger.error).toHaveBeenCalled();
    });
    
    it('should handle connection timeout', async () => {
      // Arrange
      mockTransport.connect.mockImplementation(
        () => new Promise(resolve => setTimeout(resolve, 10000))
      );
      
      // Act & Assert
      await expect(manager.connect('device', { timeout: 100 }))
        .rejects
        .toThrow(TimeoutError);
    });
  });
});
```

## Summary

The key principles demonstrated in this style guide:

1. **Type Safety**: Use TypeScript's type system fully
2. **Error Context**: Provide rich error information
3. **Performance**: Consider memory and CPU efficiency
4. **Testability**: Write code that's easy to test
5. **Observability**: Include logging and metrics
6. **Resilience**: Handle failures gracefully
7. **Clarity**: Make code self-documenting

Following these patterns will ensure consistent, maintainable, and robust code throughout the Multi-Controller App project.