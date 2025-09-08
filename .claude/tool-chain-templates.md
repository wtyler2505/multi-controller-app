# Tool Chain Templates for Command Integration

## Overview
Predefined sequences of MCP server tools optimized for common development workflows. These templates ensure consistent, efficient tool usage across commands.

## Core Tool Chains

### 1. Task Coordination Chain
**Purpose**: Task Master integration and project management workflows  
**Primary Agents**: task-orchestrator, task-executor, task-checker  
**Usage**: All Task Master commands, project coordination, workflow automation

```yaml
task-coordination:
  description: "Complete Task Master workflow integration"
  
  # Phase 1: Context Gathering
  context-phase:
    - tool: "mcp__taskmaster-ai__get_tasks"
      purpose: "Get current task state"
      timeout: 5000
      
    - tool: "mcp__cipher-memory__search_nodes" 
      purpose: "Find related knowledge"
      query: "task + project + context"
      timeout: 3000
      
    - tool: "mcp__memory__search_nodes"
      purpose: "Additional context search"
      fallback: true
      timeout: 2000
  
  # Phase 2: Analysis
  analysis-phase:
    - tool: "mcp__taskmaster-ai__complexity_report"
      purpose: "Analyze task complexity"
      conditional: "if task-expansion-needed"
      
    - tool: "mcp__clear-thought__sequentialthinking"
      purpose: "Plan execution strategy"
      parameters:
        pattern: "chain"
        depth: 3
      
  # Phase 3: Execution
  execution-phase:
    - tool: "mcp__taskmaster-ai__next_task"
      purpose: "Get next actionable task"
      
    - tool: "mcp__taskmaster-ai__set_task_status"
      purpose: "Update task progress"
      timing: "during-execution"
      
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Execute development tasks"
      conditional: "if implementation-required"
  
  # Phase 4: Documentation
  finalization-phase:
    - tool: "mcp__cipher-memory__create_entities"
      purpose: "Store execution results"
      
    - tool: "mcp__taskmaster-ai__generate"
      purpose: "Update task files"
      
  # Error handling
  error-recovery:
    - tool: "mcp__taskmaster-ai__validate_dependencies"
      purpose: "Check for blocking issues"
    - tool: "mcp__clear-thought__debuggingapproach"
      purpose: "Systematic problem resolution"
      
  performance-targets:
    total-time: "< 15 seconds"
    success-rate: "> 90%"
    user-intervention: "< 10%"
```

### 2. Rust Development Chain  
**Purpose**: Rust project development, building, and validation  
**Primary Agents**: rust-async-specialist, cargo-build-engineer, rust-performance-monitor  
**Usage**: Rust-specific commands, build processes, async debugging

```yaml
rust-development:
  description: "Complete Rust development workflow"
  
  # Phase 1: Project Analysis
  analysis-phase:
    - tool: "mcp__FileScopeMCP__find_important_files"
      purpose: "Identify key Rust files"
      parameters:
        minImportance: 7
        limit: 20
        
    - tool: "mcp__desktop-commander__read_file"
      purpose: "Read Cargo.toml for context"
      path: "./Cargo.toml"
      
    - tool: "mcp__cipher-memory__search_nodes"
      purpose: "Find Rust patterns"
      query: "rust + async + patterns"
  
  # Phase 2: Code Understanding  
  understanding-phase:
    - tool: "mcp__FileScopeMCP__recalculate_importance"
      purpose: "Update file dependency mapping"
      
    - tool: "mcp__context7__resolve-library-id"
      purpose: "Resolve Rust library documentation" 
      conditional: "if unknown-library-found"
      
    - tool: "mcp__context7__get-library-docs"
      purpose: "Get relevant documentation"
      tokens: 5000
  
  # Phase 3: Implementation
  implementation-phase:
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Start Rust development environment"
      command: "cargo check"
      timeout: 30000
      
    - tool: "mcp__desktop-commander__interact_with_process"
      purpose: "Execute Rust commands"
      timeout: 60000
      
    - tool: "mcp__desktop-commander__edit_block"
      purpose: "Apply code changes"
      conditional: "if edits-required"
  
  # Phase 4: Validation
  validation-phase:
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Run tests"
      command: "cargo test"
      timeout: 120000
      
    - tool: "mcp__desktop-commander__start_process" 
      purpose: "Check formatting"
      command: "cargo fmt --check"
      
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Run clippy"
      command: "cargo clippy -- -W clippy::all"
  
  # Phase 5: Documentation
  documentation-phase:
    - tool: "mcp__cipher-memory__create_entities"
      purpose: "Store implementation patterns"
      
    - tool: "mcp__cipher-memory__create_relations"
      purpose: "Link related concepts"
      
  performance-targets:
    total-time: "< 45 seconds"
    build-success-rate: "> 95%"
    test-pass-rate: "> 90%"
```

