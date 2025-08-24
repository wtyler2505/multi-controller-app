// @multi-controller/transport-network - TCP/UDP transport implementation
import { EventEmitter } from 'eventemitter3';
import type { ITransport } from '@multi-controller/core';
import * as net from 'net';
import * as dgram from 'dgram';
import pRetry from 'p-retry';

export class TcpTransport extends EventEmitter implements ITransport {
  id: string;
  name: string;
  type: 'tcp' = 'tcp';
  private socket?: net.Socket;
  private host: string;
  private port: number;

  constructor(host: string, port: number) {
    super();
    this.host = host;
    this.port = port;
    this.id = `tcp-${host}:${port}`;
    this.name = `TCP ${host}:${port}`;
  }

  async connect(): Promise<void> {
    await pRetry(async () => {
      return new Promise<void>((resolve, reject) => {
        this.socket = new net.Socket();
        
        this.socket.connect(this.port, this.host, () => {
          resolve();
        });

        this.socket.on('error', (err) => {
          reject(err);
        });
      });
    }, {
      retries: 3,
      minTimeout: 1000
    });

    this.socket!.on('data', (data: Buffer) => {
      this.emit('data', data);
    });

    this.socket!.on('error', (err: Error) => {
      this.emit('error', err);
    });
  }

  async disconnect(): Promise<void> {
    if (this.socket) {
      this.socket.destroy();
      this.socket = undefined;
    }
  }

  async send(data: Buffer): Promise<void> {
    if (!this.socket) {
      throw new Error('TCP socket not connected');
    }

    return new Promise((resolve, reject) => {
      this.socket!.write(data, (err) => {
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

export class UdpTransport extends EventEmitter implements ITransport {
  id: string;
  name: string;
  type: 'udp' = 'udp';
  private socket?: dgram.Socket;
  private host: string;
  private port: number;

  constructor(host: string, port: number) {
    super();
    this.host = host;
    this.port = port;
    this.id = `udp-${host}:${port}`;
    this.name = `UDP ${host}:${port}`;
  }

  async connect(): Promise<void> {
    this.socket = dgram.createSocket('udp4');
    
    this.socket.on('message', (data: Buffer) => {
      this.emit('data', data);
    });

    this.socket.on('error', (err: Error) => {
      this.emit('error', err);
    });

    return new Promise((resolve) => {
      this.socket!.bind(0, () => {
        resolve();
      });
    });
  }

  async disconnect(): Promise<void> {
    if (this.socket) {
      this.socket.close();
      this.socket = undefined;
    }
  }

  async send(data: Buffer): Promise<void> {
    if (!this.socket) {
      throw new Error('UDP socket not connected');
    }

    return new Promise((resolve, reject) => {
      this.socket!.send(data, this.port, this.host, (err) => {
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