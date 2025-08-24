# PowerShell script to migrate existing code to monorepo structure
# Multi-Controller App Monorepo Migration Script

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Multi-Controller App Monorepo Migration" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check if pnpm is installed
if (!(Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Host "Installing pnpm globally..." -ForegroundColor Yellow
    npm install -g pnpm
}

# Check if turbo is installed globally (optional)
if (!(Get-Command turbo -ErrorAction SilentlyContinue)) {
    Write-Host "Installing turbo globally (optional)..." -ForegroundColor Yellow
    npm install -g turbo
}

Write-Host "`nCreating source directories for packages..." -ForegroundColor Green

# Create source directories
$packages = @(
    "packages/core/src",
    "packages/ui/src",
    "packages/telemetry/src",
    "drivers/arduino/src",
    "drivers/esp32/src",
    "drivers/esp8266/src",
    "drivers/riorand/src",
    "drivers/raspberry-pi/src",
    "transports/serial/src",
    "transports/tcp-udp/src",
    "transports/ssh/src",
    "apps/desktop/src",
    "tools/scripts",
    "tools/testing"
)

foreach ($dir in $packages) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    Write-Host "  Created $dir" -ForegroundColor Gray
}

Write-Host "`nCreating TypeScript configurations..." -ForegroundColor Green

# Create tsconfig for each package
$tsconfigContent = @'
{
  "extends": "@multi-controller/tsconfig/base.json",
  "compilerOptions": {
    "rootDir": "./src",
    "outDir": "./dist"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts", "**/*.spec.ts"]
}
'@

$packagesWithTs = @(
    "packages/core",
    "packages/ui",
    "packages/telemetry",
    "drivers/arduino",
    "drivers/esp32",
    "drivers/esp8266",
    "drivers/riorand",
    "drivers/raspberry-pi",
    "transports/serial",
    "transports/tcp-udp",
    "transports/ssh",
    "apps/desktop"
)

foreach ($pkg in $packagesWithTs) {
    $tsconfigPath = Join-Path $pkg "tsconfig.json"
    Set-Content -Path $tsconfigPath -Value $tsconfigContent
    Write-Host "  Created $tsconfigPath" -ForegroundColor Gray
}

Write-Host "`nCreating example source files..." -ForegroundColor Green

# Create example index files
$coreIndexContent = @'
// @multi-controller/core - Main application core
export * from './interfaces';
export * from './device-manager';
export * from './transport-manager';
export * from './utils';

export const VERSION = '1.0.0';
'@

Set-Content -Path "packages/core/src/index.ts" -Value $coreIndexContent

# Create interfaces file
$interfacesContent = @'
// Core interfaces for Multi-Controller App

export interface ITransport {
  id: string;
  name: string;
  type: 'serial' | 'tcp' | 'udp' | 'ssh';
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  send(data: Buffer): Promise<void>;
  onData(handler: (data: Buffer) => void): void;
  onError(handler: (error: Error) => void): void;
}

export interface IDeviceDriver {
  name: string;
  version: string;
  supportedTransports: string[];
  probeAsync(transport: ITransport): Promise<boolean>;
  openAsync(transport: ITransport): Promise<IDeviceSession>;
}

export interface IDeviceSession {
  id: string;
  deviceName: string;
  invokeAsync(endpoint: string, args: any[]): Promise<any>;
  subscribeAsync(stream: string, handler: (data: Buffer) => void): Promise<() => void>;
  closeAsync(): Promise<void>;
}

export interface IDeviceCapabilities {
  digitalIO?: boolean;
  analogIO?: boolean;
  pwm?: boolean;
  i2c?: boolean;
  spi?: boolean;
  servo?: boolean;
}
'@

Set-Content -Path "packages/core/src/interfaces.ts" -Value $interfacesContent

Write-Host "  Created core interfaces" -ForegroundColor Gray

# Move existing C# project to apps/desktop
if (Test-Path "app/MultiControllerApp.csproj") {
    Write-Host "`nMoving C# project to apps/desktop..." -ForegroundColor Green
    Move-Item -Path "app/MultiControllerApp.csproj" -Destination "apps/desktop/" -Force
    Move-Item -Path "app/Program.cs" -Destination "apps/desktop/" -Force
    Write-Host "  Moved C# project files" -ForegroundColor Gray
}

Write-Host "`nInstalling dependencies..." -ForegroundColor Green
Write-Host "  Running: pnpm install" -ForegroundColor Gray

# Install dependencies
pnpm install

Write-Host "`nMigration preparation complete!" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "  1. Run 'pnpm build' to build all packages" -ForegroundColor White
Write-Host "  2. Run 'pnpm dev' to start development mode" -ForegroundColor White
Write-Host "  3. Run 'pnpm graph' to visualize the dependency graph" -ForegroundColor White
Write-Host "  4. Start migrating your existing code to the appropriate packages" -ForegroundColor White

Write-Host "`nPackage structure:" -ForegroundColor Cyan
Write-Host "  - packages/core: Core business logic" -ForegroundColor White
Write-Host "  - packages/ui: UI components" -ForegroundColor White
Write-Host "  - packages/telemetry: Telemetry and monitoring" -ForegroundColor White
Write-Host "  - drivers/*: Device driver implementations" -ForegroundColor White
Write-Host "  - transports/*: Transport layer implementations" -ForegroundColor White
Write-Host "  - apps/desktop: Main desktop application" -ForegroundColor White

Write-Host "`nMonorepo benefits:" -ForegroundColor Cyan
Write-Host "  - Shared dependencies and configurations" -ForegroundColor White
Write-Host "  - Parallel builds with Turborepo caching" -ForegroundColor White
Write-Host "  - Affected package detection for CI/CD" -ForegroundColor White
Write-Host "  - Consistent tooling across all packages" -ForegroundColor White
Write-Host "  - Easy cross-package refactoring" -ForegroundColor White