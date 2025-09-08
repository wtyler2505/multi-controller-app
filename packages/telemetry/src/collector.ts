import { EventEmitter } from 'eventemitter3';
import { setInterval, clearInterval } from 'node:timers';
import type { 
  ITelemetryCollector, 
  ITelemetryConfig, 
  ITelemetryDataPoint, 
  ITelemetryParser,
  ITransport 
} from './types';
import { RingBuffer } from './ring-buffer';
import { TelemetryParserFactory } from './parsers';

// Constants for configuration defaults
const DEFAULT_SAMPLING_RATE = 100; // Hz
const DEFAULT_BUFFER_CAPACITY = 2000;
const DEFAULT_DECIMATION_FACTOR = 10;
const DEFAULT_MAX_RETRIES = 3;

/**
 * Real-time telemetry data collector with buffering and processing capabilities
 */
export class TelemetryCollector extends EventEmitter implements ITelemetryCollector {
  public readonly config: ITelemetryConfig;
  private streams: Map<string, RingBuffer<ITelemetryDataPoint>> = new Map();
  private subscribers: Map<string, Set<(data: ITelemetryDataPoint[]) => void>> = new Map();
  private parsers: Map<string, ITelemetryParser> = new Map();
  private transports: Map<string, ITransport> = new Map();
  private isRunning: boolean = false;
  private samplingTimer: ReturnType<typeof setInterval> | null = null;
  private errorCounts: Map<string, number> = new Map();

  constructor(config: Partial<ITelemetryConfig> = {}) {
    super();
    
    this.config = {
      samplingRate: config.samplingRate ?? DEFAULT_SAMPLING_RATE,
      bufferCapacity: config.bufferCapacity ?? DEFAULT_BUFFER_CAPACITY,
      decimationFactor: config.decimationFactor ?? DEFAULT_DECIMATION_FACTOR,
      enableErrorCorrection: config.enableErrorCorrection ?? true,
      maxRetries: config.maxRetries ?? DEFAULT_MAX_RETRIES,
      ...config
    };

    this.validateConfig();
  }

  private validateConfig(): void {
    if (this.config.samplingRate < 10 || this.config.samplingRate > 1000) {
      throw new Error('Sampling rate must be between 10 and 1000 Hz');
    }
    if (this.config.bufferCapacity < 100) {
      throw new Error('Buffer capacity must be at least 100');
    }
    if (this.config.decimationFactor < 1) {
      throw new Error('Decimation factor must be at least 1');
    }
  }

  /**
   * Start the telemetry collection process
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      throw new Error('Telemetry collector is already running');
    }

    this.isRunning = true;
    
    // Start sampling timer
    const intervalMs = 1000 / this.config.samplingRate;
    this.samplingTimer = setInterval(() => {
      this.processPendingData();
    }, intervalMs);

    this.emit('started');
  }

  /**
   * Stop the telemetry collection process
   */
  async stop(): Promise<void> {
    if (!this.isRunning) {
      return;
    }

    this.isRunning = false;
    
    if (this.samplingTimer) {
      clearInterval(this.samplingTimer);
      this.samplingTimer = null;
    }

    // Disconnect all transports
    for (const transport of this.transports.values()) {
      try {
        await transport.disconnect();
      } catch (error) {
        this.emit('error', error);
      }
    }

    this.emit('stopped');
  }

  /**
   * Add a transport for data collection
   */
  addTransport(transport: ITransport, formatType: 'csv' | 'json' | 'binary' = 'json'): void {
    if (this.transports.has(transport.id)) {
      throw new Error(`Transport ${transport.id} already added`);
    }

    const parser = TelemetryParserFactory.createParser(formatType, transport.id);
    this.parsers.set(transport.id, parser);
    this.transports.set(transport.id, transport);
    this.errorCounts.set(transport.id, 0);

    // Set up data handling
    transport.onData((data: Buffer) => {
      this.handleTransportData(transport.id, data);
    });

    transport.onError((error: Error) => {
      this.handleTransportError(transport.id, error);
    });
  }

  /**
   * Remove a transport
   */
  async removeTransport(transportId: string): Promise<void> {
    const transport = this.transports.get(transportId);
    if (!transport) {
      return;
    }

    try {
      await transport.disconnect();
    } catch (error) {
      this.emit('error', error);
    }

    this.transports.delete(transportId);
    this.parsers.delete(transportId);
    this.errorCounts.delete(transportId);
  }

  /**
   * Subscribe to telemetry data for a specific stream
   */
  subscribe(stream: string, handler: (data: ITelemetryDataPoint[]) => void): void {
    if (!this.subscribers.has(stream)) {
      this.subscribers.set(stream, new Set());
    }
    const streamSubscribers = this.subscribers.get(stream);
    if (streamSubscribers) {
      streamSubscribers.add(handler);
    }

    // Create buffer for new stream
    if (!this.streams.has(stream)) {
      this.streams.set(stream, new RingBuffer<ITelemetryDataPoint>(this.config.bufferCapacity));
    }
  }