### 3. Performance Analysis Chain
**Purpose**: Performance monitoring, profiling, and optimization workflows  
**Primary Agents**: rust-performance-monitor, egui-performance-optimizer  
**Usage**: Performance commands, optimization analysis, bottleneck identification

```yaml
performance-analysis:
  description: "Comprehensive performance analysis workflow"
  
  # Phase 1: Baseline Measurement
  measurement-phase:
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Start monitoring process"
      command: "cargo run --release"
      run_in_background: true
      timeout: 10000
      
    - tool: "mcp__desktop-commander__list_sessions"
      purpose: "Track active processes"
      
    - tool: "mcp__FileScopeMCP__find_important_files"
      purpose: "Identify performance-critical files"
      parameters:
        minImportance: 8
  
  # Phase 2: Analysis  
  analysis-phase:
    - tool: "mcp__clear-thought__sequentialthinking"
      purpose: "Analyze performance patterns"
      parameters:
        pattern: "tree"
        depth: 4
        
    - tool: "mcp__desktop-commander__interact_with_process"
      purpose: "Collect performance metrics"
      input: "performance monitoring commands"
      
    - tool: "mcp__FileScopeMCP__recalculate_importance"
      purpose: "Update importance based on performance impact"
  
  # Phase 3: Bottleneck Identification
  identification-phase:
    - tool: "mcp__clear-thought__debuggingapproach"
      purpose: "Systematic bottleneck analysis"
      approach: "binary_search"
      
    - tool: "mcp__desktop-commander__search_code"
      purpose: "Find performance-critical code patterns"
      pattern: "async|await|lock|mutex"
      contextLines: 3
      
    - tool: "mcp__context7__get-library-docs"
      purpose: "Check optimization documentation"
      topic: "performance optimization"
  
  # Phase 4: Optimization Recommendations
  optimization-phase:
    - tool: "mcp__clear-thought__collaborativereasoning"
      purpose: "Multi-perspective optimization analysis"
      personas: ["PerformanceExpert", "RustSpecialist", "SystemsEngineer"]
      
    - tool: "mcp__cipher-memory__search_nodes"
      purpose: "Find proven optimization patterns"
      query: "rust + performance + optimization + patterns"
  
  # Phase 5: Results Storage
  storage-phase:
    - tool: "mcp__cipher-memory__create_entities"
      purpose: "Store performance analysis results"
      
    - tool: "mcp__desktop-commander__force_terminate"
      purpose: "Clean up monitoring processes"
      conditional: "if background-processes-active"
      
  performance-targets:
    analysis-time: "< 30 seconds"
    bottleneck-identification-rate: "> 85%"
    optimization-suggestion-quality: "> 80%"
```

### 4. Testing Validation Chain
**Purpose**: Test generation, execution, and validation workflows  
**Primary Agents**: mock-test-orchestrator, cargo-build-engineer  
**Usage**: Testing commands, quality assurance, validation processes

