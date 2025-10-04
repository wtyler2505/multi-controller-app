// Desktop application main entry point with telemetry integration
import { TelemetryCollector, RingBuffer, TelemetryParserFactory } from '@multi-controller/telemetry';
import { SerialTransport } from '@multi-controller/transport-serial';

// Example telemetry integration
export class DesktopTelemetryManager {
  private collector: TelemetryCollector;

  constructor() {
    this.collector = new TelemetryCollector({
      samplingRate: 100, // 100 Hz
      bufferCapacity: 2000,
      decimationFactor: 10,
      enableErrorCorrection: true,
      maxRetries: 3
    });

    this.setupEventHandlers();
  }

  private setupEventHandlers(): void {
    this.collector.on('started', () => {
      console.log('Telemetry collection started');
    });

    this.collector.on('bufferOverflow', (stream) => {
      console.warn(`Buffer overflow for stream: ${stream}`);
    });

    this.collector.on('transportError', ({ transportId, error }) => {
      console.error(`Transport error for ${transportId}:`, error);
    });

    this.collector.on('statistics', (stats) => {
      console.log('Telemetry stats:', stats);
    });
  }

  async addDevice(portPath: string, formatType: 'csv' | 'json' | 'binary' = 'json'): Promise<void> {
    const transport = new SerialTransport(portPath, 115200);
    await transport.connect();
    
    this.collector.addTransport(transport, formatType);
    console.log(`Added device on ${portPath} with format ${formatType}`);
  }

  subscribeToStream(streamName: string, callback: (data: any[]) => void): void {
    this.collector.subscribe(streamName, callback);
  }

  getLatestData(streamName: string, count: number = 100): any[] {
    return this.collector.getLatestData(streamName, count);
  }

  getDecimatedData(streamName: string): any[] {
    return this.collector.getDecimatedData(streamName);
  }

  async start(): Promise<void> {
    await this.collector.start();
  }

  async stop(): Promise<void> {
    await this.collector.stop();
  }

  getStatistics(): any {
    return this.collector.getStatistics();
  }
}

// Example usage demonstration
export async function demonstrateTelemetryIntegration(): Promise<void> {
  console.log('Starting telemetry demonstration...');
  
  const manager = new DesktopTelemetryManager();
  
  // Subscribe to temperature data
  manager.subscribeToStream('temperature', (dataPoints) => {
    console.log('Temperature data:', dataPoints.map(p => ({ timestamp: p.timestamp, value: p.value })));
  });

  // Subscribe to humidity data
  manager.subscribeToStream('humidity', (dataPoints) => {
    console.log('Humidity data:', dataPoints.map(p => ({ timestamp: p.timestamp, value: p.value })));
  });

  await manager.start();
  
  // Example: Add mock data simulation
  simulateMockDevices(manager);
  
  // Let it run for a few seconds
  setTimeout(async () => {
    console.log('Final statistics:', manager.getStatistics());
    await manager.stop();
  }, 5000);
}

// Mock device simulation for testing
function simulateMockDevices(manager: DesktopTelemetryManager): void {
  // This would normally be handled by real device connections
  // For demonstration, we'll show how the data flows work
  console.log('Mock device simulation - in real usage, this would be actual device data');
  
  setInterval(() => {
    // Simulate temperature and humidity readings
    const tempValue = 20 + Math.random() * 10;
    const humidityValue = 40 + Math.random() * 20;
    
    console.log(`Simulated readings - Temp: ${tempValue.toFixed(1)}°C, Humidity: ${humidityValue.toFixed(1)}%`);
  }, 1000);
}

export const VERSION = '1.0.0';
