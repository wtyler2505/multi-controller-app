# Memory MCP Knowledge Graph - Advanced Usage Guide

## Overview

The Memory MCP server provides a powerful knowledge graph system for persistent, searchable memory across Claude Code sessions. This guide presents systematic, creative, and intelligent strategies for maximizing its utility in project development.

## Core Concepts

### Knowledge Graph Structure

```
Entities (Nodes) → Relations (Edges) → Observations (Properties)
```

- **Entities**: Core concepts, components, decisions, conventions
- **Relations**: Connections between entities (depends_on, implements, replaces, etc.)
- **Observations**: Detailed facts, notes, and metadata about entities

## Strategic Memory Categories

### 1. Project Architecture Memory

#### Entity Types

- `Architecture.Component` - System components
- `Architecture.Interface` - Key interfaces
- `Architecture.Pattern` - Design patterns
- `Architecture.Decision` - Architectural decisions

#### Example Workflow

```javascript
// Store core architecture decision
mcp__memory__create_entities({
  entities: [
    {
      name: 'TechnologyStack',
      entityType: 'Architecture.Decision',
      observations: [
        'Evaluated C# Native AOT vs Rust on 2024-01-23',
        'Selected based on: startup < 2s, RAM < 150MB',
        'Decision pending prototype benchmarks',
        'Location: docs/decisions/decision-log.md',
      ],
    },
  ],
});

// Create relationships
mcp__memory__create_relations({
  relations: [
    {
      from: 'TechnologyStack',
      to: 'PerformanceBudgets',
      relationType: 'constrains',
    },
  ],
});
```

### 2. Development Conventions

#### Entity Types

- `Convention.Naming` - Naming conventions
- `Convention.Structure` - File/folder structures
- `Convention.Process` - Development processes
- `Convention.Style` - Code style rules

#### Intelligent Storage Pattern

```javascript
// Store naming convention with context
mcp__memory__create_entities({
  entities: [
    {
      name: 'DriverNamingConvention',
      entityType: 'Convention.Naming',
      observations: [
        'Format: /drivers/<device-name>/',
        'Manifest: manifest.json in each driver directory',
        'Interface: Must implement IDeviceDriver',
        'Example: /drivers/arduino-uno/',
        'Established: Task 5.2 implementation',
      ],
    },
  ],
});
```

### 3. Performance Tracking

#### Entity Types

- `Performance.Benchmark` - Benchmark results
- `Performance.Budget` - Performance constraints
- `Performance.Optimization` - Optimization attempts

#### Systematic Tracking

```javascript
// Record benchmark results
mcp__memory__create_entities({
  entities: [
    {
      name: 'CSharpPrototypeBenchmark',
      entityType: 'Performance.Benchmark',
      observations: [
        'Date: 2024-01-23',
        'Startup: 1.8s (PASS < 2s)',
        'Idle RAM: 142MB (PASS < 150MB)',
        'Idle CPU: 1.2% (PASS < 2%)',
        'Build: dotnet publish -c Release -r win-x64',
        'AOT Size: 18MB single exe',
      ],
    },
  ],
});

// Link to decision
mcp__memory__create_relations({
  relations: [
    {
      from: 'CSharpPrototypeBenchmark',
      to: 'TechnologyStack',
      relationType: 'influences',
    },
  ],
});
```

### 4. API & Interface Evolution

#### Entity Types

- `API.Endpoint` - API endpoints
- `API.Contract` - Interface contracts
- `API.Version` - Version history
- `API.Breaking` - Breaking changes

#### Version Tracking Strategy

```javascript
// Track interface evolution
mcp__memory__create_entities({
  entities: [
    {
      name: 'IDeviceDriver_v1',
      entityType: 'API.Contract',
      observations: [
        'Version: 1.0',
        'Methods: ProbeAsync, OpenAsync',
        'Properties: Name, SupportedTransports',
        'Introduced: Milestone 2, Task 5',
        'Status: Current',
      ],
    },
  ],
});

// Record breaking changes
mcp__memory__add_observations({
  observations: [
    {
      entityName: 'IDeviceDriver_v1',
      contents: [
        'BREAKING: Added CancelAsync method in v1.1',
        'Migration: Implement empty method for compatibility',
      ],
    },
  ],
});
```