```yaml
testing-validation:
  description: "Comprehensive testing and validation workflow"
  
  # Phase 1: Test Environment Setup
  setup-phase:
    - tool: "mcp__FileScopeMCP__find_important_files"
      purpose: "Identify testable components"
      parameters:
        tags: ["src", "lib"]
        minImportance: 6
        
    - tool: "mcp__desktop-commander__read_file"
      purpose: "Check existing test configuration"
      path: "Cargo.toml"
      
    - tool: "mcp__cipher-memory__search_nodes"
      purpose: "Find testing patterns and examples"
      query: "rust + testing + patterns + examples"
  
  # Phase 2: Test Generation
  generation-phase:
    - tool: "mcp__context7__resolve-library-id"
      purpose: "Find testing framework documentation"
      libraryName: "tokio-test"
      
    - tool: "mcp__context7__get-library-docs"
      purpose: "Get testing best practices"
      topic: "testing patterns"
      tokens: 8000
      
    - tool: "mcp__clear-thought__sequentialthinking"
      purpose: "Plan comprehensive test coverage"
      parameters:
        pattern: "tree"
        focus: "edge cases and error conditions"
  
  # Phase 3: Test Implementation
  implementation-phase:
    - tool: "mcp__desktop-commander__create_directory"
      purpose: "Create test directory structure"
      conditional: "if tests-directory-missing"
      
    - tool: "mcp__desktop-commander__write_file"
      purpose: "Generate test files"
      mode: "rewrite"
      
    - tool: "mcp__desktop-commander__edit_block"
      purpose: "Update existing tests"
      conditional: "if tests-exist"
  
  # Phase 4: Test Execution
  execution-phase:
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Run test suite"
      command: "cargo test"
      timeout: 180000
      
    - tool: "mcp__desktop-commander__start_process"
      purpose: "Generate coverage report"
      command: "cargo tarpaulin --out Html"
      timeout: 240000
      conditional: "if coverage-requested"
      
    - tool: "mcp__desktop-commander__read_process_output"
      purpose: "Analyze test results"
      timeout: 5000
  
  # Phase 5: Result Analysis
  analysis-phase:
    - tool: "mcp__clear-thought__debuggingapproach"
      purpose: "Analyze test failures"
      conditional: "if tests-failed"
      approach: "cause_elimination"
      
    - tool: "mcp__cipher-memory__create_entities"
      purpose: "Store test results and patterns"
      
    - tool: "mcp__taskmaster-ai__update_task"
      purpose: "Update related tasks"
      conditional: "if task-context-available"
      
  performance-targets:
    test-generation-time: "< 20 seconds"
    test-execution-time: "< 3 minutes"
    coverage-target: "> 80%"
```

### 5. Research Heavy Chain
**Purpose**: Research, documentation, and knowledge-intensive workflows  
**Primary Agents**: general-purpose  
**Usage**: Architecture decisions, documentation, complex analysis, learning tasks

```yaml
research-heavy:
  description: "Comprehensive research and analysis workflow"
  
  # Phase 1: Knowledge Gathering
  gathering-phase:
    - tool: "mcp__cipher-memory__search_nodes"
      purpose: "Search existing knowledge base"
      timeout: 5000
      
    - tool: "mcp__memory__search_nodes"
      purpose: "Additional knowledge search"
      fallback: true
      
    - tool: "mcp__perplexity-ask__perplexity_ask"
      purpose: "Real-time research"
      conditional: "if external-research-needed"
      timeout: 15000
  
  # Phase 2: Context Resolution
  resolution-phase:
    - tool: "mcp__context7__resolve-library-id"
      purpose: "Identify relevant libraries/frameworks"
      conditional: "if technical-research"
      
    - tool: "mcp__context7__get-library-docs"
      purpose: "Get authoritative documentation"
      tokens: 10000
      
    - tool: "mcp__FileScopeMCP__find_important_files"
      purpose: "Identify relevant project files"
      conditional: "if project-context-needed"
  
  # Phase 3: Deep Analysis
  analysis-phase:
    - tool: "mcp__clear-thought__sequentialthinking"
      purpose: "Systematic analysis"
      parameters:
        pattern: "graph"
        depth: 5
        
    - tool: "mcp__clear-thought__collaborativereasoning"
      purpose: "Multi-perspective analysis"
      personas: ["Expert", "Critic", "PracticalEngineer"]
      
    - tool: "mcp__clear-thought__decisionframework"
      purpose: "Structured decision analysis"
      conditional: "if decision-required"
  
  # Phase 4: Synthesis
  synthesis-phase:
    - tool: "mcp__clear-thought__structuredargumentation"
      purpose: "Build comprehensive argument"
      
    - tool: "mcp__clear-thought__metacognitivemonitoring"
      purpose: "Validate reasoning quality"
      
    - tool: "mcp__clear-thought__session_export"
      purpose: "Export reasoning session"
      conditional: "if preserve-analysis"
  
  # Phase 5: Knowledge Storage
  storage-phase:
    - tool: "mcp__cipher-memory__create_entities"
      purpose: "Store research findings"
      
    - tool: "mcp__cipher-memory__create_relations"
      purpose: "Link concepts and findings"
      
    - tool: "mcp__memory__create_entities"
      purpose: "Additional storage"
      fallback: true
      
  performance-targets:
    research-depth: "comprehensive"
    accuracy-rate: "> 90%"
    knowledge-retention: "> 95%"
```