  /**
   * Unsubscribe from telemetry data for a specific stream
   */
  unsubscribe(stream: string, handler?: (data: ITelemetryDataPoint[]) => void): void {
    const streamSubscribers = this.subscribers.get(stream);
    if (!streamSubscribers) {
      return;
    }

    if (handler) {
      streamSubscribers.delete(handler);
    } else {
      streamSubscribers.clear();
    }

    // Clean up empty subscribers
    if (streamSubscribers.size === 0) {
      this.subscribers.delete(stream);
    }
  }

  /**
   * Get the latest data points for a stream
   */
  getLatestData(stream: string, count: number = 100): ITelemetryDataPoint[] {
    const buffer = this.streams.get(stream);
    if (!buffer) {
      return [];
    }

    return buffer.getRecent(count);
  }

  /**
   * Get decimated data for efficient visualization
   */
  getDecimatedData(stream: string, factor?: number): ITelemetryDataPoint[] {
    const buffer = this.streams.get(stream);
    if (!buffer) {
      return [];
    }

    const decimationFactor = factor ?? this.config.decimationFactor;
    const allData = buffer.toArray();
    
    if (decimationFactor <= 1) {
      return allData;
    }

    const decimated: ITelemetryDataPoint[] = [];
    for (let i = 0; i < allData.length; i += decimationFactor) {
      decimated.push(allData[i]);
    }

    return decimated;
  }

  /**
   * Handle incoming data from a transport
   */
  private handleTransportData(transportId: string, data: Buffer): void {
    try {
      const parser = this.parsers.get(transportId);
      if (!parser) {
        this.emit('error', new Error(`No parser found for transport ${transportId}`));
        return;
      }

      // Validate data if error correction is enabled
      if (this.config.enableErrorCorrection && !parser.validate(data)) {
        this.handleDataError(transportId, new Error('Invalid data format'));
        return;
      }

      const dataPoints = parser.parse(data);
      this.processDataPoints(dataPoints);

      // Reset error count on successful processing
      this.errorCounts.set(transportId, 0);

    } catch (error) {
      this.handleDataError(transportId, error as Error);
    }
  }

  /**
   * Process parsed data points
   */
  private processDataPoints(dataPoints: ITelemetryDataPoint[]): void {
    for (const point of dataPoints) {
      // Get or create buffer for this stream
      let buffer = this.streams.get(point.stream);
      if (!buffer) {
        buffer = new RingBuffer<ITelemetryDataPoint>(this.config.bufferCapacity);
        this.streams.set(point.stream, buffer);
      }

      // Add to buffer
      const wasOverwritten = !buffer.push(point);
      if (wasOverwritten) {
        this.emit('bufferOverflow', point.stream);
      }

      // Notify subscribers
      const subscribers = this.subscribers.get(point.stream);
      if (subscribers && subscribers.size > 0) {
        const recentData = buffer.getRecent(1); // Send just the new point
        subscribers.forEach(handler => {
          try {
            handler(recentData);
          } catch (error) {
            this.emit('subscriberError', error);
          }
        });
      }
    }
  }

  /**
   * Handle transport errors
   */
  private handleTransportError(transportId: string, error: Error): void {
    this.emit('transportError', { transportId, error });
  }

  /**
   * Handle data parsing errors
   */
  private handleDataError(transportId: string, error: Error): void {
    const errorCount = (this.errorCounts.get(transportId) ?? 0) + 1;
    this.errorCounts.set(transportId, errorCount);

    if (errorCount >= this.config.maxRetries) {
      this.emit('maxRetriesExceeded', { transportId, error });
    } else {
      this.emit('dataError', { transportId, error, retryCount: errorCount });
    }
  }

  /**
   * Process any pending data (called by sampling timer)
   */
  private processPendingData(): void {
    // This method is called periodically and can be used for:
    // - Batch processing of accumulated data
    // - Triggering periodic notifications
    // - Cleanup operations
    
    // Emit periodic statistics
    const stats = {
      totalStreams: this.streams.size,
      totalDataPoints: Array.from(this.streams.values()).reduce((sum, buffer) => sum + buffer.size, 0),
      activeTransports: this.transports.size
    };
    
    this.emit('statistics', stats);
  }

  /**
   * Get collector statistics
   */
  getStatistics(): any {
    const streamStats: Record<string, any> = {};
    
    for (const [streamName, buffer] of this.streams.entries()) {
      streamStats[streamName] = {
        dataPoints: buffer.size,
        bufferUtilization: (buffer.size / buffer.capacity) * 100,
        isFull: buffer.isFull
      };
    }

    return {
      isRunning: this.isRunning,
      config: this.config,
      totalStreams: this.streams.size,
      totalTransports: this.transports.size,
      streamStats,
      errorCounts: Object.fromEntries(this.errorCounts)
    };
  }
}