### 5. Bug Patterns & Solutions

#### Entity Types

- `Bug.Pattern` - Recurring issues
- `Bug.Solution` - Proven fixes
- `Bug.Workaround` - Temporary solutions

#### Knowledge Accumulation

```javascript
// Document bug pattern
mcp__memory__create_entities({
  entities: [
    {
      name: 'SerialPortAccessDenied',
      entityType: 'Bug.Pattern',
      observations: [
        'Error: Access denied to COM port',
        'Cause: Port already in use or insufficient permissions',
        'Frequency: Common on Windows 11',
        'Detection: System.UnauthorizedAccessException',
      ],
    },
  ],
});

// Link solution
mcp__memory__create_entities({
  entities: [
    {
      name: 'SerialPortAccessDeniedFix',
      entityType: 'Bug.Solution',
      observations: [
        'Solution 1: Release port handles properly in Dispose()',
        'Solution 2: Add retry with exponential backoff',
        'Solution 3: Check port availability before opening',
        'Code: transports/SerialTransport.cs:OpenAsync',
      ],
    },
  ],
});

mcp__memory__create_relations({
  relations: [
    {
      from: 'SerialPortAccessDenied',
      to: 'SerialPortAccessDeniedFix',
      relationType: 'solved_by',
    },
  ],
});
```

### 6. Testing Intelligence

#### Entity Types

- `Test.Strategy` - Test approaches
- `Test.Coverage` - Coverage areas
- `Test.Flaky` - Unstable tests
- `Test.Performance` - Performance tests

#### Test Knowledge Management

```javascript
// Store test strategies
mcp__memory__create_entities({
  entities: [
    {
      name: 'TransportLayerTestStrategy',
      entityType: 'Test.Strategy',
      observations: [
        'Unit Tests: Mock ITransport interface',
        'Integration: Use loopback serial ports',
        'Soak Test: 8-hour continuous operation',
        'Performance: Measure latency percentiles',
        'Coverage Target: > 80%',
      ],
    },
  ],
});
```

### 7. Dependencies & Compatibility

#### Entity Types

- `Dependency.Package` - External packages
- `Dependency.Compatibility` - Compatibility notes
- `Dependency.Alternative` - Alternative packages

#### Dependency Tracking

```javascript
// Track package compatibility
mcp__memory__create_entities({
  entities: [
    {
      name: 'SystemIOPorts',
      entityType: 'Dependency.Package',
      observations: [
        'Package: System.IO.Ports',
        'Version: 8.0.0',
        'AOT Compatible: YES',
        'Trimming: Partial - requires config',
        'Alternative: SerialPortStream (better Linux support)',
        'Issues: Windows-only reliable',
      ],
    },
  ],
});
```

## Advanced Query Strategies

### 1. Context-Aware Searches

```javascript
// Before implementing a feature
let context = await mcp__memory__search_nodes({
  query: 'serial transport implementation',
});
// Returns related patterns, bugs, conventions

// Before making decisions
let decisions = await mcp__memory__search_nodes({
  query: 'performance budget decisions',
});
```

### 2. Relationship Traversal

```javascript
// Find all entities influencing a decision
let graph = await mcp__memory__read_graph();
// Traverse relations to understand impact
```

### 3. Historical Analysis

```javascript
// Track evolution over time
let history = await mcp__memory__open_nodes({
  names: ['IDeviceDriver_v1', 'IDeviceDriver_v2'],
});
// Compare versions and migration paths
```

## Intelligent Memory Patterns

### 1. Decision Journal Pattern

Every significant decision creates:

- Decision entity with context
- Observation with rationale
- Relations to affected components
- Alternative options considered

### 2. Learning Loop Pattern

After each task:

- Store what worked
- Document what didn't
- Link to relevant code
- Create reusable patterns

### 3. Context Preservation Pattern

Before context switches:

- Store current approach
- Document open questions
- Save partial solutions
- Link related investigations

### 4. Knowledge Synthesis Pattern

Periodically:

- Query related entities
- Identify patterns
- Create meta-entities
- Establish new relations