## Specialized Chains

### 6. Hardware Integration Chain
**Purpose**: Serial communication, device integration, hardware debugging  
**Primary Agents**: serial-comm-specialist, serial-hardware-specialist

```yaml
hardware-integration:
  description: "Hardware and serial communication workflow"
  
  phases:
    device-discovery:
      - "mcp__desktop-commander__start_process" # Port enumeration
      - "mcp__desktop-commander__interact_with_process" # Device probing
      
    protocol-analysis:
      - "mcp__cipher-memory__search_nodes" # Find protocol patterns
      - "mcp__context7__get-library-docs" # Serial library docs
      
    implementation:
      - "mcp__desktop-commander__edit_block" # Code changes
      - "mcp__FileScopeMCP__recalculate_importance" # Update dependencies
      
    validation:
      - "mcp__desktop-commander__start_process" # Test communication
      - "mcp__cipher-memory__create_entities" # Store results
```

### 7. UI Performance Chain  
**Purpose**: egui performance optimization, rendering analysis  
**Primary Agents**: egui-performance-optimizer

```yaml
ui-performance:
  description: "UI and rendering performance optimization"
  
  phases:
    performance-profiling:
      - "mcp__desktop-commander__start_process" # Profile renderer
      - "mcp__FileScopeMCP__find_important_files" # Find UI files
      
    bottleneck-analysis:
      - "mcp__clear-thought__debuggingapproach" # Systematic analysis
      - "mcp__desktop-commander__search_code" # Find performance issues
      
    optimization:
      - "mcp__context7__get-library-docs" # egui optimization guides
      - "mcp__desktop-commander__edit_block" # Apply optimizations
      
    validation:
      - "mcp__desktop-commander__start_process" # Measure improvements
      - "mcp__cipher-memory__create_entities" # Store optimization patterns
```

## Chain Selection Logic

### 8. Automatic Chain Selection
```yaml
chain-selection-rules:
  # Direct category mapping
  task-management: "task-coordination"
  performance-optimization: "performance-analysis" 
  testing-quality: "testing-validation"
  
  # Agent-based selection
  task-orchestrator: "task-coordination"
  rust-performance-monitor: "performance-analysis"
  mock-test-orchestrator: "testing-validation"
  general-purpose: "research-heavy"
  
  # Context-based selection
  rust + hardware: "hardware-integration"
  rust + ui: "ui-performance" 
  rust + async: "rust-development"
  complex + research: "research-heavy"
  
  # Fallback chain
  default: "rust-development"
```

### 9. Chain Customization
```yaml
chain-customization:
  parameter-overrides:
    timeout-multiplier: 1.0      # Adjust all timeouts
    skip-phases: []              # Skip specific phases
    additional-tools: []         # Add extra tools
    tool-substitutions: {}       # Replace specific tools
    
  conditional-execution:
    enable-background: boolean   # Allow background processes
    require-user-confirmation: boolean # Ask before destructive operations
    auto-retry-on-failure: boolean # Retry failed operations
    
  performance-tuning:
    parallel-tool-execution: boolean # Run compatible tools in parallel
    cache-tool-results: boolean      # Cache results for reuse
    optimize-for-speed: boolean      # Prioritize speed over thoroughness
```

These tool chains provide consistent, optimized workflows that can be reused across multiple commands while maintaining flexibility for customization and optimization.