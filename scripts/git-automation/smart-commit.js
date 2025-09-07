#!/usr/bin/env node

/**
 * Smart Commit Message Generator for Multi-Controller App
 * Generates conventional commit messages with task context
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const readline = require('readline');

// Colors for terminal output
const colors = {
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
  magenta: '\x1b[35m',
  reset: '\x1b[0m'
};

// Conventional commit types
const commitTypes = {
  feat: 'A new feature',
  fix: 'A bug fix',
  docs: 'Documentation only changes',
  style: 'Changes that do not affect the meaning of the code',
  refactor: 'A code change that neither fixes a bug nor adds a feature',
  perf: 'A code change that improves performance',
  test: 'Adding missing tests or correcting existing tests',
  build: 'Changes that affect the build system or external dependencies',
  ci: 'Changes to CI configuration files and scripts',
  chore: 'Other changes that don\'t modify src or test files',
  revert: 'Reverts a previous commit'
};

// Paths
const tasksPath = path.join(__dirname, '../../.taskmaster/tasks/tasks.json');

/**
 * Get staged files
 */
function getStagedFiles() {
  try {
    const output = execSync('git diff --cached --name-only', { encoding: 'utf8' });
    return output.split('\n').filter(file => file.length > 0);
  } catch (error) {
    return [];
  }
}

/**
 * Get current branch name
 */
function getCurrentBranch() {
  try {
    return execSync('git symbolic-ref --short HEAD', { encoding: 'utf8' }).trim();
  } catch (error) {
    return null;
  }
}

/**
 * Extract task ID from branch name
 */
function getTaskFromBranch() {
  const branch = getCurrentBranch();
  if (!branch) return null;
  
  const match = branch.match(/task-([0-9]+(?:\.[0-9]+)*)/);
  return match ? match[1] : null;
}

/**
 * Load task information
 */
function loadTask(taskId) {
  if (!fs.existsSync(tasksPath)) return null;
  
  try {
    const data = JSON.parse(fs.readFileSync(tasksPath, 'utf8'));
    return findTask(data.tasks, taskId);
  } catch (error) {
    return null;
  }
}

/**
 * Find task recursively
 */
function findTask(tasks, targetId) {
  for (const task of tasks) {
    if (task.id === targetId) return task;
    if (task.subtasks?.length > 0) {
      const found = findTask(task.subtasks, targetId);
      if (found) return found;
    }
  }
  return null;
}

/**
 * Analyze staged changes to suggest commit type
 */
function analyzeChanges(files) {
  const analysis = {
    hasTests: false,
    hasDocs: false,
    hasConfig: false,
    hasSource: false,
    hasStyles: false,
    hasPerf: false,
    mainScope: null
  };
  
  const scopeCounts = {};
  
  files.forEach(file => {
    // Detect file types
    if (file.includes('test') || file.includes('spec')) {
      analysis.hasTests = true;
    }
    if (file.match(/\.(md|txt|rst)$/i) || file.includes('docs/')) {
      analysis.hasDocs = true;
    }
    if (file.match(/\.(json|yml|yaml|toml|ini|config)$/i) || file.includes('config')) {
      analysis.hasConfig = true;
    }
    if (file.match(/\.(ts|tsx|js|jsx|cs|cpp|c|h)$/i)) {
      analysis.hasSource = true;
    }
    if (file.match(/\.(css|scss|sass|less)$/i)) {
      analysis.hasStyles = true;
    }
    if (file.includes('performance') || file.includes('perf') || file.includes('benchmark')) {
      analysis.hasPerf = true;
    }
    
    // Detect scope from path
    const parts = file.split('/');
    if (parts.length > 1) {
      const scope = parts[0] === 'src' ? parts[1] : parts[0];
      scopeCounts[scope] = (scopeCounts[scope] || 0) + 1;
    }
  });
  
  // Determine main scope
  if (Object.keys(scopeCounts).length > 0) {
    analysis.mainScope = Object.entries(scopeCounts)
      .sort((a, b) => b[1] - a[1])[0][0];
  }
  
  // Suggest commit type
  if (analysis.hasTests && !analysis.hasSource) {
    analysis.suggestedType = 'test';
  } else if (analysis.hasDocs && !analysis.hasSource) {
    analysis.suggestedType = 'docs';
  } else if (analysis.hasStyles && !analysis.hasSource) {
    analysis.suggestedType = 'style';
  } else if (analysis.hasPerf) {
    analysis.suggestedType = 'perf';
  } else if (analysis.hasConfig && !analysis.hasSource) {
    analysis.suggestedType = 'chore';
  } else {
    // Check commit history for patterns
    const recentCommits = getRecentCommits();
    if (recentCommits.some(c => c.includes('fix'))) {
      analysis.suggestedType = 'fix';
    } else {
      analysis.suggestedType = 'feat';
    }
  }
  
  return analysis;
}

/**
 * Get recent commit messages
 */
function getRecentCommits() {
  try {
    const output = execSync('git log --oneline -10', { encoding: 'utf8' });
    return output.split('\n').filter(line => line.length > 0);
  } catch (error) {
    return [];
  }
}

/**
 * Create readline interface for user input
 */
function createInterface() {
  return readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });
}

/**
 * Prompt user for input
 */
function prompt(rl, question, defaultValue = '') {
  return new Promise(resolve => {
    const q = defaultValue ? `${question} [${defaultValue}]: ` : `${question}: `;
    rl.question(q, answer => {
      resolve(answer || defaultValue);
    });
  });
}

