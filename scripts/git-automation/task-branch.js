#!/usr/bin/env node

/**
 * Task-Branch Mapper for Multi-Controller App
 * Creates and manages git branches based on Task Master tasks
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Colors for terminal output
const colors = {
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
  reset: '\x1b[0m'
};

// Parse command line arguments
const args = process.argv.slice(2);
const taskId = args[0];
const command = args[1] || 'create'; // create, switch, list

// Paths
const tasksPath = path.join(__dirname, '../../.taskmaster/tasks/tasks.json');

/**
 * Load tasks from Task Master
 */
function loadTasks() {
  if (!fs.existsSync(tasksPath)) {
    console.error(`${colors.red}❌ Tasks file not found: ${tasksPath}${colors.reset}`);
    console.error(`${colors.yellow}   Make sure Task Master is initialized${colors.reset}`);
    process.exit(1);
  }
  
  try {
    const content = fs.readFileSync(tasksPath, 'utf8');
    return JSON.parse(content);
  } catch (error) {
    console.error(`${colors.red}❌ Failed to load tasks: ${error.message}${colors.reset}`);
    process.exit(1);
  }
}

/**
 * Find a task by ID (supports nested subtasks)
 */
function findTask(tasks, targetId) {
  for (const task of tasks) {
    if (task.id === targetId) {
      return task;
    }
    if (task.subtasks && task.subtasks.length > 0) {
      const found = findTask(task.subtasks, targetId);
      if (found) return found;
    }
  }
  return null;
}

/**
 * Slugify task title for branch name
 */
function slugify(text) {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .substring(0, 50); // Limit length
}

/**
 * Generate branch name from task
 */
function generateBranchName(task) {
  const slug = slugify(task.title);
  return `feature/task-${task.id}-${slug}`;
}

/**
 * Get current git branch
 */
function getCurrentBranch() {
  try {
    return execSync('git symbolic-ref --short HEAD', { encoding: 'utf8' }).trim();
  } catch (error) {
    return null;
  }
}

/**
 * Check if branch exists
 */
function branchExists(branchName) {
  try {
    execSync(`git show-ref --verify --quiet refs/heads/${branchName}`);
    return true;
  } catch (error) {
    return false;
  }
}

/**
 * Create and switch to task branch
 */
function createTaskBranch(task) {
  const branchName = generateBranchName(task);
  const currentBranch = getCurrentBranch();
  
  console.log(`${colors.blue}📌 Task #${task.id}: ${task.title}${colors.reset}`);
  console.log(`${colors.cyan}   Status: ${task.status}${colors.reset}`);
  
  // Check if branch already exists
  if (branchExists(branchName)) {
    console.log(`${colors.yellow}⚠️  Branch already exists: ${branchName}${colors.reset}`);
    
    // Switch to existing branch
    console.log(`${colors.blue}   Switching to existing branch...${colors.reset}`);
    try {
      execSync(`git checkout ${branchName}`, { stdio: 'pipe' });
      console.log(`${colors.green}✅ Switched to branch: ${branchName}${colors.reset}`);
    } catch (error) {
      console.error(`${colors.red}❌ Failed to switch branch: ${error.message}${colors.reset}`);
      process.exit(1);
    }
  } else {
    // Create new branch
    console.log(`${colors.blue}   Creating new branch: ${branchName}${colors.reset}`);
    
    // Check for uncommitted changes
    const status = execSync('git status --porcelain', { encoding: 'utf8' });
    if (status.trim()) {
      console.log(`${colors.yellow}⚠️  You have uncommitted changes:${colors.reset}`);
      console.log(status);
      console.log(`${colors.yellow}   Consider committing or stashing before creating branch${colors.reset}`);
      console.log(`${colors.yellow}   Use 'git stash' to save changes temporarily${colors.reset}`);
      process.exit(1);
    }
    
    try {
      // Create and switch to new branch
      execSync(`git checkout -b ${branchName}`, { stdio: 'pipe' });
      console.log(`${colors.green}✅ Created and switched to branch: ${branchName}${colors.reset}`);
      
      // Update task status to in-progress
      updateTaskStatus(task.id, 'in-progress');
      
    } catch (error) {
      console.error(`${colors.red}❌ Failed to create branch: ${error.message}${colors.reset}`);
      process.exit(1);
    }
  }
  
  // Show next steps
  console.log(`\n${colors.blue}Next steps:${colors.reset}`);
  console.log(`  1. Implement changes for task #${task.id}`);
  console.log(`  2. Commit with: git commit -m "feat: ${task.title} (task ${task.id})"`);
  console.log(`  3. Push branch: git push -u origin ${branchName}`);
  console.log(`  4. Create PR: npm run task:pr ${task.id}`);
}

