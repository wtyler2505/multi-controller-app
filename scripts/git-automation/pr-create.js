#!/usr/bin/env node

/**
 * Pull Request Automation for Multi-Controller App
 * Creates GitHub PRs with task context and validation checklist
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

// Paths
const tasksPath = path.join(__dirname, '../../.taskmaster/tasks/tasks.json');

// Parse command line arguments
const args = process.argv.slice(2);
const taskId = args[0];

/**
 * Check if gh CLI is installed
 */
function checkGitHubCLI() {
  try {
    execSync('gh --version', { stdio: 'ignore' });
    return true;
  } catch (error) {
    console.error(`${colors.red}❌ GitHub CLI not installed${colors.reset}`);
    console.error(`${colors.yellow}   Install from: https://cli.github.com${colors.reset}`);
    console.error(`${colors.yellow}   Or use: winget install GitHub.cli${colors.reset}`);
    return false;
  }
}

/**
 * Check if authenticated with GitHub
 */
function checkAuthentication() {
  try {
    execSync('gh auth status', { stdio: 'ignore' });
    return true;
  } catch (error) {
    console.error(`${colors.red}❌ Not authenticated with GitHub${colors.reset}`);
    console.error(`${colors.yellow}   Run: gh auth login${colors.reset}`);
    return false;
  }
}

/**
 * Get current branch
 */
function getCurrentBranch() {
  try {
    return execSync('git symbolic-ref --short HEAD', { encoding: 'utf8' }).trim();
  } catch (error) {
    return null;
  }
}

/**
 * Get task from branch or ID
 */
