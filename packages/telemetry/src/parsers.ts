import type { ITelemetryParser, ITelemetryDataPoint } from './types';

// Constants for parser configuration
const MIN_CSV_FIELDS = 3;
const MIN_BINARY_LENGTH = 9;
const BINARY_TIMESTAMP_SIZE = 4;
const BINARY_STRING_LENGTH_SIZE = 4;
const BINARY_VALUE_SIZE = 4;

/**
 * Base class for telemetry parsers with common validation logic
 */
export abstract class BaseTelemetryParser implements ITelemetryParser {
  abstract formatType: 'csv' | 'json' | 'binary';
  
  constructor(protected deviceId: string = 'unknown') {}

  abstract parse(data: Buffer): ITelemetryDataPoint[];
  abstract validate(data: Buffer): boolean;

  protected createDataPoint(value: number | string | Buffer, stream: string): ITelemetryDataPoint {
    return {
      timestamp: Date.now(),
      value,
      stream,
      deviceId: this.deviceId
    };
  }
}

/**
 * CSV format parser
 * Expected format: timestamp,stream,value\n
 * Example: 1234567890,temperature,23.5\n1234567891,humidity,65.2\n
 */
export class CsvTelemetryParser extends BaseTelemetryParser {
  formatType: 'csv' = 'csv';
  
  validate(data: Buffer): boolean {
    try {
      const text = data.toString('utf8').trim();
      if (!text) return false;
      
      const lines = text.split('\n');
      for (const line of lines) {
        if (!line.trim()) continue;
        const parts = line.split(',');
        if (parts.length < 2) return false;
        
        // Accept stream,value format (2 parts) or timestamp,stream,value format (3+ parts)
        if (parts.length >= 2) {
          if (parts.length >= 3) {
            // timestamp,stream,value format - validate timestamp
            const timestamp = parseFloat(parts[0]);
            if (isNaN(timestamp)) return false;
          }
          // All formats are valid if they have at least 2 parts
          continue;
        } else {
          return false;
        }
      }
      return true;
    } catch {
      return false;
    }
  }

  parse(data: Buffer): ITelemetryDataPoint[] {
    if (!this.validate(data)) {
      throw new Error('Invalid CSV telemetry data format');
    }

    const text = data.toString('utf8').trim();
    const lines = text.split('\n');
    const points: ITelemetryDataPoint[] = [];

    for (const line of lines) {
      if (!line.trim()) continue;
      
      const parts = line.split(',');
      
      if (parts.length === 2) {
        // stream,value format - use current timestamp
        const [stream, valueStr] = parts;
        const value = this.parseValue(valueStr);
        points.push(this.createDataPoint(value, stream.trim()));
      } else if (parts.length >= 3) {
        // timestamp,stream,value format
        const [timestampStr, stream, valueStr] = parts;
        const timestamp = parseFloat(timestampStr);
        const value = this.parseValue(valueStr);
        
        const point = this.createDataPoint(value, stream.trim());
        point.timestamp = timestamp * 1000; // Convert to milliseconds if needed
        points.push(point);
      }
    }

    return points;
  }

  private parseValue(valueStr: string): number | string {
    const trimmed = valueStr.trim();
    const numeric = parseFloat(trimmed);
    return isNaN(numeric) ? trimmed : numeric;
  }
}

/**
 * JSON format parser
 * Expected format: {"timestamp": 1234567890, "stream": "temperature", "value": 23.5}
 * Or array: [{"timestamp": 1234567890, "stream": "temperature", "value": 23.5}, ...]
 */
export class JsonTelemetryParser extends BaseTelemetryParser {
  formatType: 'json' = 'json';
  
  validate(data: Buffer): boolean {
    try {
      const text = data.toString('utf8').trim();
      const parsed = JSON.parse(text);
      
      if (Array.isArray(parsed)) {
        return parsed.every(item => this.validateDataPoint(item));
      } else {
        return this.validateDataPoint(parsed);
      }
    } catch {
      return false;
    }
  }

  parse(data: Buffer): ITelemetryDataPoint[] {
    if (!this.validate(data)) {
      throw new Error('Invalid JSON telemetry data format');
    }

    const text = data.toString('utf8').trim();
    const parsed = JSON.parse(text);
    const points: ITelemetryDataPoint[] = [];

    if (Array.isArray(parsed)) {
      for (const item of parsed) {
        points.push(this.createDataPointFromJson(item));
      }
    } else {
      points.push(this.createDataPointFromJson(parsed));
    }

    return points;
  }

  private validateDataPoint(item: any): boolean {
    return (
      typeof item === 'object' &&
      item !== null &&
      ('stream' in item) &&
      ('value' in item)
    );
  }

  private createDataPointFromJson(item: any): ITelemetryDataPoint {
    const point = this.createDataPoint(item.value, item.stream);
    
    if (item.timestamp) {
      point.timestamp = typeof item.timestamp === 'number' 
        ? item.timestamp * 1000 // Convert to milliseconds if needed
        : Date.now();
    }
    
    return point;
  }
}

/**
 * Binary format parser
 * Expected format: 4-byte timestamp (uint32) + 1-byte stream length + stream name + 4-byte value (float32)
 * Repeating pattern for multiple data points
 */
export class BinaryTelemetryParser extends BaseTelemetryParser {
  formatType: 'binary' = 'binary';
  
  validate(data: Buffer): boolean {
    try {
      if (data.length < 9) return false; // Minimum: 4 + 1 + 1 + 4 = 10 bytes (but stream can be empty)
      
      let offset = 0;
      while (offset < data.length) {
        if (offset + 8 >= data.length) return false; // Need at least timestamp + stream length + value
        
        const streamLength = data.readUInt8(offset + 4);
        const requiredBytes = 4 + 1 + streamLength + 4; // timestamp + length + stream + value
        
        if (offset + requiredBytes > data.length) return false;
        
        offset += requiredBytes;
      }
      
      return offset === data.length;
    } catch {
      return false;
    }
  }

  parse(data: Buffer): ITelemetryDataPoint[] {
    if (!this.validate(data)) {
      throw new Error('Invalid binary telemetry data format');
    }

    const points: ITelemetryDataPoint[] = [];
    let offset = 0;

    while (offset < data.length) {
      // Read timestamp (4 bytes, uint32, little endian)
      const timestamp = data.readUInt32LE(offset) * 1000; // Convert to milliseconds
      offset += 4;

      // Read stream name length (1 byte)
      const streamLength = data.readUInt8(offset);
      offset += 1;

      // Read stream name
      const stream = data.subarray(offset, offset + streamLength).toString('utf8');
      offset += streamLength;

      // Read value (4 bytes, float32, little endian)
      const value = data.readFloatLE(offset);
      offset += 4;

      const point = this.createDataPoint(value, stream);
      point.timestamp = timestamp;
      points.push(point);
    }

    return points;
  }
}

/**
 * Factory for creating telemetry parsers
 */
export class TelemetryParserFactory {
  static createParser(formatType: 'csv' | 'json' | 'binary', deviceId: string = 'unknown'): ITelemetryParser {
    switch (formatType) {
      case 'csv':
        return new CsvTelemetryParser(deviceId);
      case 'json':
        return new JsonTelemetryParser(deviceId);
      case 'binary':
        return new BinaryTelemetryParser(deviceId);
      default:
        throw new Error(`Unsupported telemetry format: ${formatType}`);
    }
  }
}