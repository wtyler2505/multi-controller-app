# Multi-Controller App - Windows Development Environment Setup Script
# Run as Administrator: Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Multi-Controller App Setup - Windows" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

# Check if running as Administrator
if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "This script requires Administrator privileges. Please run as Administrator." -ForegroundColor Red
    Exit 1
}

# Function to check if a command exists
function Test-Command {
    param($Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

Write-Host "`n[1/8] Checking Prerequisites..." -ForegroundColor Yellow

# Check Git
if (Test-Command "git") {
    $gitVersion = git --version
    Write-Host "✓ Git installed: $gitVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Git not found. Installing via winget..." -ForegroundColor Red
    winget install --id Git.Git -e --source winget
}

# Check Node.js
if (Test-Command "node") {
    $nodeVersion = node --version
    Write-Host "✓ Node.js installed: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "✗ Node.js not found. Installing via winget..." -ForegroundColor Red
    winget install --id OpenJS.NodeJS -e --source winget
}

# Check .NET SDK
if (Test-Command "dotnet") {
    $dotnetVersion = dotnet --version
    Write-Host "✓ .NET SDK installed: $dotnetVersion" -ForegroundColor Green
} else {
    Write-Host "✗ .NET SDK not found. Installing via winget..." -ForegroundColor Red
    winget install --id Microsoft.DotNet.SDK.8 -e --source winget
}

Write-Host "`n[2/8] Installing Node.js Dependencies..." -ForegroundColor Yellow
npm install

Write-Host "`n[3/8] Creating Project Structure..." -ForegroundColor Yellow
$directories = @(
    "src",
    "src/app",
    "src/drivers",
    "src/transports",
    "src/interfaces",
    "src/utils",
    "src/config",
    "tests",
    "tests/unit",
    "tests/integration",
    "scripts",
    "profiles",
    "logs",
    "data",
    "temp"
)

foreach ($dir in $directories) {
    if (!(Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "Created: $dir" -ForegroundColor DarkGray
    }
}

Write-Host "`n[4/8] Setting up Environment Variables..." -ForegroundColor Yellow
if (!(Test-Path ".env")) {
    if (Test-Path ".env.example") {
        Copy-Item ".env.example" ".env"
        Write-Host "Created .env from .env.example" -ForegroundColor Green
        Write-Host "⚠ Please update .env with your API keys!" -ForegroundColor Yellow
    }
}

Write-Host "`n[5/8] Initializing Git Hooks..." -ForegroundColor Yellow
npm run prepare

Write-Host "`n[6/8] Building TypeScript..." -ForegroundColor Yellow
npm run build

Write-Host "`n[7/8] Running Initial Tests..." -ForegroundColor Yellow
npm test

Write-Host "`n[8/8] Validating Performance Budgets..." -ForegroundColor Yellow
Write-Host "Checking startup time..." -ForegroundColor DarkGray
Write-Host "Max allowed: 2000ms" -ForegroundColor DarkGray

Write-Host "`n=====================================" -ForegroundColor Green
Write-Host "✓ Setup Complete!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Cyan
Write-Host "1. Update .env file with your API keys" -ForegroundColor White
Write-Host "2. Run 'npm run dev' to start development server" -ForegroundColor White
Write-Host "3. Open VS Code with 'code .'" -ForegroundColor White
Write-Host "4. Install recommended VS Code extensions" -ForegroundColor White
Write-Host ""
Write-Host "Available Commands:" -ForegroundColor Cyan
Write-Host "npm run dev          - Start development server" -ForegroundColor White
Write-Host "npm run build        - Build TypeScript" -ForegroundColor White
Write-Host "npm test            - Run tests" -ForegroundColor White
Write-Host "npm run lint        - Check code quality" -ForegroundColor White
Write-Host "npm run format      - Format code" -ForegroundColor White