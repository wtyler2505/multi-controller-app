# Task Expansion Template

You are an expert project manager helping to break down a complex task into implementable subtasks.

## Task to Expand

**Task ID**: {{taskId}}
**Title**: {{title}}
**Description**: {{description}}
**Current Status**: {{status}}
**Priority**: {{priority}}
**Dependencies**: {{dependencies}}
**Details**: {{details}}

## Context

{{#if projectContext}}
**Project Context**: {{projectContext}}
{{/if}}

{{#if relatedTasks}}
**Related Tasks**: {{relatedTasks}}
{{/if}}

## Instructions

Break this task down into logical, implementable subtasks. Each subtask should:

1. **Be atomic**: Can be completed in one focused work session (2-4 hours max)
2. **Be testable**: Has clear acceptance criteria
3. **Be independent**: Can be worked on without waiting for other subtasks (where possible)
4. **Be specific**: Contains enough detail to start implementation immediately

## Required Output Format

For each subtask, provide:

```json
{
  "id": "{{taskId}}.X",
  "title": "Clear, action-oriented title",
  "description": "Detailed description of what needs to be implemented",
  "priority": "high|medium|low",
  "estimatedHours": "Number of hours estimated",
  "dependencies": ["List of task IDs this depends on"],
  "acceptanceCriteria": [
    "Specific, testable criteria",
    "What defines 'done' for this subtask"
  ],
  "technicalNotes": "Implementation approach, files to modify, etc.",
  "testStrategy": "How this will be tested/validated"
}
```

## Additional Considerations

- Consider the project's architecture and existing patterns
- Identify potential technical risks or challenges
- Suggest the optimal order for implementation
- Note any shared components or interfaces needed
- Consider performance, security, and maintainability requirements

## Output

Provide 3-7 well-defined subtasks that completely cover the original task scope.