function getTask(taskIdOrBranch) {
  if (!fs.existsSync(tasksPath)) return null;
  
  try {
    const data = JSON.parse(fs.readFileSync(tasksPath, 'utf8'));
    
    // Try direct task ID first
    let task = findTask(data.tasks, taskIdOrBranch);
    if (task) return task;
    
    // Try extracting from branch name
    const match = taskIdOrBranch.match(/task-([0-9]+(?:\.[0-9]+)*)/);
    if (match) {
      return findTask(data.tasks, match[1]);
    }
    
    return null;
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
 * Get commit history for branch
 */
function getCommitHistory(baseBranch = 'main') {
  try {
    const commits = execSync(`git log ${baseBranch}..HEAD --oneline`, { encoding: 'utf8' })
      .split('\n')
      .filter(line => line.length > 0);
    return commits;
  } catch (error) {
    return [];
  }
}

/**
 * Get changed files
 */
function getChangedFiles(baseBranch = 'main') {
  try {
    const files = execSync(`git diff ${baseBranch}...HEAD --name-only`, { encoding: 'utf8' })
      .split('\n')
      .filter(file => file.length > 0);
    return files;
  } catch (error) {
    return [];
  }
}

/**
 * Generate PR body
 */
function generatePRBody(task, commits, changedFiles) {
  let body = '';
  
  // Task context
  if (task) {
    body += `## 📌 Task Context\n\n`;
    body += `**Task #${task.id}**: ${task.title}\n\n`;
    
    if (task.description) {
      body += `**Description**: ${task.description}\n\n`;
    }
    
    if (task.details) {
      body += `### Implementation Details\n\n`;
      body += `${task.details}\n\n`;
    }
  }
  
  // Changes summary
  body += `## 📝 Changes\n\n`;
  
  // Group files by type
  const groups = {
    features: [],
    fixes: [],
    tests: [],
    docs: [],
    config: [],
    other: []
  };
  
  changedFiles.forEach(file => {
    if (file.includes('test') || file.includes('spec')) {
      groups.tests.push(file);
    } else if (file.match(/\.(md|txt|rst)$/i) || file.includes('docs/')) {
      groups.docs.push(file);
    } else if (file.match(/\.(json|yml|yaml|toml|ini)$/i)) {
      groups.config.push(file);
    } else if (commits.some(c => c.toLowerCase().includes('fix'))) {
      groups.fixes.push(file);
    } else if (commits.some(c => c.toLowerCase().includes('feat'))) {
      groups.features.push(file);
    } else {
      groups.other.push(file);
    }
  });
  
  // List changes by category
  if (groups.features.length > 0) {
    body += `### ✨ Features\n`;
    groups.features.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  if (groups.fixes.length > 0) {
    body += `### 🐛 Fixes\n`;
    groups.fixes.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  if (groups.tests.length > 0) {
    body += `### 🧪 Tests\n`;
    groups.tests.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  if (groups.docs.length > 0) {
    body += `### 📚 Documentation\n`;
    groups.docs.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  if (groups.config.length > 0) {
    body += `### ⚙️ Configuration\n`;
    groups.config.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  if (groups.other.length > 0) {
    body += `### 📦 Other\n`;
    groups.other.forEach(file => body += `- ${file}\n`);
    body += '\n';
  }
  
  // Commit history
  if (commits.length > 0) {
    body += `## 📜 Commit History\n\n`;
    commits.forEach(commit => body += `- ${commit}\n`);
    body += '\n';
  }
  
  // Validation checklist
  body += `## ✅ Validation Checklist\n\n`;
  
  if (task) {
    body += `### Task Completion\n`;
    body += `- [ ] Task #${task.id} requirements met\n`;
    if (task.subtasks?.length > 0) {
      const completedSubtasks = task.subtasks.filter(st => st.status === 'done').length;
      body += `- [ ] All subtasks completed (${completedSubtasks}/${task.subtasks.length})\n`;
    }
    if (task.dependencies?.length > 0) {
      body += `- [ ] Dependencies resolved: ${task.dependencies.join(', ')}\n`;
    }
    body += '\n';
  }
  
  body += `### Code Quality\n`;
  body += `- [ ] No secrets or credentials exposed\n`;
  body += `- [ ] Performance budgets met (<2s startup, <150MB RAM)\n`;
  body += `- [ ] All tests passing\n`;
  body += `- [ ] Code follows project conventions\n`;
  body += `- [ ] No console.log or Debug.WriteLine statements\n`;
  body += `\n`;
  
  body += `### Testing\n`;
  body += `- [ ] Manual testing completed\n`;
  body += `- [ ] Unit tests added/updated\n`;
  body += `- [ ] Integration tests passing\n`;
  
  if (task?.testStrategy) {
    body += `\n**Test Strategy**: ${task.testStrategy}\n`;
  }
  
  body += '\n';
  
  // Related issues/PRs
  if (task?.dependencies?.length > 0) {
    body += `## 🔗 Related\n\n`;
    body += `**Depends on tasks**: ${task.dependencies.join(', ')}\n\n`;
  }
  
  // Footer
  body += `---\n`;
  body += `*Generated by Multi-Controller App PR Automation*\n`;
  body += `*Task Master Integration v1.0.0*`;
  
  return body;
}

/**
 * Create interface for user input
 */
function createInterface() {
  return readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });
}

/**
 * Prompt user
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
 * Main PR creation
 */
async function createPR() {
  console.log(`${colors.blue}╔══════════════════════════════════════╗${colors.reset}`);
  console.log(`${colors.blue}║      Pull Request Automation         ║${colors.reset}`);
  console.log(`${colors.blue}╚══════════════════════════════════════╝${colors.reset}\n`);
  
  // Check prerequisites
  if (!checkGitHubCLI() || !checkAuthentication()) {
    process.exit(1);
  }
  
  // Get current branch
  const currentBranch = getCurrentBranch();
  if (!currentBranch) {
    console.error(`${colors.red}❌ Not in a git repository${colors.reset}`);
    process.exit(1);
  }
  
  if (currentBranch === 'main' || currentBranch === 'master') {
    console.error(`${colors.red}❌ Cannot create PR from main branch${colors.reset}`);
    console.error(`${colors.yellow}   Create a feature branch first${colors.reset}`);
    process.exit(1);
  }
  
  console.log(`${colors.cyan}📌 Current branch: ${currentBranch}${colors.reset}\n`);
  
  // Get task context
  const task = getTask(taskId || currentBranch);
  if (task) {
    console.log(`${colors.green}✅ Task found: #${task.id} - ${task.title}${colors.reset}\n`);
  } else if (taskId) {
    console.log(`${colors.yellow}⚠️  Task not found: ${taskId}${colors.reset}\n`);
  }
  
  // Check for unpushed commits
  try {
    const unpushed = execSync(`git log origin/${currentBranch}..HEAD --oneline 2>/dev/null`, { encoding: 'utf8' });
    if (unpushed.trim()) {
      console.log(`${colors.yellow}⚠️  You have unpushed commits:${colors.reset}`);
      console.log(unpushed);
      console.log(`${colors.blue}Pushing branch...${colors.reset}`);
      execSync(`git push -u origin ${currentBranch}`, { stdio: 'inherit' });
      console.log(`${colors.green}✅ Branch pushed${colors.reset}\n`);
    }
  } catch (error) {
    // Branch doesn't exist on remote yet
    console.log(`${colors.blue}Pushing new branch...${colors.reset}`);
    execSync(`git push -u origin ${currentBranch}`, { stdio: 'inherit' });
    console.log(`${colors.green}✅ Branch pushed${colors.reset}\n`);
  }
  
  // Get commit history and changed files
  const commits = getCommitHistory();
  const changedFiles = getChangedFiles();
  
  console.log(`${colors.cyan}📊 Summary:${colors.reset}`);
  console.log(`   • ${commits.length} commits`);
  console.log(`   • ${changedFiles.length} files changed\n`);
  
  // Create readline interface
  const rl = createInterface();
  
  try {
    // Get PR title
    let defaultTitle = task ? `Task #${task.id}: ${task.title}` : '';
    if (!defaultTitle && commits.length > 0) {
      // Use first commit message as default
      defaultTitle = commits[0].replace(/^[a-f0-9]+ /, '');
    }
    
    const title = await prompt(rl, 'PR Title', defaultTitle);
    if (!title) {
      console.error(`${colors.red}❌ Title is required${colors.reset}`);
      process.exit(1);
    }
    
    // Get base branch
    const baseBranch = await prompt(rl, 'Base branch', 'main');
    
    // Generate PR body
    const body = generatePRBody(task, commits, changedFiles);
    
    // Show preview
    console.log(`\n${colors.blue}═══ PR Preview ═══${colors.reset}`);
    console.log(`${colors.cyan}Title:${colors.reset} ${title}`);
    console.log(`${colors.cyan}Base:${colors.reset} ${baseBranch} ← ${currentBranch}`);
    console.log(`${colors.cyan}Body:${colors.reset}\n${body.substring(0, 500)}...`);
    console.log(`${colors.blue}════════════════${colors.reset}\n`);
    
    // Confirm
    const confirm = await prompt(rl, 'Create PR? (Y/n)', 'Y');
    if (confirm.toLowerCase() === 'n') {
      console.log(`${colors.yellow}PR creation cancelled${colors.reset}`);
      process.exit(0);
    }
    
    // Create PR
    console.log(`\n${colors.blue}Creating pull request...${colors.reset}`);
    
    // Write body to temp file
    const tempFile = path.join(process.cwd(), '.pr-body-temp.md');
    fs.writeFileSync(tempFile, body);
    
    try {
      // Create PR using gh CLI
      const output = execSync(
        `gh pr create --title "${title}" --base "${baseBranch}" --body-file "${tempFile}"`,
        { encoding: 'utf8' }
      );
      
      console.log(`\n${colors.green}✅ Pull request created successfully!${colors.reset}`);
      console.log(output);
      
      // Update task status if applicable
      if (task && task.status === 'in-progress') {
        updateTaskStatus(task.id, 'review');
        console.log(`${colors.green}✅ Task #${task.id} status updated to: review${colors.reset}`);
      }
      
      // Show next steps
      console.log(`\n${colors.blue}Next steps:${colors.reset}`);
      console.log(`  • Review PR checklist`);
      console.log(`  • Request reviewers: gh pr edit --add-reviewer <username>`);
      console.log(`  • Add labels: gh pr edit --add-label <label>`);
      if (task) {
        console.log(`  • After merge, complete task: npm run task:complete ${task.id}`);
      }
      
    } catch (error) {
      console.error(`${colors.red}❌ Failed to create PR: ${error.message}${colors.reset}`);
      process.exit(1);
    } finally {
      // Clean up temp file
      if (fs.existsSync(tempFile)) {
        fs.unlinkSync(tempFile);
      }
    }
    
  } finally {
    rl.close();
  }
}

/**
 * Update task status
 */
function updateTaskStatus(taskId, newStatus) {
  try {
    const data = JSON.parse(fs.readFileSync(tasksPath, 'utf8'));
    const task = findTask(data.tasks, taskId);
    
    if (task) {
      task.status = newStatus;
      fs.writeFileSync(tasksPath, JSON.stringify(data, null, 2));
    }
  } catch (error) {
    console.warn(`${colors.yellow}⚠️  Could not update task status: ${error.message}${colors.reset}`);
  }
}

// Main execution
if (require.main === module) {
  createPR().catch(error => {
    console.error(`${colors.red}Error: ${error.message}${colors.reset}`);
    process.exit(1);
  });
}

module.exports = { generatePRBody, getTask };