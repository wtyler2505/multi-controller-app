---
model: claude-sonnet-4-20250514
category: documentation
priority: high
tags: ["documentation", "docs", "documentation-sync", "knowledge-management", "project-status"]
description: Comprehensive documentation update and synchronization with universal memory integration and intelligent content recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["documentation", "content-management", "project-status", "knowledge-sync", "technical-writing"]
  complexity-level: "standard"
  
  # Enhanced selection criteria for documentation updates with memory integration
  selection-criteria:
    keyword-match: 0.85       # Strong documentation/sync patterns
    argument-analysis: 0.80   # Documentation type context important
    project-context: 0.90     # Project type affects documentation approach
    error-context: 0.4        # May include documentation inconsistencies
  
  # Specialized documentation agents with memory capabilities
  preferred-agents: ["general-purpose"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.80

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-documentation-workflow"
  
  base-tools:
    - "Read"  # Core documentation reading
    - "Edit"  # Documentation updates
    - "Bash"  # Git and analysis commands
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    documentation-analysis:
      - "mcp__FileScopeMCP__find_important_files"  # Find documentation files
      - "mcp__context7__get-library-docs"  # Documentation best practices
      - "mcp__cipher-memory__open_nodes"  # Load documentation patterns
    
    content-synchronization:
      - "mcp__cipher-memory__create_entities"  # Store documentation insights
      - "mcp__desktop-commander__search_code"  # Find documentation references
      - "mcp__cipher-memory__add_observations"  # Store sync patterns
    
    project-status-tracking:
      - "mcp__cipher-memory__create_relations"  # Connect status patterns
      - "mcp__taskmaster-ai__get_tasks"  # Integration with task status
      - "mcp__cipher-memory__create_entities"  # Store project status patterns

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    documentation-patterns-search:
      - query-pattern: "documentation-update + content-sync + ${project_type}-documentation"
      - tools: ["mcp__cipher-memory__search_nodes"]
    project-status-analysis:
      - query-pattern: "project-status + documentation-sync + knowledge-management"
      - tools: ["mcp__cipher-memory__open_nodes"]
    content-strategies:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "documentation-related"
  execution-memory:
    update-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "documentation-section-update"
    pattern-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "effective-documentation-identified"
    sync-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "documentation-sync-completed"
  post-execution-memory:
    documentation-session-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-documentation-update-session-pattern"
    content-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["update-type-to-approach", "sync-strategy-to-effectiveness", "status-to-documentation"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "documentation-insights + sync-strategies + content-optimization"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - documentation-scope-analysis
      - content-pattern-search
      - memory-pattern-analysis
    
    execution:
      - documentation-analysis-results
      - content-update-processing
      - status-synchronization
      - cross-reference-validation
      - formatting-standardization
    
    post-execution:
      - update-summary
      - sync-validation-results
      - memory-operations
      - documentation-recommendations
  
  structured-metadata:
    command-id: "update-docs"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    documentation-scope: "${update_arguments}"
    files-updated: "${updated_files_count}"
    content-changes: "${content_change_count}"
    status-updates: "${status_update_count}"
    cross-references-added: "${cross_reference_count}"
    execution-time: "${duration_ms}"
    update-quality-score: "${documentation_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-documentation-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
documentation-pattern-recognition: true

allowed-tools: Read, Write, Edit, Bash, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__desktop-commander__*, mcp__taskmaster-ai__*

argument-hint: [doc-type] | --implementation | --api | --architecture | --sync | --validate | --comprehensive

pre-execution:
  validate-tools: true
  load-context: true
  analyze-documentation-state: true
  search-documentation-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-documentation-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Documentation Update & Synchronization (Universal Integration)

Update project documentation systematically with universal memory integration and intelligent content recognition: $ARGUMENTS

**ENHANCED WORKFLOW**: This command utilizes specialized documentation agents with complete Cipher Memory integration for documentation pattern recognition, content synchronization, and knowledge management persistence.

## Enhanced Pre-Execution Memory Analysis
Before documentation updates, the system will:
1. **Search documentation patterns**: Query Cipher Memory for effective documentation strategies and content organization approaches
2. **Load project knowledge**: Retrieve project status patterns and documentation best practices
3. **Analyze sync strategies**: Understanding documentation synchronization and cross-reference methodologies
4. **Connect knowledge management**: Access comprehensive content management and technical writing patterns

## Current Documentation State

- Documentation structure: !`find . -name "*.md" | head -10`
- Specs directory: @specs/ (if exists)
- Implementation status: !`grep -r "âœ…\|âŒ\|âš ï¸" docs/ specs/ 2>/dev/null | wc -l` status indicators
- Recent changes: !`git log --oneline --since="1 week ago" -- "*.md" | head -5`
- Project progress: @CLAUDE.md or @README.md (if exists)

## Task

## Documentation Analysis

1. Review current documentation status:
   - Check `specs/implementation_status.md` for overall project status
   - Review implemented phase document (`specs/phase{N}_implementation_plan.md`)
   - Review `specs/flutter_structurizr_implementation_spec.md` and `specs/flutter_structurizr_implementation_spec_updated.md`
   - Review `specs/testing_plan.md` to ensure it is current given recent test passes, failures, and changes
   - Examine `CLAUDE.md` and `README.md` for project-wide documentation
   - Check for and document any new lessons learned or best practices in CLAUDE.md

2. Analyze implementation and testing results:
   - Review what was implemented in the last phase
   - Review testing results and coverage
   - Identify new best practices discovered during implementation
   - Note any implementation challenges and solutions
   - Cross-reference updated documentation with recent implementation and test results to ensure accuracy

## Documentation Updates

1. Update phase implementation document:
   - Mark completed tasks with âœ… status
   - Update implementation percentages
   - Add detailed notes on implementation approach
   - Document any deviations from original plan with justification
   - Add new sections if needed (lessons learned, best practices)
   - Document specific implementation details for complex components
   - Include a summary of any new troubleshooting tips or workflow improvements discovered during the phase

2. Update implementation status document:
   - Update phase completion percentages
   - Add or update implementation status for components
   - Add notes on implementation approach and decisions
   - Document best practices discovered during implementation
   - Note any challenges overcome and solutions implemented

3. Update implementation specification documents:
   - Mark completed items with âœ… or strikethrough but preserve original requirements
   - Add notes on implementation details where appropriate
   - Add references to implemented files and classes
   - Update any implementation guidance based on experience

4. Update CLAUDE.md and README.md if necessary:
   - Add new best practices
   - Update project status
   - Add new implementation guidance
   - Document known issues or limitations
   - Update usage examples to include new functionality

5. Document new testing procedures:
   - Add details on test files created
   - Include test running instructions
   - Document test coverage
   - Explain testing approach for complex components

## Documentation Formatting and Structure

1. Maintain consistent documentation style:
   - Use clear headings and sections
   - Include code examples where helpful
   - Use status indicators (âœ…, âš ï¸, âŒ) consistently
   - Maintain proper Markdown formatting

2. Ensure documentation completeness:
   - Cover all implemented features
   - Include usage examples
   - Document API changes or additions
   - Include troubleshooting guidance for common issues

## Guidelines

- DO NOT CREATE new specification files
- UPDATE existing files in the `specs/` directory
- Maintain consistent documentation style
- Include practical examples where appropriate
- Cross-reference related documentation sections
- Document best practices and lessons learned
- Provide clear status updates on project progress
- Update numerical completion percentages
- Ensure documentation reflects actual implementation

Provide a summary of documentation updates after completion, including:
1. Files updated
2. Major changes to documentation
3. Updated completion percentages
4. New best practices documented
5. Status of the overall project after this phase

## Universal Memory Integration Outcomes

### Documentation Knowledge Storage
This command will automatically:
- **Store comprehensive documentation update sessions** in Cipher Memory for content pattern recognition
- **Create relationships** between documentation types, update strategies, and project effectiveness
- **Document content management methodologies** and synchronization best practices
- **Build knowledge graph** of documentation-project mappings and content optimization strategies

### Cross-Command Learning Enhancement
Documentation patterns will improve:
- Future project commands through documented project status and progress patterns
- Setup commands via established documentation and knowledge sharing practices
- Quality assurance commands through documentation quality integration
- Development commands via proven documentation synchronization workflows

### Advanced Documentation Intelligence
- **Content Optimization**: Automatic identification of optimal documentation structures based on project characteristics
- **Sync Strategy**: Intelligent documentation synchronization based on successful content management patterns
- **Status Tracking**: Smart project status documentation using proven tracking and reporting methodologies
- **Cross-Reference Management**: Automated cross-reference creation and validation using established linking patterns

### Intelligent Documentation Enhancement Features
- **Project-Specific Documentation**: Tailored documentation approaches based on project type and development stage
- **Context-Aware Content Updates**: Smart content updates considering project progress and team knowledge needs
- **Progressive Documentation Learning**: Each update session improves future documentation through pattern accumulation
- **Cross-Project Documentation Knowledge**: Shared documentation insights across different projects and domains

### Centralized Documentation Logging
All documentation update operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete update methodology and content processing tracking
- Documentation synchronization results and validation metrics
- Memory operations for documentation pattern capture and learning
- Content optimization effectiveness and cross-reference management

**Next Commands**: Enhanced documentation patterns will automatically improve commands like `create-onboarding-guide`, `project-health-check`, `create-architecture-documentation`, and `setup-development-environment`.

