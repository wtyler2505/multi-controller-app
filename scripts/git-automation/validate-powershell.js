#!/usr/bin/env node

/**
 * PowerShell Script Validator
 * Validates PowerShell scripts for compatibility issues that cause failures
 */

const fs = require('fs');
const path = require('path');

// Colors for terminal output
const colors = {
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  reset: '\x1b[0m'
};

/**
 * Validate PowerShell script for compatibility issues
 */
function validatePowerShellScript(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  const issues = [];
  const warnings = [];
  
  // Check for emojis (most common issue)
  const emojiRegex = /[\u{1F300}-\u{1F9FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]/gu;
  const emojis = content.match(emojiRegex);
  if (emojis) {
    issues.push(`Contains ${emojis.length} emoji character(s): ${emojis.join(', ')}`);
  }
  
  // Check for box-drawing characters
  const boxChars = content.match(/[╔╗╚╝═║╠╣╦╩╬│─┌┐└┘├┤┬┴┼]/g);
  if (boxChars) {
    issues.push(`Contains ${boxChars.length} box-drawing character(s)`);
  }
  
  // Check for other problematic Unicode
  const nonAscii = content.match(/[^\x00-\x7F]/g);
  if (nonAscii) {
    // Filter out already reported characters
    const filtered = nonAscii.filter(char => {
      const code = char.charCodeAt(0);
      return !(code >= 0x1F300 && code <= 0x1F9FF) && // Not emoji
             !(code >= 0x2600 && code <= 0x27BF) &&   // Not symbols
             !boxChars?.includes(char);                // Not box-drawing
    });
    
    if (filtered.length > 0) {
      issues.push(`Contains ${filtered.length} other non-ASCII character(s)`);
      
      // Show first few problematic characters
      const samples = [...new Set(filtered)].slice(0, 5);
      samples.forEach(char => {
        const code = char.charCodeAt(0).toString(16).toUpperCase();
        issues.push(`  - Character '${char}' (U+${code.padStart(4, '0')})`);
      });
    }
  }
  
  // Check for here-strings (can cause parsing issues)
  if (/@["'][\s\S]*?["']@/.test(content)) {
    warnings.push('Contains here-strings (@" "@) that may cause parsing issues');
  }
  
  // Check for specific problematic patterns
  const problematicPatterns = [
    { pattern: /Write-Host\s+["'].*?[✓✗✅❌⚠️].*?["']/g, message: 'Write-Host with emoji symbols' },
    { pattern: /\$.*?\s*=\s*["'].*?[✓✗✅❌⚠️].*?["']/g, message: 'Variable assignment with emoji' },
    { pattern: /echo\s+["'].*?[✓✗✅❌⚠️].*?["']/g, message: 'Echo with emoji symbols' }
  ];
  
  problematicPatterns.forEach(({ pattern, message }) => {
    const matches = content.match(pattern);
    if (matches) {
      issues.push(`${message}: ${matches.length} occurrence(s)`);
    }
  });
  
  // Provide line numbers for issues
  if (issues.length > 0 || warnings.length > 0) {
    const lines = content.split('\n');
    const problemLines = [];
    
    lines.forEach((line, index) => {
      if (/[^\x00-\x7F]/.test(line)) {
        const lineNum = index + 1;
        const nonAsciiInLine = line.match(/[^\x00-\x7F]/g);
        if (nonAsciiInLine) {
          problemLines.push(`  Line ${lineNum}: Contains ${nonAsciiInLine.length} non-ASCII character(s)`);
        }
      }
    });
    
    if (problemLines.length > 0) {
      issues.push('\nProblematic lines:');
      issues.push(...problemLines.slice(0, 10)); // Show first 10 lines
      if (problemLines.length > 10) {
        issues.push(`  ... and ${problemLines.length - 10} more lines`);
      }
    }
  }
  
  return { issues, warnings };
}

/**
 * Suggest fixes for common issues
 */
function suggestFixes() {
  console.log(`\n${colors.blue}Common replacements:${colors.reset}`);
  console.log('  ✅ → [OK]');
  console.log('  ❌ → [ERROR]');
  console.log('  ⚠️ → [WARNING]');
  console.log('  ℹ️ → [INFO]');
  console.log('  ╔══╗ → +==+');
  console.log('  ║  ║ → |  |');
  console.log('  ╚══╝ → +==+');
  console.log('\nUse Find & Replace with these patterns to fix issues quickly.');
}

// CLI usage
if (require.main === module) {
  const scriptPath = process.argv[2];
  
  if (!scriptPath) {
    console.error(`${colors.red}Usage: node validate-powershell.js <script.ps1>${colors.reset}`);
    console.error(`${colors.yellow}Example: node validate-powershell.js scripts/git-automation/sync-status.ps1${colors.reset}`);
    process.exit(1);
  }
  
  if (!fs.existsSync(scriptPath)) {
    console.error(`${colors.red}File not found: ${scriptPath}${colors.reset}`);
    process.exit(1);
  }
  
  if (!scriptPath.endsWith('.ps1')) {
    console.warn(`${colors.yellow}Warning: File doesn't have .ps1 extension${colors.reset}`);
  }
  
  console.log(`${colors.blue}Validating PowerShell script: ${scriptPath}${colors.reset}\n`);
  
  const { issues, warnings } = validatePowerShellScript(scriptPath);
  
  if (issues.length === 0 && warnings.length === 0) {
    console.log(`${colors.green}✅ PowerShell script is compatible!${colors.reset}`);
    console.log('No Unicode or encoding issues detected.');
    process.exit(0);
  }
  
  if (issues.length > 0) {
    console.error(`${colors.red}❌ PowerShell compatibility issues found:${colors.reset}`);
    issues.forEach(issue => console.error(`  ${colors.red}✗${colors.reset} ${issue}`));
  }
  
  if (warnings.length > 0) {
    console.warn(`\n${colors.yellow}⚠️ Warnings:${colors.reset}`);
    warnings.forEach(warning => console.warn(`  ${colors.yellow}!${colors.reset} ${warning}`));
  }
  
  suggestFixes();
  
  process.exit(issues.length > 0 ? 1 : 0);
}

module.exports = { validatePowerShellScript };