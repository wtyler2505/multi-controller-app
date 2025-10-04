// Local type definitions for telemetry package
// These are copied from core to avoid import issues during development

/**
 * Telemetry data point with timestamp
 */
export interface ITelemetryDataPoint {
  timestamp: number;
  value: number | string | Buffer;
  stream: string;
  deviceId: string;
}

/**
 * Telemetry parser interface for different data formats
 */
export interface ITelemetryParser {
  formatType: 'csv' | 'json' | 'binary';
  parse(data: Buffer): ITelemetryDataPoint[];
  validate(data: Buffer): boolean;
}

/**
 * Ring buffer for storing telemetry data
 */
export interface IRingBuffer<T> {
  capacity: number;
  size: number;
  isFull: boolean;
  push(item: T): boolean;
  pop(): T | undefined;
  peek(): T | undefined;
  clear(): void;
  toArray(): T[];
}

/**
 * Telemetry collector configuration
 */
export interface ITelemetryConfig {
  samplingRate: number; // Hz (10-1000)
  bufferCapacity: number; // Default 2000+
  decimationFactor: number; // For visualization optimization
  enableErrorCorrection: boolean;
  maxRetries: number;
}

/**
 * Telemetry collector interface
 */
export interface ITelemetryCollector {
  config: ITelemetryConfig;
  start(): Promise<void>;
  stop(): Promise<void>;
  subscribe(stream: string, handler: (data: ITelemetryDataPoint[]) => void): void;
  unsubscribe(stream: string): void;
  getLatestData(stream: string, count?: number): ITelemetryDataPoint[];
  getDecimatedData(stream: string, factor?: number): ITelemetryDataPoint[];
}

/**
 * Base transport interface for all communication protocols
 */
export interface ITransport {
  id: string;
  name: string;
  type: 'serial' | 'tcp' | 'udp' | 'ssh';
  
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  send(data: Buffer): Promise<void>;
  onData(handler: (data: Buffer) => void): void;
  onError(handler: (error: Error) => void): void;
}