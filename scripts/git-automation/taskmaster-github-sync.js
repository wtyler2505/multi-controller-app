#!/usr/bin/env node

/**
 * Task Master ↔ GitHub Issues Bidirectional Sync
 * 
 * This script synchronizes Task Master tasks with GitHub Issues
 * for team visibility and collaboration.
 */

const fs = require('fs').promises;
const path = require('path');
const { execSync } = require('child_process');

// Configuration paths
const TASKS_FILE = path.join('.taskmaster', 'tasks', 'tasks.json');
const GITHUB_MAP_FILE = path.join('.taskmaster', 'github-map.json');
const SYNC_LOG_DIR = path.join('.taskmaster', 'sync-logs');

// Status mapping
const STATUS_MAP = {
  taskmaster_to_github: {
    'pending': { state: 'open', labels: [] },
    'in-progress': { state: 'open', labels: ['in-progress'] },
    'review': { state: 'open', labels: ['review'] },
    'done': { state: 'closed', labels: [] },
    'blocked': { state: 'open', labels: ['blocked'] },
    'deferred': { state: 'open', labels: ['deferred'] },
    'cancelled': { state: 'closed', labels: [] }
  },
  github_to_taskmaster: {
    'open': 'pending',
    'closed': 'done'
  }
};

class TaskMasterGitHubSync {
  constructor(options = {}) {
    this.dryRun = options.dryRun || false;
    this.verbose = options.verbose || false;
    this.repository = null;
    this.githubMap = null;
    this.tasks = null;
    this.syncStats = {
      created: 0,
      updated: 0,
      errors: 0,
      skipped: 0
    };
  }

  async init() {
    // Load Task Master tasks
    const tasksData = await fs.readFile(TASKS_FILE, 'utf8');
    const tasksJson = JSON.parse(tasksData);
    this.tasks = tasksJson.master?.tasks || [];

    // Load GitHub mapping
    const mapData = await fs.readFile(GITHUB_MAP_FILE, 'utf8');
    this.githubMap = JSON.parse(mapData);
    this.repository = this.githubMap.config?.repository;

    if (!this.repository) {
      throw new Error('Repository not configured in github-map.json');
    }

    // Create sync log directory if it doesn't exist
    await fs.mkdir(SYNC_LOG_DIR, { recursive: true });

    this.log(`Initialized sync for repository: ${this.repository}`);
    this.log(`Found ${this.tasks.length} tasks in Task Master`);
  }

  log(message, level = 'info') {
    if (this.verbose || level === 'error') {
      const timestamp = new Date().toISOString();
      console.log(`[${timestamp}] [${level.toUpperCase()}] ${message}`);
    }
  }

  async syncTaskToGitHub(task) {
    const issueNumber = this.githubMap.tasks[task.id];
    
    if (issueNumber) {
      // Update existing issue
      await this.updateGitHubIssue(task, issueNumber);
    } else if (task.status !== 'done' && task.status !== 'cancelled') {
      // Create new issue only for active tasks
      await this.createGitHubIssue(task);
    } else {
      this.log(`Skipping completed/cancelled task ${task.id}: ${task.title}`);
      this.syncStats.skipped++;
    }
  }

  async createGitHubIssue(task) {
    const title = `[Task ${task.id}] ${task.title}`;
    const body = this.formatIssueBody(task);
    const labels = this.getLabelsForTask(task);

    const command = [
      'gh', 'issue', 'create',
      '--title', JSON.stringify(title),
      '--body', JSON.stringify(body)
    ];

    if (labels.length > 0) {
      command.push('--label', labels.join(','));
    }

    try {
      if (this.dryRun) {
        this.log(`[DRY RUN] Would create issue: ${title}`, 'info');
        return;
      }

      const result = execSync(command.join(' '), { encoding: 'utf8' });
      const issueUrl = result.trim();
      const issueNumber = parseInt(issueUrl.split('/').pop());

      // Update mapping
      this.githubMap.tasks[task.id] = issueNumber;
      this.log(`Created issue #${issueNumber} for task ${task.id}`, 'success');
      this.syncStats.created++;
    } catch (error) {
      this.log(`Failed to create issue for task ${task.id}: ${error.message}`, 'error');
      this.syncStats.errors++;
    }
  }

  async updateGitHubIssue(task, issueNumber) {
    const githubStatus = STATUS_MAP.taskmaster_to_github[task.status];
    const labels = this.getLabelsForTask(task);

    try {
      // Get current issue state
      const currentIssue = JSON.parse(
        execSync(`gh issue view ${issueNumber} --json state,labels`, { encoding: 'utf8' })
      );

      // Check if state needs updating
      if (githubStatus.state !== currentIssue.state) {
        const action = githubStatus.state === 'closed' ? 'close' : 'reopen';
        
        if (this.dryRun) {
          this.log(`[DRY RUN] Would ${action} issue #${issueNumber}`, 'info');
        } else {
          execSync(`gh issue ${action} ${issueNumber}`, { encoding: 'utf8' });
          this.log(`Updated issue #${issueNumber} state to ${githubStatus.state}`, 'success');
        }
      }

      // Update labels if needed
      const currentLabelNames = currentIssue.labels.map(l => l.name);
      const labelsDiff = labels.filter(l => !currentLabelNames.includes(l));
      
      if (labelsDiff.length > 0) {
        if (this.dryRun) {
          this.log(`[DRY RUN] Would add labels to issue #${issueNumber}: ${labelsDiff.join(', ')}`, 'info');
        } else {
          execSync(`gh issue edit ${issueNumber} --add-label ${labelsDiff.join(',')}`, { encoding: 'utf8' });
          this.log(`Added labels to issue #${issueNumber}: ${labelsDiff.join(', ')}`, 'success');
        }
      }

      this.syncStats.updated++;
    } catch (error) {
      this.log(`Failed to update issue #${issueNumber}: ${error.message}`, 'error');
      this.syncStats.errors++;
    }
  }

