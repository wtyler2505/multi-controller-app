# Project Reflection - AI Code Assistant Optimization

You are an expert in prompt engineering, specializing in optimizing AI code assistant instructions for maximum effectiveness and efficiency.

## Core Objective
Analyze the current project state, codebase patterns, and development practices to generate optimized instructions for AI code assistants working on this project.

## Analysis Process

### 1. Project Context Analysis
- **Architecture Review**: Examine the overall architecture and design patterns
- **Technology Stack**: Identify all technologies, frameworks, and libraries in use
- **Code Conventions**: Detect coding standards, naming conventions, and style guides
- **Directory Structure**: Map the project organization and module boundaries
- **Development Workflow**: Understand git workflow, CI/CD, and deployment processes

### 2. Codebase Pattern Recognition
- **Common Patterns**: Identify recurring design patterns and implementation approaches
- **Anti-patterns**: Detect areas that need improvement or refactoring
- **Technical Debt**: Assess accumulated debt and its impact
- **Quality Metrics**: Evaluate test coverage, documentation, and code quality
- **Performance Characteristics**: Understand performance requirements and optimizations

### 3. Development Practice Assessment
- **Testing Strategy**: Review unit, integration, and e2e testing approaches
- **Documentation Standards**: Assess inline comments, README files, and API docs
- **Error Handling**: Examine error handling patterns and logging practices
- **Security Practices**: Review authentication, authorization, and data protection
- **Dependency Management**: Analyze dependency health and update strategies

### 4. AI Assistant Optimization
Generate specific recommendations for AI assistants working on this project:

#### Context Awareness
- Key files and modules to understand first
- Critical dependencies and their relationships
- Domain-specific terminology and concepts
- Project-specific conventions and rules

#### Task Execution Guidelines
- Preferred implementation patterns for common tasks
- Testing requirements for different types of changes
- Documentation standards to maintain
- Performance considerations to keep in mind
- Security requirements to enforce

#### Common Pitfalls to Avoid
- Known issues and workarounds
- Deprecated patterns to avoid
- Performance bottlenecks to watch for
- Security vulnerabilities to prevent
- Breaking changes to be careful about

### 5. Generated Instructions
Create a comprehensive CLAUDE.md update with:

```markdown
## Project-Specific AI Assistant Instructions

### Quick Context
[Brief project overview and key technologies]

### Critical Files
[List of essential files to understand]

### Implementation Guidelines
[Specific patterns and practices to follow]

### Testing Requirements
[What tests to write/run for different changes]

### Common Tasks
[Step-by-step guides for frequent operations]

### Warnings & Pitfalls
[Things to avoid or be careful about]

### Performance Considerations
[Key performance requirements and optimizations]

### Security Requirements
[Security practices that must be followed]
```

## Execution Steps

1. **Scan Project Structure**
   - Read package.json/Cargo.toml for dependencies
   - Analyze directory structure
   - Review configuration files

2. **Sample Code Analysis**
   - Read 5-10 representative source files
   - Identify coding patterns and conventions
   - Note testing approaches

3. **Documentation Review**
   - Check README.md and other docs
   - Review existing CLAUDE.md if present
   - Examine inline code comments

4. **Workflow Analysis**
   - Review .github/workflows if present
   - Check build/deployment scripts
   - Understand development commands

5. **Generate Optimizations**
   - Create targeted instructions
   - Include specific examples
   - Provide clear do's and don'ts

6. **Output Results**
   - Present findings in structured format
   - Provide actionable recommendations
   - Suggest CLAUDE.md updates

## Output Format

Present findings as:

```
# Project Reflection Analysis

## 📊 Project Overview
[High-level summary]

## 🏗️ Architecture Insights
[Key architectural findings]

## 📝 Code Patterns Detected
[Common patterns and practices]

## ⚠️ Areas for Improvement
[Technical debt and issues]

## 🤖 AI Assistant Optimization
[Specific instructions for AI assistants]

## 📋 Recommended CLAUDE.md Updates
[Proposed additions/changes to CLAUDE.md]

## ✅ Action Items
[Concrete next steps]
```

## Usage
Run this command periodically to keep AI assistant instructions optimized as the project evolves. Especially useful after:
- Major refactoring
- Technology stack changes
- New team members joining
- Significant feature additions
- Architecture updates