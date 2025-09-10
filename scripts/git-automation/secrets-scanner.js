#!/usr/bin/env node

/**
 * Secrets Scanner for Git Pre-commit Hook
 * Prevents accidental commit of secrets, API keys, and credentials
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Load configuration
const configPath = path.join(__dirname, '../../.gitmeta/config/secrets-patterns.json');
const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));

// Colors for terminal output
const colors = {
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  green: '\x1b[32m',
  reset: '\x1b[0m'
};

/**
 * Get list of staged files
 */
function getStagedFiles() {
  try {
    const output = execSync('git diff --cached --name-only', { encoding: 'utf8' });
    return output.split('\n').filter(file => file.length > 0);
  } catch (error) {
    console.error('Error getting staged files:', error.message);
    return [];
  }
}

/**
 * Check if file should be scanned based on patterns
 */
function shouldScanFile(filePath) {
  const fileName = path.basename(filePath);
  
  // Check if file matches any pattern that should be blocked
  for (const pattern of config.files) {
    if (pattern.includes('*')) {
      const regex = new RegExp('^' + pattern.replace(/\*/g, '.*') + '$');
      if (regex.test(fileName)) {
        return { scan: false, block: true, reason: `File matches blocked pattern: ${pattern}` };
      }
    } else if (fileName === pattern) {
      return { scan: false, block: true, reason: `File is in blocked list: ${pattern}` };
    }
  }
  
  // Skip binary files
  const binaryExtensions = ['.exe', '.dll', '.so', '.dylib', '.jpg', '.jpeg', '.png', '.gif', '.pdf', '.zip', '.tar', '.gz'];
  if (binaryExtensions.some(ext => filePath.toLowerCase().endsWith(ext))) {
    return { scan: false, block: false };
  }
  
  return { scan: true, block: false };
}

/**
 * Scan file content for secrets
 */
function scanFileContent(filePath) {
  const issues = [];
  
  try {
    // Get the staged content (not the working directory content)
    const content = execSync(`git show :${filePath}`, { encoding: 'utf8', maxBuffer: 10 * 1024 * 1024 });
    const lines = content.split('\n');
    
    lines.forEach((line, lineNum) => {
      // Check for simple pattern matches
      for (const pattern of config.patterns) {
        if (line.toUpperCase().includes(pattern)) {
          // Check if it's likely a real secret (has an assignment)
          if (line.includes('=') || line.includes(':')) {
            issues.push({
              file: filePath,
              line: lineNum + 1,
              pattern: pattern,
              content: line.substring(0, 100) // Truncate for safety
            });
          }
        }
      }
      
      // Check regex patterns
      for (const regexPattern of config.regex_patterns) {
        const regex = new RegExp(regexPattern, 'gi');
        if (regex.test(line)) {
          issues.push({
            file: filePath,
            line: lineNum + 1,
            pattern: 'Regex pattern',
            content: line.substring(0, 100).replace(regex, '[REDACTED]')
          });
        }
      }
    });
  } catch (error) {
    // File might be deleted or binary
    if (!error.message.includes('does not exist')) {
      console.warn(`Warning: Could not scan ${filePath}: ${error.message}`);
    }
  }
  
  return issues;
}

/**
 * Main scanning function
 */
function main() {
  console.log(`${colors.yellow}🔍 Scanning for secrets and sensitive data...${colors.reset}`);
  
  const stagedFiles = getStagedFiles();
  if (stagedFiles.length === 0) {
    console.log(`${colors.green}✅ No staged files to scan${colors.reset}`);
    process.exit(0);
  }
  
  let totalIssues = [];
  let blockedFiles = [];
  
  for (const file of stagedFiles) {
    const scanResult = shouldScanFile(file);
    
    if (scanResult.block) {
      blockedFiles.push({ file, reason: scanResult.reason });
    } else if (scanResult.scan) {
      const issues = scanFileContent(file);
      totalIssues = totalIssues.concat(issues);
    }
  }
  
  // Report results
  if (blockedFiles.length > 0) {
    console.log(`\n${colors.red}❌ BLOCKED FILES DETECTED:${colors.reset}`);
    blockedFiles.forEach(({ file, reason }) => {
      console.log(`   ${colors.red}✗${colors.reset} ${file}`);
      console.log(`     Reason: ${reason}`);
    });
  }
  
  if (totalIssues.length > 0) {
    console.log(`\n${colors.red}❌ POTENTIAL SECRETS DETECTED:${colors.reset}`);
    totalIssues.forEach(issue => {
      console.log(`   ${colors.red}✗${colors.reset} ${issue.file}:${issue.line}`);
      console.log(`     Pattern: ${issue.pattern}`);
      console.log(`     Content: ${issue.content}`);
    });
  }
  
  if (blockedFiles.length > 0 || totalIssues.length > 0) {
    console.log(`\n${colors.red}❌ Commit blocked: ${blockedFiles.length} blocked files, ${totalIssues.length} potential secrets found${colors.reset}`);
    console.log(`\n${colors.yellow}To bypass (use with caution):${colors.reset}`);
    console.log('   git commit --no-verify');
    console.log(`\n${colors.yellow}To remove sensitive files from staging:${colors.reset}`);
    console.log('   git reset HEAD <file>');
    process.exit(1);
  }
  
  console.log(`${colors.green}✅ No secrets detected in ${stagedFiles.length} files${colors.reset}`);
  process.exit(0);
}

// Run if called directly
if (require.main === module) {
  main();
}

module.exports = { getStagedFiles, shouldScanFile, scanFileContent };