/**
 * Update task status in tasks.json
 */
function updateTaskStatus(taskId, newStatus) {
  try {
    const data = JSON.parse(fs.readFileSync(tasksPath, 'utf8'));
    const task = findTask(data.tasks, taskId);
    
    if (task) {
      task.status = newStatus;
      fs.writeFileSync(tasksPath, JSON.stringify(data, null, 2));
      console.log(`${colors.green}✅ Updated task status to: ${newStatus}${colors.reset}`);
    }
  } catch (error) {
    console.warn(`${colors.yellow}⚠️  Could not update task status: ${error.message}${colors.reset}`);
  }
}

/**
 * List all task branches
 */
function listTaskBranches() {
  console.log(`${colors.blue}📋 Task Branches:${colors.reset}\n`);
  
  try {
    // Get all branches
    const branches = execSync('git branch -a', { encoding: 'utf8' })
      .split('\n')
      .filter(b => b.includes('task-'));
    
    const currentBranch = getCurrentBranch();
    const data = JSON.parse(fs.readFileSync(tasksPath, 'utf8'));
    
    branches.forEach(branch => {
      const branchName = branch.replace(/^\*?\s+/, '').replace(/^remotes\/origin\//, '');
      const isCurrent = branch.startsWith('*');
      const isRemote = branch.includes('remotes/origin/');
      
      // Extract task ID from branch name
      const match = branchName.match(/task-([0-9]+(?:\.[0-9]+)*)/);
      if (match) {
        const taskId = match[1];
        const task = findTask(data.tasks, taskId);
        
        if (task) {
          const marker = isCurrent ? '→' : ' ';
          const location = isRemote ? '[remote]' : '[local]';
          const statusColor = task.status === 'done' ? colors.green : 
                            task.status === 'in-progress' ? colors.yellow : 
                            colors.cyan;
          
          console.log(`${marker} ${branchName} ${location}`);
          console.log(`   Task #${task.id}: ${task.title}`);
          console.log(`   Status: ${statusColor}${task.status}${colors.reset}\n`);
        }
      }
    });
    
  } catch (error) {
    console.error(`${colors.red}❌ Failed to list branches: ${error.message}${colors.reset}`);
    process.exit(1);
  }
}

/**
 * Main execution
 */
function main() {
  console.log(`${colors.blue}╔══════════════════════════════════════╗${colors.reset}`);
  console.log(`${colors.blue}║      Task-Branch Mapper v1.0.0       ║${colors.reset}`);
  console.log(`${colors.blue}╚══════════════════════════════════════╝${colors.reset}\n`);
  
  // Handle list command
  if (taskId === 'list' || command === 'list') {
    listTaskBranches();
    return;
  }
  
  // Validate task ID
  if (!taskId) {
    console.error(`${colors.red}❌ Usage: node task-branch.js <task-id> [command]${colors.reset}`);
    console.error(`${colors.yellow}   Commands: create (default), switch, list${colors.reset}`);
    console.error(`${colors.yellow}   Example: node task-branch.js 11${colors.reset}`);
    console.error(`${colors.yellow}   Example: node task-branch.js list${colors.reset}`);
    process.exit(1);
  }
  
  // Load tasks
  const data = loadTasks();
  const task = findTask(data.tasks, taskId);
  
  if (!task) {
    console.error(`${colors.red}❌ Task not found: ${taskId}${colors.reset}`);
    console.error(`${colors.yellow}   Available tasks:${colors.reset}`);
    data.tasks.slice(0, 10).forEach(t => {
      console.error(`     ${t.id}: ${t.title}`);
    });
    process.exit(1);
  }
  
  // Execute command
  switch (command) {
    case 'create':
    case 'switch':
      createTaskBranch(task);
      break;
    default:
      console.error(`${colors.red}❌ Unknown command: ${command}${colors.reset}`);
      process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main();
}

module.exports = { loadTasks, findTask, generateBranchName, slugify };