// Telemetry package - Real-time data collection and buffering
export * from './ring-buffer';
export * from './parsers';
export * from './collector';

// Re-export core types for convenience
export type {
  ITelemetryDataPoint,
  ITelemetryParser,
  ITelemetryConfig,
  ITelemetryCollector,
  IRingBuffer
} from '../../core/dist/index';

export const VERSION = '1.0.0';