  formatIssueBody(task) {
    let body = `## Description\n${task.description}\n\n`;
    
    if (task.details) {
      body += `## Details\n${task.details}\n\n`;
    }

    if (task.subtasks && task.subtasks.length > 0) {
      body += `## Subtasks\n`;
      task.subtasks.forEach(subtask => {
        const checked = subtask.status === 'done' ? 'x' : ' ';
        body += `- [${checked}] ${subtask.title}\n`;
      });
      body += '\n';
    }

    if (task.dependencies && task.dependencies.length > 0) {
      body += `## Dependencies\n`;
      task.dependencies.forEach(dep => {
        const depIssue = this.githubMap.tasks[dep];
        if (depIssue) {
          body += `- Depends on #${depIssue} (Task ${dep})\n`;
        } else {
          body += `- Depends on Task ${dep}\n`;
        }
      });
      body += '\n';
    }

    if (task.testStrategy) {
      body += `## Test Strategy\n${task.testStrategy}\n\n`;
    }

    body += `## Metadata\n`;
    body += `- Task Master ID: ${task.id}\n`;
    body += `- Priority: ${task.priority || 'medium'}\n`;
    body += `- Status: ${task.status}\n`;
    
    if (task.complexityScore) {
      body += `- Complexity Score: ${task.complexityScore}\n`;
    }

    return body;
  }

  getLabelsForTask(task) {
    const labels = ['task:multi-controller'];
    
    // Add priority label
    const priorityLabel = this.githubMap.config.priorityLabels[task.priority];
    if (priorityLabel) {
      labels.push(priorityLabel);
    }

    // Add status labels
    const statusLabels = this.githubMap.config.statusLabels[task.status] || [];
    labels.push(...statusLabels);

    // Add epic label if task has subtasks
    if (task.subtasks && task.subtasks.length > 0) {
      labels.push('epic');
    }

    return labels;
  }

  async saveGitHubMap() {
    this.githubMap.lastSync = new Date().toISOString();
    await fs.writeFile(GITHUB_MAP_FILE, JSON.stringify(this.githubMap, null, 2));
    this.log('Saved GitHub mapping file');
  }

  async generateSyncReport() {
    const report = {
      timestamp: new Date().toISOString(),
      repository: this.repository,
      stats: this.syncStats,
      mappings: this.githubMap.tasks
    };

    const reportFile = path.join(
      SYNC_LOG_DIR,
      `sync-${new Date().toISOString().split('T')[0]}.json`
    );

    await fs.writeFile(reportFile, JSON.stringify(report, null, 2));
    
    console.log('\n=== Sync Report ===');
    console.log(`Created: ${this.syncStats.created} issues`);
    console.log(`Updated: ${this.syncStats.updated} issues`);
    console.log(`Skipped: ${this.syncStats.skipped} tasks`);
    console.log(`Errors: ${this.syncStats.errors}`);
    console.log(`Report saved to: ${reportFile}`);
  }

  async run() {
    try {
      await this.init();

      // Sync pending tasks only (27-36)
      const pendingTasks = this.tasks.filter(t => 
        t.id >= 27 && t.id <= 36 && t.status === 'pending'
      );

      console.log(`\nSyncing ${pendingTasks.length} pending tasks to GitHub...`);

      for (const task of pendingTasks) {
        await this.syncTaskToGitHub(task);
      }

      if (!this.dryRun) {
        await this.saveGitHubMap();
      }

      await this.generateSyncReport();
    } catch (error) {
      this.log(`Sync failed: ${error.message}`, 'error');
      process.exit(1);
    }
  }
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);
  const options = {
    dryRun: args.includes('--dry-run'),
    verbose: args.includes('--verbose') || args.includes('-v')
  };

  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
Task Master ↔ GitHub Issues Sync

Usage: node taskmaster-github-sync.js [options]

Options:
  --dry-run    Preview changes without applying them
  --verbose    Show detailed output
  --help       Show this help message

Examples:
  node taskmaster-github-sync.js              # Run sync
  node taskmaster-github-sync.js --dry-run    # Preview sync
  node taskmaster-github-sync.js --verbose    # Detailed output
    `);
    process.exit(0);
  }

  const sync = new TaskMasterGitHubSync(options);
  sync.run();
}

module.exports = TaskMasterGitHubSync;