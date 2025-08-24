// @multi-controller/transport-serial - Serial transport implementation
import { EventEmitter } from 'eventemitter3';
import type { ITransport } from '@multi-controller/core';
import { SerialPort } from 'serialport';
import pRetry from 'p-retry';

export class SerialTransport extends EventEmitter implements ITransport {
  id: string;
  name: string;
  type: 'serial' = 'serial';
  private port?: SerialPort;
  private path: string;
  private baudRate: number;

  constructor(path: string, baudRate: number = 115200) {
    super();
    this.path = path;
    this.baudRate = baudRate;
    this.id = `serial-${path}`;
    this.name = `Serial ${path}`;
  }

  async connect(): Promise<void> {
    await pRetry(async () => {
      this.port = new SerialPort({
        path: this.path,
        baudRate: this.baudRate,
        autoOpen: false
      });

      return new Promise<void>((resolve, reject) => {
        this.port!.open((err) => {
          if (err) reject(err);
          else resolve();
        });
      });
    }, {
      retries: 3,
      minTimeout: 1000
    });

    this.port!.on('data', (data: Buffer) => {
      this.emit('data', data);
    });

    this.port!.on('error', (err: Error) => {
      this.emit('error', err);
    });
  }

  async disconnect(): Promise<void> {
    if (this.port && this.port.isOpen) {
      return new Promise((resolve) => {
        this.port!.close(() => resolve());
      });
    }
  }

  async send(data: Buffer): Promise<void> {
    if (!this.port || !this.port.isOpen) {
      throw new Error('Serial port not connected');
    }

    return new Promise((resolve, reject) => {
      this.port!.write(data, (err) => {
        if (err) reject(err);
        else resolve();
      });
    });
  }

  onData(handler: (data: Buffer) => void): void {
    this.on('data', handler);
  }

  onError(handler: (error: Error) => void): void {
    this.on('error', handler);
  }
}

export const VERSION = '1.0.0';