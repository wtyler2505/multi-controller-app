// @multi-controller/transport-ssh - SSH transport implementation
import { EventEmitter } from 'eventemitter3';
import type { ITransport } from '@multi-controller/core';
import { Client } from 'ssh2';
import pRetry from 'p-retry';

export class SshTransport extends EventEmitter implements ITransport {
  id: string;
  name: string;
  type: 'ssh' = 'ssh';
  private client?: Client;
  private stream?: any;
  private host: string;
  private port: number;
  private username: string;
  private password?: string;
  private privateKey?: Buffer;

  constructor(config: {
    host: string;
    port?: number;
    username: string;
    password?: string;
    privateKey?: Buffer;
  }) {
    super();
    this.host = config.host;
    this.port = config.port || 22;
    this.username = config.username;
    this.password = config.password;
    this.privateKey = config.privateKey;
    this.id = `ssh-${this.host}:${this.port}`;
    this.name = `SSH ${this.host}:${this.port}`;
  }

  async connect(): Promise<void> {
    await pRetry(async () => {
      return new Promise<void>((resolve, reject) => {
        this.client = new Client();
        
        this.client.on('ready', () => {
          this.client!.shell((err, stream) => {
            if (err) {
              reject(err);
              return;
            }
            
            this.stream = stream;
            
            stream.on('data', (data: Buffer) => {
              this.emit('data', data);
            });
            
            stream.on('error', (err: Error) => {
              this.emit('error', err);
            });
            
            resolve();
          });
        });

        this.client.on('error', (err) => {
          reject(err);
        });

        const connectConfig: any = {
          host: this.host,
          port: this.port,
          username: this.username
        };

        if (this.password) {
          connectConfig.password = this.password;
        } else if (this.privateKey) {
          connectConfig.privateKey = this.privateKey;
        }

        this.client.connect(connectConfig);
      });
    }, {
      retries: 3,
      minTimeout: 1000
    });
  }

  async disconnect(): Promise<void> {
    if (this.stream) {
      this.stream.close();
      this.stream = undefined;
    }
    if (this.client) {
      this.client.end();
      this.client = undefined;
    }
  }

  async send(data: Buffer): Promise<void> {
    if (!this.stream) {
      throw new Error('SSH stream not connected');
    }

    return new Promise((resolve, reject) => {
      this.stream.write(data, (err?: Error) => {
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