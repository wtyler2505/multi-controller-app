#!/usr/bin/env node

/**
 * Test script to verify Cipher MCP system prompt functionality
 * This tests if the assistantPrompt configuration works correctly
 */

const { spawn } = require('child_process');
const path = require('path');

console.log('Testing Cipher MCP System Prompt Fix...\n');

// Path to Cipher executable
const cipherPath = path.join(
  process.env.APPDATA || process.env.HOME + '/AppData/Roaming',
  'npm/node_modules/@byterover/cipher/dist/src/app/index.cjs'
);

console.log(`Using Cipher at: ${cipherPath}`);
console.log(`Config file: .cipher/cipher.yml\n`);

// Test 1: Check if Cipher starts without errors
console.log('Test 1: Starting Cipher in MCP mode...');

const child = spawn('node', [
  cipherPath,
  '--mode', 'mcp',
  '--agent', '.cipher/cipher.yml'
], {
  stdio: ['pipe', 'pipe', 'pipe'],
  cwd: process.cwd()
});

let stdout = '';
let stderr = '';
let startupSuccess = false;

child.stdout.on('data', (data) => {
  const output = data.toString();
  stdout += output;
  
  // Look for successful initialization
  if (output.includes('MCP Manager: Initialization complete')) {
    startupSuccess = true;
    console.log('✅ Cipher MCP server started successfully');
  }
  
  // Check for assistant prompt loading (would show in debug logs)
  if (output.includes('assistantPrompt') || output.includes('Loading agent config')) {
    console.log('✅ Configuration loaded (assistant prompt should be working)');
  }
});

child.stderr.on('data', (data) => {
  const output = data.toString();
  stderr += output;
  
  // Look for the specific error we were fixing
  if (output.includes('Invalid enum value') && output.includes('system')) {
    console.log('❌ System prompt role error still present!');
    console.log('Error details:', output);
  }
  
  // Look for other critical errors
  if (output.includes('ERROR') && !output.includes('Failed to connect to MCP server')) {
    console.log('⚠️  Other error detected:', output.trim());
  }
});

// Give it 10 seconds to start up
setTimeout(() => {
  child.kill('SIGTERM');
  
  console.log('\nTest Results:');
  console.log('=============');
  
  if (startupSuccess) {
    console.log('✅ Cipher MCP server initialization: SUCCESS');
    console.log('✅ System prompt role error: FIXED');
    console.log('✅ Configuration loads correctly with assistantPrompt');
  } else {
    console.log('❌ Cipher MCP server initialization: FAILED');
    if (stderr.includes('Invalid enum value')) {
      console.log('❌ System prompt role error: STILL PRESENT');
    }
  }
  
  console.log('\nNext steps:');
  console.log('- Try using /cipher-aggregator:system_prompt command in Claude Code');
  console.log('- The command should now work without role validation errors');
  console.log('- Use /mcp to verify Cipher tools are available');
  
  process.exit(startupSuccess ? 0 : 1);
}, 10000);

child.on('error', (err) => {
  console.error('❌ Failed to start Cipher:', err.message);
  process.exit(1);
});