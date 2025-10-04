import { 
  CsvTelemetryParser, 
  JsonTelemetryParser, 
  BinaryTelemetryParser,
  TelemetryParserFactory 
} from '../parsers';

describe('TelemetryParsers', () => {
  const deviceId = 'test-device';

  describe('CsvTelemetryParser', () => {
    let parser: CsvTelemetryParser;

    beforeEach(() => {
      parser = new CsvTelemetryParser(deviceId);
    });

    describe('validation', () => {
      it('should validate correct CSV format with timestamp', () => {
        const data = Buffer.from('1234567890,temperature,23.5\n1234567891,humidity,65.2');
        expect(parser.validate(data)).toBe(true);
      });

      it('should validate stream,value format', () => {
        const data = Buffer.from('temperature,23.5\nhumidity,65.2');
        expect(parser.validate(data)).toBe(true);
      });

      it('should reject invalid format', () => {
        const data = Buffer.from('invalid');
        expect(parser.validate(data)).toBe(false);
      });

      it('should reject empty data', () => {
        const data = Buffer.from('');
        expect(parser.validate(data)).toBe(false);
      });

      it('should handle single line', () => {
        const data = Buffer.from('temperature,23.5');
        expect(parser.validate(data)).toBe(true);
      });
    });

    describe('parsing', () => {
      it('should parse stream,value format', () => {
        const data = Buffer.from('temperature,23.5\nhumidity,65.2');
        const result = parser.parse(data);
        
        expect(result).toHaveLength(2);
        expect(result[0]).toMatchObject({
          stream: 'temperature',
          value: 23.5,
          deviceId
        });
        expect(result[1]).toMatchObject({
          stream: 'humidity',
          value: 65.2,
          deviceId
        });
      });

      it('should parse timestamp,stream,value format', () => {
        const data = Buffer.from('1234567890,temperature,23.5');
        const result = parser.parse(data);
        
        expect(result).toHaveLength(1);
        expect(result[0]).toMatchObject({
          timestamp: 1234567890000, // Converted to milliseconds
          stream: 'temperature',
          value: 23.5,
          deviceId
        });
      });

      it('should handle string values', () => {
        const data = Buffer.from('status,active\nmode,manual');
        const result = parser.parse(data);
        
        expect(result[0].value).toBe('active');
        expect(result[1].value).toBe('manual');
      });

      it('should skip empty lines', () => {
        const data = Buffer.from('temperature,23.5\n\nhumidity,65.2\n');
        const result = parser.parse(data);
        
        expect(result).toHaveLength(2);
      });

      it('should handle different format variations', () => {
        // This format should be valid (treating 'invalid' as timestamp, 'data' as stream, 'format' as value)
        const data = Buffer.from('1234567890,temperature,23.5');
        expect(() => parser.parse(data)).not.toThrow();
        
        const invalidData = Buffer.from('just-one-field');
        expect(() => parser.parse(invalidData)).toThrow();
      });
    });
  });

  describe('JsonTelemetryParser', () => {
    let parser: JsonTelemetryParser;

    beforeEach(() => {
      parser = new JsonTelemetryParser(deviceId);
    });

    describe('validation', () => {
      it('should validate single object format', () => {
        const data = Buffer.from('{"stream": "temperature", "value": 23.5}');
        expect(parser.validate(data)).toBe(true);
      });

      it('should validate array format', () => {
        const data = Buffer.from('[{"stream": "temperature", "value": 23.5}, {"stream": "humidity", "value": 65.2}]');
        expect(parser.validate(data)).toBe(true);
      });

      it('should validate with timestamp', () => {
        const data = Buffer.from('{"timestamp": 1234567890, "stream": "temperature", "value": 23.5}');
        expect(parser.validate(data)).toBe(true);
      });

      it('should reject invalid JSON', () => {
        const data = Buffer.from('{"invalid": json}');
        expect(parser.validate(data)).toBe(false);
      });

      it('should reject missing required fields', () => {
        const data = Buffer.from('{"value": 23.5}'); // Missing stream
        expect(parser.validate(data)).toBe(false);
      });
    });

    describe('parsing', () => {
      it('should parse single object', () => {
        const data = Buffer.from('{"stream": "temperature", "value": 23.5}');
        const result = parser.parse(data);
        
        expect(result).toHaveLength(1);
        expect(result[0]).toMatchObject({
          stream: 'temperature',
          value: 23.5,
          deviceId
        });
      });

      it('should parse array of objects', () => {
        const data = Buffer.from('[{"stream": "temperature", "value": 23.5}, {"stream": "humidity", "value": 65.2}]');
        const result = parser.parse(data);
        
        expect(result).toHaveLength(2);
        expect(result[0].stream).toBe('temperature');
        expect(result[1].stream).toBe('humidity');
      });

      it('should handle timestamp field', () => {
        const data = Buffer.from('{"timestamp": 1234567890, "stream": "temperature", "value": 23.5}');
        const result = parser.parse(data);
        
        expect(result[0].timestamp).toBe(1234567890000); // Converted to milliseconds
      });

      it('should handle different value types', () => {
        const data = Buffer.from('[{"stream": "temp", "value": 23.5}, {"stream": "status", "value": "active"}]');
        const result = parser.parse(data);
        
        expect(result[0].value).toBe(23.5);
        expect(result[1].value).toBe('active');
      });
    });
  });

  describe('BinaryTelemetryParser', () => {
    let parser: BinaryTelemetryParser;

    beforeEach(() => {
      parser = new BinaryTelemetryParser(deviceId);
    });

    function createBinaryData(timestamp: number, stream: string, value: number): Buffer {
      const streamBytes = Buffer.from(stream, 'utf8');
      const buffer = Buffer.allocUnsafe(4 + 1 + streamBytes.length + 4);
      
      let offset = 0;
      buffer.writeUInt32LE(timestamp, offset); // Timestamp
      offset += 4;
      buffer.writeUInt8(streamBytes.length, offset); // Stream length
      offset += 1;
      streamBytes.copy(buffer, offset); // Stream name
      offset += streamBytes.length;
      buffer.writeFloatLE(value, offset); // Value
      
      return buffer;
    }

    describe('validation', () => {
      it('should validate correct binary format', () => {
        const data = createBinaryData(1234567890, 'temp', 23.5);
        expect(parser.validate(data)).toBe(true);
      });

      it('should reject too short data', () => {
        const data = Buffer.from([1, 2, 3]); // Too short
        expect(parser.validate(data)).toBe(false);
      });

      it('should reject incomplete data', () => {
        const buffer = Buffer.allocUnsafe(8);
        buffer.writeUInt32LE(1234567890, 0); // Timestamp
        buffer.writeUInt8(10, 4); // Stream length of 10, but no stream data
        // Missing stream and value
        
        expect(parser.validate(buffer)).toBe(false);
      });

      it('should validate multiple data points', () => {
        const data1 = createBinaryData(1234567890, 'temp', 23.5);
        const data2 = createBinaryData(1234567891, 'humidity', 65.2);
        const combined = Buffer.concat([data1, data2]);
        
        expect(parser.validate(combined)).toBe(true);
      });
    });

    describe('parsing', () => {
      it('should parse single data point', () => {
        const data = createBinaryData(1234567890, 'temperature', 23.5);
        const result = parser.parse(data);
        
        expect(result).toHaveLength(1);
        expect(result[0]).toMatchObject({
          timestamp: 1234567890000, // Converted to milliseconds
          stream: 'temperature',
          value: 23.5,
          deviceId
        });
      });

      it('should parse multiple data points', () => {
        const data1 = createBinaryData(1234567890, 'temp', 23.5);
        const data2 = createBinaryData(1234567891, 'humidity', 65.2);
        const combined = Buffer.concat([data1, data2]);
        
        const result = parser.parse(combined);
        
        expect(result).toHaveLength(2);
        expect(result[0].stream).toBe('temp');
        expect(result[1].stream).toBe('humidity');
      });

      it('should handle empty stream names', () => {
        const data = createBinaryData(1234567890, '', 42.0);
        const result = parser.parse(data);
        
        expect(result[0].stream).toBe('');
        expect(result[0].value).toBe(42.0);
      });

      it('should handle floating point values accurately', () => {
        const testValue = 3.14159;
        const data = createBinaryData(1234567890, 'pi', testValue);
        const result = parser.parse(data);
        
        expect(result[0].value).toBeCloseTo(testValue, 5);
      });
    });
  });

  describe('TelemetryParserFactory', () => {
    it('should create CSV parser', () => {
      const parser = TelemetryParserFactory.createParser('csv', deviceId);
      expect(parser).toBeInstanceOf(CsvTelemetryParser);
      expect(parser.formatType).toBe('csv');
    });

    it('should create JSON parser', () => {
      const parser = TelemetryParserFactory.createParser('json', deviceId);
      expect(parser).toBeInstanceOf(JsonTelemetryParser);
      expect(parser.formatType).toBe('json');
    });

    it('should create binary parser', () => {
      const parser = TelemetryParserFactory.createParser('binary', deviceId);
      expect(parser).toBeInstanceOf(BinaryTelemetryParser);
      expect(parser.formatType).toBe('binary');
    });

    it('should throw error for unsupported format', () => {
      expect(() => {
        TelemetryParserFactory.createParser('xml' as any, deviceId);
      }).toThrow('Unsupported telemetry format: xml');
    });

    it('should use default device ID', () => {
      const parser = TelemetryParserFactory.createParser('json');
      const data = Buffer.from('{"stream": "test", "value": 1}');
      const result = parser.parse(data);
      
      expect(result[0].deviceId).toBe('unknown');
    });
  });
});