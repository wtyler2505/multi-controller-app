import type { IRingBuffer } from './types';

/**
 * Thread-safe ring buffer implementation for telemetry data
 * Uses a fixed-size circular buffer with overflow handling
 */
export class RingBuffer<T> implements IRingBuffer<T> {
  private buffer: T[];
  private head: number = 0;
  private tail: number = 0;
  private count: number = 0;
  private readonly _capacity: number;

  constructor(capacity: number = 2000) {
    if (capacity <= 0) {
      throw new Error('Ring buffer capacity must be positive');
    }
    this._capacity = capacity;
    this.buffer = new Array(capacity);
  }

  get capacity(): number {
    return this._capacity;
  }

  get size(): number {
    return this.count;
  }

  get isFull(): boolean {
    return this.count === this._capacity;
  }

  /**
   * Add an item to the buffer
   * Returns true if item was added, false if buffer was full and oldest item was overwritten
   */
  push(item: T): boolean {
    const wasOverwritten = this.isFull;
    
    this.buffer[this.tail] = item;
    this.tail = (this.tail + 1) % this._capacity;
    
    if (this.isFull) {
      // Buffer is full, move head to maintain circular nature
      this.head = (this.head + 1) % this._capacity;
    } else {
      this.count++;
    }
    
    return !wasOverwritten;
  }

  /**
   * Remove and return the oldest item from the buffer
   */
  pop(): T | undefined {
    if (this.count === 0) {
      return undefined;
    }
    
    const item = this.buffer[this.head];
    this.head = (this.head + 1) % this._capacity;
    this.count--;
    
    return item;
  }

  /**
   * Return the oldest item without removing it
   */
  peek(): T | undefined {
    if (this.count === 0) {
      return undefined;
    }
    return this.buffer[this.head];
  }

  /**
   * Clear all items from the buffer
   */
  clear(): void {
    this.head = 0;
    this.tail = 0;
    this.count = 0;
  }

  /**
   * Convert buffer contents to array (oldest to newest)
   */
  toArray(): T[] {
    if (this.count === 0) {
      return [];
    }
    
    const result: T[] = [];
    let index = this.head;
    
    for (let i = 0; i < this.count; i++) {
      result.push(this.buffer[index]);
      index = (index + 1) % this._capacity;
    }
    
    return result;
  }

  /**
   * Get the most recent N items (newest to oldest)
   */
  getRecent(count: number): T[] {
    if (count <= 0 || this.count === 0) {
      return [];
    }
    
    const actualCount = Math.min(count, this.count);
    const result: T[] = [];
    
    // Start from the most recent item (just before tail)
    let index = (this.tail - 1 + this._capacity) % this._capacity;
    
    for (let i = 0; i < actualCount; i++) {
      result.push(this.buffer[index]);
      index = (index - 1 + this._capacity) % this._capacity;
    }
    
    return result;
  }
}