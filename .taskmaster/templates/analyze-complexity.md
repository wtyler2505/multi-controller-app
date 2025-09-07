# Project Complexity Analysis Template

You are an expert software architect analyzing project complexity to provide insights for better planning and risk management.

## Project Overview

**Project Name**: {{projectName}}
**Total Tasks**: {{totalTasks}}
**Analysis Scope**: {{analysisScope}}

## Tasks to Analyze

{{#each tasks}}
### Task {{id}}: {{title}}

**Description**: {{description}}
**Current Status**: {{status}}
**Priority**: {{priority}}
**Dependencies**: {{#each dependencies}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{#if details}}**Details**: {{details}}{{/if}}
{{#if subtasks.length}}**Subtasks**: {{subtasks.length}}{{/if}}

{{/each}}

## Analysis Framework

Analyze each task across these dimensions:

### 1. Technical Complexity (1-10 scale)
- **Code Complexity**: New patterns, algorithms, data structures
- **Integration Complexity**: APIs, external systems, interfaces  
- **Infrastructure Complexity**: Deployment, configuration, environment setup
- **Performance Requirements**: Optimization, scaling, resource constraints

### 2. Business Complexity (1-10 scale)
- **Requirements Clarity**: How well-defined are the requirements?
- **Stakeholder Alignment**: Multiple stakeholders with competing needs?
- **Domain Knowledge Required**: Specialized expertise needed?
- **Regulatory/Compliance**: Legal, security, or compliance requirements?

### 3. Risk Factors
- **Technical Risks**: Unknown technologies, performance unknowns
- **Resource Risks**: Skills availability, time constraints
- **Dependency Risks**: External dependencies, blocking relationships
- **Change Risks**: Likelihood of requirements changing

### 4. Implementation Factors
- **Effort Estimation**: Person-hours required
- **Skill Requirements**: Expertise levels needed
- **Testing Complexity**: Unit, integration, performance testing needs
- **Documentation Needs**: Technical docs, user guides, API docs

## Required Output Format

```json
{
  "overallComplexity": {
    "score": "1-10 average across all tasks",
    "summary": "Brief assessment of project complexity"
  },
  "taskAnalysis": [
    {
      "taskId": "X.Y",
      "complexityScore": "1-10",
      "technicalComplexity": "1-10",
      "businessComplexity": "1-10", 
      "riskLevel": "low|medium|high",
      "estimatedHours": "number",
      "skillLevel": "junior|intermediate|senior|expert",
      "criticalPath": true/false,
      "recommendedSubtasks": "3-7 number",
      "keyRisks": ["List of main risk factors"],
      "dependencies": ["Critical dependencies"],
      "notes": "Additional insights or concerns"
    }
  ],
  "recommendations": {
    "priorityOrder": ["Recommended task execution order"],
    "parallelizationOpportunities": ["Tasks that can be done in parallel"],
    "criticalPathTasks": ["Tasks that affect project timeline most"],
    "skillGaps": ["Areas where additional expertise may be needed"],
    "riskMitigation": ["Suggested approaches to reduce risks"]
  },
  "resourcePlanning": {
    "totalEstimatedHours": "sum across all tasks",
    "skillMix": {
      "junior": "hours requiring junior level",
      "intermediate": "hours requiring intermediate level", 
      "senior": "hours requiring senior level",
      "expert": "hours requiring expert level"
    },
    "timelineRecommendation": "suggested project duration and phases"
  }
}
```

## Analysis Instructions

1. **Score each task** objectively across the complexity dimensions
2. **Identify bottlenecks** and critical path dependencies
3. **Assess resource needs** realistically based on task complexity
4. **Provide actionable insights** for project planning and risk management
5. **Consider the specific technology stack and domain** of this project

Focus on practical insights that will help with task planning, resource allocation, and risk management.