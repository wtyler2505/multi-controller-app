// Core interfaces for Multi-Controller App
import { EventEmitter } from 'eventemitter3';

/**
 * Base transport interface for all communication protocols
 */
export interface ITransport extends EventEmitter {
  id: string;
  name: string;
  type: 'serial' | 'tcp' | 'udp' | 'ssh';
  
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  send(data: Buffer): Promise<void>;
  onData(handler: (data: Buffer) => void): void;
  onError(handler: (error: Error) => void): void;
}

/**
 * Device driver interface
 */
export interface IDeviceDriver {
  name: string;
  supportedTransports: string[];
  probe(transport: ITransport): Promise<boolean>;
  open(transport: ITransport): Promise<IDeviceSession>;
}

/**
 * Device session interface for active connections
 */
export interface IDeviceSession {
  deviceId: string;
  invoke(endpoint: string, args: unknown[]): Promise<unknown>;
  subscribe(stream: string, handler: (data: Buffer) => void): Promise<void>;
  unsubscribe(stream: string): Promise<void>;
  close(): Promise<void>;
}

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

export const VERSION = '1.0.0';