## Memory Hygiene Practices

### 1. Regular Cleanup

```javascript
// Remove outdated observations
mcp__memory__delete_observations({
  deletions: [
    {
      entityName: 'OldPrototype',
      observations: ['Deprecated approach'],
    },
  ],
});
```

### 2. Entity Consolidation

- Merge similar entities
- Update relationships
- Maintain single source of truth

### 3. Sensitive Data Protection

- Never store passwords/keys
- Avoid PII in observations
- Use references to secure storage

## Integration Workflows

### 1. Task Completion Workflow

```javascript
// After completing a task
async function memorizeTaskCompletion(taskId, learnings) {
  // Store implementation details
  await mcp__memory__create_entities({
    entities: [
      {
        name: `Task_${taskId}_Implementation`,
        entityType: 'Task.Completed',
        observations: learnings,
      },
    ],
  });

  // Link to related components
  await mcp__memory__create_relations({
    relations: [
      {
        from: `Task_${taskId}_Implementation`,
        to: 'ProjectArchitecture',
        relationType: 'implements',
      },
    ],
  });
}
```

### 2. Debug Session Workflow

```javascript
// During debugging
async function memorizeDebugSession(issue, solution) {
  // Create bug pattern if new
  await mcp__memory__create_entities({
    entities: [
      {
        name: `BugPattern_${Date.now()}`,
        entityType: 'Bug.Pattern',
        observations: [issue.description, issue.stackTrace],
      },
    ],
  });

  // Store solution
  await mcp__memory__create_entities({
    entities: [
      {
        name: `Solution_${Date.now()}`,
        entityType: 'Bug.Solution',
        observations: [solution.approach, solution.code],
      },
    ],
  });
}
```

### 3. Performance Optimization Workflow

```javascript
// Track optimization attempts
async function memorizeOptimization(metric, before, after, approach) {
  await mcp__memory__create_entities({
    entities: [
      {
        name: `Optimization_${metric}_${Date.now()}`,
        entityType: 'Performance.Optimization',
        observations: [
          `Metric: ${metric}`,
          `Before: ${before}`,
          `After: ${after}`,
          `Improvement: ${(((before - after) / before) * 100).toFixed(1)}%`,
          `Approach: ${approach}`,
        ],
      },
    ],
  });
}
```

## Creative Memory Applications

### 1. Failure Museum

Create a "museum" of failures:

- What was attempted
- Why it failed
- Lessons learned
- Alternative approaches

### 2. Success Patterns Library

Build a library of successful patterns:

- Problem context
- Solution approach
- Implementation code
- Reuse instructions

### 3. Dependency Web

Map the entire dependency web:

- Direct dependencies
- Transitive dependencies
- Version constraints
- Compatibility matrix

### 4. Performance Timeline

Create a timeline of performance:

- Benchmark history
- Optimization attempts
- Regression points
- Recovery strategies

## Best Practices

### 1. Naming Conventions

- Use hierarchical names: `Category.Subcategory.Item`
- Include timestamps for temporal data
- Be consistent across entity types

### 2. Observation Structure

- Start with date/timestamp
- Include location references
- Add status indicators
- Link to code/docs

### 3. Relationship Semantics

- Use consistent relation types
- Create bidirectional links when appropriate
- Document relationship meaning

### 4. Query Optimization

- Use specific search terms
- Leverage entity types
- Cache frequently accessed nodes

## Memory Metrics

Track memory system health:

- Total entities
- Relationship density
- Observation depth
- Query performance
- Knowledge gaps

## Automation Opportunities

### 1. Auto-capture from Task Master

- Task completions
- Decision points
- Blockers and solutions

### 2. Performance Auto-logging

- Benchmark results
- Resource usage
- Latency measurements

### 3. Convention Enforcement

- Detect deviations
- Suggest corrections
- Update patterns

## Conclusion

The Memory MCP server, when used systematically and creatively, becomes a powerful second brain for the project. It captures not just facts but the evolution of understanding, the rationale behind decisions, and the accumulated wisdom from development experience. By following these patterns and practices, the knowledge graph becomes an invaluable asset that grows more useful over time.
