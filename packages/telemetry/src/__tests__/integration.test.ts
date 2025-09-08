import { RingBuffer } from '../ring-buffer';
import { TelemetryParserFactory } from '../parsers';

describe('Telemetry Integration', () => {
  describe('Basic functionality', () => {
    it('should create and use ring buffer', () => {
      const buffer = new RingBuffer<number>(10);
      
      buffer.push(1);
      buffer.push(2);
      buffer.push(3);
      
      expect(buffer.size).toBe(3);
      expect(buffer.pop()).toBe(1);
      expect(buffer.size).toBe(2);
    });

    it('should parse CSV data', () => {
      const parser = TelemetryParserFactory.createParser('csv', 'test-device');
      const data = Buffer.from('temperature,23.5\nhumidity,65.2');
      
      expect(parser.validate(data)).toBe(true);
      
      const result = parser.parse(data);
      expect(result).toHaveLength(2);
      expect(result[0].stream).toBe('temperature');
      expect(result[0].value).toBe(23.5);
    });

    it('should parse JSON data', () => {
      const parser = TelemetryParserFactory.createParser('json', 'test-device');
      const data = Buffer.from('{"stream": "temperature", "value": 23.5}');
      
      expect(parser.validate(data)).toBe(true);
      
      const result = parser.parse(data);
      expect(result).toHaveLength(1);
      expect(result[0].stream).toBe('temperature');
      expect(result[0].value).toBe(23.5);
    });

    it('should handle ring buffer overflow', () => {
      const buffer = new RingBuffer<string>(3);
      
      // Fill buffer
      buffer.push('a');
      buffer.push('b');
      buffer.push('c');
      expect(buffer.isFull).toBe(true);
      
      // Overflow - should overwrite oldest
      expect(buffer.push('d')).toBe(false); // false indicates overflow
      expect(buffer.peek()).toBe('b'); // 'a' was overwritten
    });
  });

  describe('Performance characteristics', () => {
    it('should handle high-volume data efficiently', () => {
      const buffer = new RingBuffer<number>(1000);
      const startTime = Date.now();
      
      // Add 10,000 items
      for (let i = 0; i < 10000; i++) {
        buffer.push(i);
      }
      
      const endTime = Date.now();
      const duration = endTime - startTime;
      
      expect(buffer.size).toBe(1000); // Should maintain capacity
      expect(duration).toBeLessThan(100); // Should be fast (< 100ms)
    });

    it('should parse large CSV efficiently', () => {
      const parser = TelemetryParserFactory.createParser('csv');
      
      // Create large CSV data
      const lines = [];
      for (let i = 0; i < 1000; i++) {
        lines.push(`sensor${i % 10},${Math.random() * 100}`);
      }
      const data = Buffer.from(lines.join('\n'));
      
      const startTime = Date.now();
      const result = parser.parse(data);
      const endTime = Date.now();
      
      expect(result).toHaveLength(1000);
      expect(endTime - startTime).toBeLessThan(50); // Should be fast
    });
  });

  describe('Data validation and error handling', () => {
    it('should validate different data formats correctly', () => {
      const csvParser = TelemetryParserFactory.createParser('csv');
      const jsonParser = TelemetryParserFactory.createParser('json');
      const binaryParser = TelemetryParserFactory.createParser('binary');
      
      // Valid data
      expect(csvParser.validate(Buffer.from('temp,23.5'))).toBe(true);
      expect(jsonParser.validate(Buffer.from('{"stream":"temp","value":23.5}'))).toBe(true);
      
      // Invalid data
      expect(csvParser.validate(Buffer.from('invalid'))).toBe(false);
      expect(jsonParser.validate(Buffer.from('invalid json'))).toBe(false);
      expect(binaryParser.validate(Buffer.from('short'))).toBe(false);
    });

    it('should handle empty and malformed data gracefully', () => {
      const parser = TelemetryParserFactory.createParser('csv');
      
      expect(parser.validate(Buffer.from(''))).toBe(false);
      expect(parser.validate(Buffer.from('\n\n\n'))).toBe(false);
      expect(() => parser.parse(Buffer.from('invalid'))).toThrow();
    });
  });
});