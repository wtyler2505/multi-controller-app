import { RingBuffer } from '../ring-buffer';

describe('RingBuffer', () => {
  let buffer: RingBuffer<number>;

  beforeEach(() => {
    buffer = new RingBuffer<number>(5); // Small buffer for testing
  });

  describe('constructor', () => {
    it('should create a buffer with correct capacity', () => {
      expect(buffer.capacity).toBe(5);
      expect(buffer.size).toBe(0);
      expect(buffer.isFull).toBe(false);
    });

    it('should throw error for invalid capacity', () => {
      expect(() => new RingBuffer(0)).toThrow('Ring buffer capacity must be positive');
      expect(() => new RingBuffer(-1)).toThrow('Ring buffer capacity must be positive');
    });

    it('should create buffer with default capacity', () => {
      const defaultBuffer = new RingBuffer<number>();
      expect(defaultBuffer.capacity).toBe(2000);
    });
  });

  describe('push operation', () => {
    it('should add items to empty buffer', () => {
      expect(buffer.push(1)).toBe(true);
      expect(buffer.size).toBe(1);
      expect(buffer.isFull).toBe(false);
    });

    it('should fill buffer to capacity', () => {
      for (let i = 1; i <= 5; i++) {
        expect(buffer.push(i)).toBe(true);
      }
      expect(buffer.size).toBe(5);
      expect(buffer.isFull).toBe(true);
    });

    it('should overwrite oldest item when buffer is full', () => {
      // Fill buffer
      for (let i = 1; i <= 5; i++) {
        buffer.push(i);
      }
      
      // Add one more item - should overwrite oldest (1)
      expect(buffer.push(6)).toBe(false); // Returns false indicating overwrite
      expect(buffer.size).toBe(5);
      expect(buffer.isFull).toBe(true);
      
      // Verify oldest item is now 2, not 1
      expect(buffer.peek()).toBe(2);
    });
  });

  describe('pop operation', () => {
    it('should return undefined for empty buffer', () => {
      expect(buffer.pop()).toBeUndefined();
    });

    it('should return oldest item and reduce size', () => {
      buffer.push(1);
      buffer.push(2);
      
      expect(buffer.pop()).toBe(1);
      expect(buffer.size).toBe(1);
      expect(buffer.pop()).toBe(2);
      expect(buffer.size).toBe(0);
    });

    it('should handle FIFO correctly', () => {
      const values = [10, 20, 30, 40, 50];
      values.forEach(v => buffer.push(v));
      
      for (const expectedValue of values) {
        expect(buffer.pop()).toBe(expectedValue);
      }
      expect(buffer.size).toBe(0);
    });
  });

  describe('peek operation', () => {
    it('should return undefined for empty buffer', () => {
      expect(buffer.peek()).toBeUndefined();
    });

    it('should return oldest item without removing it', () => {
      buffer.push(1);
      buffer.push(2);
      
      expect(buffer.peek()).toBe(1);
      expect(buffer.size).toBe(2); // Size should not change
      expect(buffer.peek()).toBe(1); // Should still return same item
    });
  });

  describe('clear operation', () => {
    it('should reset buffer to empty state', () => {
      buffer.push(1);
      buffer.push(2);
      buffer.push(3);
      
      buffer.clear();
      
      expect(buffer.size).toBe(0);
      expect(buffer.isFull).toBe(false);
      expect(buffer.peek()).toBeUndefined();
      expect(buffer.pop()).toBeUndefined();
    });
  });

  describe('toArray operation', () => {
    it('should return empty array for empty buffer', () => {
      expect(buffer.toArray()).toEqual([]);
    });

    it('should return items in insertion order (oldest to newest)', () => {
      const values = [1, 2, 3, 4];
      values.forEach(v => buffer.push(v));
      
      expect(buffer.toArray()).toEqual(values);
    });

    it('should handle wraparound correctly', () => {
      // Fill buffer completely
      for (let i = 1; i <= 5; i++) {
        buffer.push(i);
      }
      
      // Add more items to cause wraparound
      buffer.push(6);
      buffer.push(7);
      
      // Should contain [3, 4, 5, 6, 7] (oldest to newest)
      expect(buffer.toArray()).toEqual([3, 4, 5, 6, 7]);
    });
  });

  describe('getRecent operation', () => {
    beforeEach(() => {
      // Add test data: [1, 2, 3, 4]
      [1, 2, 3, 4].forEach(v => buffer.push(v));
    });

    it('should return empty array for count <= 0', () => {
      expect(buffer.getRecent(0)).toEqual([]);
      expect(buffer.getRecent(-1)).toEqual([]);
    });

    it('should return most recent items in reverse order (newest to oldest)', () => {
      expect(buffer.getRecent(2)).toEqual([4, 3]);
      expect(buffer.getRecent(3)).toEqual([4, 3, 2]);
    });

    it('should return all items if count exceeds buffer size', () => {
      expect(buffer.getRecent(10)).toEqual([4, 3, 2, 1]);
    });

    it('should return empty array for empty buffer', () => {
      const emptyBuffer = new RingBuffer<number>(5);
      expect(emptyBuffer.getRecent(5)).toEqual([]);
    });

    it('should handle wraparound correctly', () => {
      // Fill buffer: [1, 2, 3, 4, 5]
      buffer.push(5);
      
      // Add more to cause wraparound: [3, 4, 5, 6, 7]
      buffer.push(6);
      buffer.push(7);
      
      expect(buffer.getRecent(3)).toEqual([7, 6, 5]);
      expect(buffer.getRecent(5)).toEqual([7, 6, 5, 4, 3]);
    });
  });

  describe('stress testing', () => {
    it('should handle high volume operations correctly', () => {
      const largeBuffer = new RingBuffer<number>(1000);
      
      // Add 2000 items (double the capacity)
      for (let i = 0; i < 2000; i++) {
        largeBuffer.push(i);
      }
      
      expect(largeBuffer.size).toBe(1000);
      expect(largeBuffer.isFull).toBe(true);
      
      // Should contain items 1000-1999
      const items = largeBuffer.toArray();
      expect(items[0]).toBe(1000);
      expect(items[999]).toBe(1999);
    });

    it('should maintain correct state after mixed operations', () => {
      // Mix of push and pop operations
      buffer.push(1);
      buffer.push(2);
      expect(buffer.pop()).toBe(1);
      
      buffer.push(3);
      buffer.push(4);
      buffer.push(5);
      buffer.push(6);
      
      expect(buffer.size).toBe(5); // Should be 5: popped 1, so remaining are 2,3,4,5,6
      expect(buffer.toArray()).toEqual([2, 3, 4, 5, 6]);
    });
  });

  describe('edge cases', () => {
    it('should handle single-item buffer', () => {
      const singleBuffer = new RingBuffer<string>(1);
      
      expect(singleBuffer.push('a')).toBe(true);
      expect(singleBuffer.isFull).toBe(true);
      expect(singleBuffer.push('b')).toBe(false); // Overwrite
      
      expect(singleBuffer.toArray()).toEqual(['b']);
      expect(singleBuffer.getRecent(1)).toEqual(['b']);
    });

    it('should handle different data types', () => {
      const stringBuffer = new RingBuffer<string>(3);
      const objectBuffer = new RingBuffer<{id: number}>(3);
      
      stringBuffer.push('hello');
      stringBuffer.push('world');
      expect(stringBuffer.toArray()).toEqual(['hello', 'world']);
      
      objectBuffer.push({id: 1});
      objectBuffer.push({id: 2});
      expect(objectBuffer.toArray()).toEqual([{id: 1}, {id: 2}]);
    });
  });
});