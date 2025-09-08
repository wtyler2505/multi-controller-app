---
model: claude-sonnet-4-20250514
category: project-management
priority: medium
tags: ["project-management", "github"]
description: PRD Parse
allowed-tools: Bash, Read, Write, LS
---

# Epic: $ARGUMENTS

## Overview
Brief technical summary of the implementation approach

## Architecture Decisions
- Key technical decisions and rationale
- Technology choices
- Design patterns to use

## Technical Approach
### Frontend Components
- UI components needed
- State management approach
- User interaction patterns

### Backend Services
- API endpoints required
- Data models and schema
- Business logic components

### Infrastructure
- Deployment considerations
- Scaling requirements
- Monitoring and observability

## Implementation Strategy
- Development phases
- Risk mitigation
- Testing approach

## Task Breakdown Preview
High-level task categories that will be created:
- [ ] Category 1: Description
- [ ] Category 2: Description
- [ ] etc.

## Dependencies
- External service dependencies
- Internal team dependencies
- Prerequisite work

## Success Criteria (Technical)
- Performance benchmarks
- Quality gates
- Acceptance criteria

## Estimated Effort
- Overall timeline estimate
- Resource requirements
- Critical path items
```

### 4. Frontmatter Guidelines
- **name**: Use the exact feature name (same as $ARGUMENTS)
- **status**: Always start with "backlog" for new epics
- **created**: Get REAL current datetime by running: `date -u +"%Y-%m-%dT%H:%M:%SZ"`
- **progress**: Always start with "0%" for new epics
- **prd**: Reference the source PRD file path
- **github**: Leave placeholder text - will be updated during sync

### 5. Output Location
Create the directory structure if it doesn't exist:
- `.claude/epics/$ARGUMENTS/` (directory)
- `.claude/epics/$ARGUMENTS/epic.md` (epic file)

### 6. Quality Validation

Before saving the epic, verify:
- [ ] All PRD requirements are addressed in the technical approach
- [ ] Task breakdown categories cover all implementation areas
- [ ] Dependencies are technically accurate
- [ ] Effort estimates are realistic
- [ ] Architecture decisions are justified

### 7. Post-Creation

After successfully creating the epic:
1. Confirm: "âœ… Epic created: .claude/epics/$ARGUMENTS/epic.md"
2. Show summary of:
   - Number of task categories identified
   - Key architecture decisions
   - Estimated effort
3. Suggest next step: "Ready to break down into tasks? Run: /pm:epic-decompose $ARGUMENTS"

## Error Recovery

If any step fails:
- Clearly explain what went wrong
- If PRD is incomplete, list specific missing sections
- If technical approach is unclear, identify what needs clarification
- Never create an epic with incomplete information

Focus on creating a technically sound implementation plan that addresses all PRD requirements while being practical and achievable for "$ARGUMENTS".

## IMPORTANT:
- Aim for as few tasks as possible and limit the total number of tasks to 10 or less.
- When creating the epic, identify ways to simplify and improve it. Look for ways to leverage existing functionality instead of creating more code when possible.



