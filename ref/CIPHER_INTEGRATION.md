# Cipher Integration Reference

## Overview
Cipher v0.2.2 serves as the central MCP aggregator hub for the Multi-Controller App, managing 8 MCP servers with 106 total tools while providing dual-layer memory capabilities.

## Installation & Configuration

### Global Installation
```bash
npm install -g @byterover/cipher
```

### Critical Paths
- **Executable**: `C:\Users\wtyle\AppData\Roaming\npm\node_modules\@byterover\cipher\dist\src\app\index.cjs`
- **Config**: `.cipher/cipher.yml`
- **MCP Entry**: `.mcp.json` (single aggregator entry)

## Embedding Configuration (Ollama)

### Setup Process
1. Install Ollama: `winget install Ollama.Ollama`
2. Pull embedding model: `ollama pull nomic-embed-text`
3. Verify: `ollama list`

### Configuration in cipher.yml
```yaml
embedding:
  type: ollama
  model: nomic-embed-text
  baseUrl: http://localhost:11434
```

### Why Ollama?
- **Cost**: Free local embeddings (vs OpenAI API costs)
- **Privacy**: Data stays local
- **Performance**: 274MB model, efficient processing
- **Reliability**: No API quotas or rate limits

## MCP Aggregation

### Aggregated Servers (8 Total)
1. **taskmaster-ai** - Task management (authoritative)
2. **desktop-commander** - File/terminal operations
3. **FileScopeMCP** - Code graphing & analysis
4. **clear-thought** - Structured reasoning tools
5. **context7** - Official documentation
6. **perplexity-ask** - Research capabilities
7. **memory** - Long-term memory storage
8. **time-server** - Time/timezone utilities

### Tool Count: 106 Total
- Task Master: 35+ tools
- Desktop Commander: 25+ tools
- FileScope: 15+ tools
- Clear Thought: 10+ tools
- Others: 21+ tools

## Memory Architecture

### Dual-Layer System
1. **System 1 (Fast)**
   - Concepts and logic
   - Quick retrieval
   - Pattern matching

2. **System 2 (Reasoning)**
   - Detailed reasoning traces
   - Decision processes
   - Complex analysis

### Memory Configuration
```yaml
memory:
  maxMemoryMB: 512
  retentionDays: 30
  vectorStore:
    type: in-memory
```

## .mcp.json Configuration

### Single Aggregator Entry
```json
{
  "mcpServers": {
    "cipher-aggregator": {
      "command": "node",
      "args": [
        "C:\\Users\\wtyle\\AppData\\Roaming\\npm\\node_modules\\@byterover\\cipher\\dist\\src\\app\\index.cjs",
        "--mode", "mcp",
        "--agent", "./.cipher/cipher.yml"
      ],
      "env": {
        "ANTHROPIC_API_KEY": "${ANTHROPIC_API_KEY}",
        "USE_MEMORY_ONLY": "true",
        "CIPHER_MAX_MEMORY_MB": "512",
        "MCP_SERVER_MODE": "aggregator"
      }
    }
  }
}
```

## Troubleshooting

### Common Issues & Solutions

#### 1. Connection Timeout (30 seconds)
**Problem**: Cipher times out during MCP handshake
**Solution**: 
- Use correct executable path (`dist/src/app/index.cjs`)
- Switch from npx to direct node execution
- Check cipher.yml is properly formatted

#### 2. Embeddings Disabled
**Problem**: "Embeddings disabled due to OpenAI API quota"
**Solution**:
- Install Ollama
- Pull nomic-embed-text model
- Update cipher.yml to use ollama type

#### 3. maxIterations Undefined
**Problem**: Configuration error on startup
**Solution**:
- Add `maxIterations: 50` to agent config
- Ensure llm section is complete
- Check systemPrompt structure

#### 4. Memory Not Persisting
**Problem**: Memories lost after restart
**Solution**:
- Check memory configuration in cipher.yml
- Verify vector store settings
- Ensure proper shutdown (not forced kill)

## Performance Optimization

### Memory Management
- Limit: 512MB configured
- Monitor with: `mcp__cipher-aggregator__get_usage_stats`
- Prune old memories periodically

### Embedding Performance
- Local Ollama: ~100ms per embedding
- Batch operations when possible
- Cache frequently accessed memories

### Connection Pooling
- Single aggregator reduces connections
- Shared context across all tools
- Reduced overhead vs 8 separate servers

## Testing & Validation

### Verify Installation
```bash
# Check Cipher
which cipher

# Check Ollama
ollama --version
ollama list

# Test embedding
echo "test" | ollama embed nomic-embed-text
```

### Test Memory Operations
```javascript
// Store memory
mcp__cipher-aggregator__cipher_extract_and_operate_memory({
  interaction: "Test memory storage",
  memoryMetadata: {projectId: "test"}
})

// Search memory
mcp__cipher-aggregator__cipher_memory_search({
  query: "test memory"
})
```

### Validate All Tools
```bash
# List all available tools
/mcp

# Should show 106 tools from cipher-aggregator
```

## Integration Points

### With Task Master
- Task context stored in memory
- Progress tracking
- Decision traces

### With Claude Code
- Single MCP connection
- Restart required after config changes
- Enhanced with memory capabilities

### With Ruler (Future)
- Will share configuration patterns
- Unified AI enhancement strategy
- Complementary systems

## Best Practices

### Configuration Management
1. Always backup cipher.yml before changes
2. Test changes in isolation
3. Document custom configurations
4. Version control cipher.yml

### Memory Hygiene
1. Regular pruning of old memories
2. Categorize memories properly
3. Avoid storing sensitive data
4. Monitor memory usage

### Performance Monitoring
1. Track embedding times
2. Monitor memory growth
3. Check tool response times
4. Review aggregation efficiency

## Related Files
- `.cipher/cipher.yml` - Main configuration
- `.mcp.json` - MCP server setup
- `/ref/MCP_SERVERS.md` - Individual server details
- `/ref/MEMORY.md` - Memory system architecture
- `CLAUDE.md` - Integration instructions