/**
 * Generate commit message interactively
 */
async function generateCommitMessage() {
  console.log(`${colors.blue}╔══════════════════════════════════════╗${colors.reset}`);
  console.log(`${colors.blue}║   Smart Commit Message Generator     ║${colors.reset}`);
  console.log(`${colors.blue}╚══════════════════════════════════════╝${colors.reset}\n`);
  
  // Get staged files
  const stagedFiles = getStagedFiles();
  if (stagedFiles.length === 0) {
    console.log(`${colors.red}❌ No staged files found${colors.reset}`);
    console.log(`${colors.yellow}   Stage files with: git add <files>${colors.reset}`);
    process.exit(1);
  }
  
  console.log(`${colors.cyan}📁 Staged files (${stagedFiles.length}):${colors.reset}`);
  stagedFiles.slice(0, 10).forEach(file => {
    console.log(`   • ${file}`);
  });
  if (stagedFiles.length > 10) {
    console.log(`   ... and ${stagedFiles.length - 10} more`);
  }
  console.log();
  
  // Analyze changes
  const analysis = analyzeChanges(stagedFiles);
  
  // Get task context
  const taskId = getTaskFromBranch();
  const task = taskId ? loadTask(taskId) : null;
  
  if (task) {
    console.log(`${colors.green}📌 Task #${task.id}: ${task.title}${colors.reset}`);
    console.log(`   Status: ${task.status}\n`);
  }
  
  // Create readline interface
  const rl = createInterface();
  
  try {
    // Select commit type
    console.log(`${colors.blue}Commit types:${colors.reset}`);
    Object.entries(commitTypes).forEach(([type, desc]) => {
      const marker = type === analysis.suggestedType ? '→' : ' ';
      console.log(`${marker} ${colors.cyan}${type}${colors.reset}: ${desc}`);
    });
    
    const type = await prompt(rl, '\nType', analysis.suggestedType);
    if (!commitTypes[type]) {
      console.log(`${colors.red}❌ Invalid type: ${type}${colors.reset}`);
      process.exit(1);
    }
    
    // Enter scope
    const scope = await prompt(rl, 'Scope (optional)', analysis.mainScope || '');
    
    // Enter description
    let defaultDesc = '';
    if (task) {
      // Use task title as default description
      defaultDesc = task.title.toLowerCase()
        .replace(/^(implement|create|add|fix|update)\s+/i, '');
    }
    
    const description = await prompt(rl, 'Description', defaultDesc);
    if (!description) {
      console.log(`${colors.red}❌ Description is required${colors.reset}`);
      process.exit(1);
    }
    
    // Enter body (optional)
    console.log('\nBody (optional, press Enter twice to finish):');
    const bodyLines = [];
    let line = await prompt(rl, '');
    while (line !== '') {
      bodyLines.push(line);
      line = await prompt(rl, '');
    }
    const body = bodyLines.join('\n');
    
    // Check for breaking changes
    const breaking = await prompt(rl, 'Breaking change? (y/N)', 'N');
    const isBreaking = breaking.toLowerCase() === 'y';
    
    let breakingDesc = '';
    if (isBreaking) {
      breakingDesc = await prompt(rl, 'Describe breaking change');
    }
    
    // Construct commit message
    let commitMsg = type;
    if (scope) commitMsg += `(${scope})`;
    if (isBreaking) commitMsg += '!';
    commitMsg += `: ${description}`;
    
    if (task) {
      commitMsg += ` (task ${task.id})`;
    }
    
    if (body || breakingDesc) {
      commitMsg += '\n\n';
      if (body) commitMsg += body;
      if (breakingDesc) {
        if (body) commitMsg += '\n\n';
        commitMsg += `BREAKING CHANGE: ${breakingDesc}`;
      }
    }
    
    // Show preview
    console.log(`\n${colors.blue}═══ Commit Message Preview ═══${colors.reset}`);
    console.log(commitMsg);
    console.log(`${colors.blue}═══════════════════════════════${colors.reset}\n`);
    
    // Confirm and commit
    const confirm = await prompt(rl, 'Commit with this message? (Y/n)', 'Y');
    if (confirm.toLowerCase() === 'n') {
      console.log(`${colors.yellow}Commit cancelled${colors.reset}`);
      process.exit(0);
    }
    
    // Execute commit
    try {
      // Write commit message to temp file to handle multiline properly
      const tempFile = path.join(process.cwd(), '.git', 'COMMIT_EDITMSG_TEMP');
      fs.writeFileSync(tempFile, commitMsg);
      
      execSync(`git commit -F "${tempFile}"`, { stdio: 'inherit' });
      
      // Clean up temp file
      fs.unlinkSync(tempFile);
      
      console.log(`\n${colors.green}✅ Commit successful!${colors.reset}`);
      
      // Show next steps
      console.log(`\n${colors.blue}Next steps:${colors.reset}`);
      console.log(`  • Push changes: git push`);
      if (task) {
        console.log(`  • Create PR: npm run task:pr ${task.id}`);
        console.log(`  • Update task status: npm run task:complete ${task.id}`);
      }
      
    } catch (error) {
      console.error(`${colors.red}❌ Commit failed: ${error.message}${colors.reset}`);
      process.exit(1);
    }
    
  } finally {
    rl.close();
  }
}

// Main execution
if (require.main === module) {
  generateCommitMessage().catch(error => {
    console.error(`${colors.red}Error: ${error.message}${colors.reset}`);
    process.exit(1);
  });
}

module.exports = { analyzeChanges, getTaskFromBranch };