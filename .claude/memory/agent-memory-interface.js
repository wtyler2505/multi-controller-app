// Universal Agent Memory Interface v1.0
// Core API for Tyler's Collective Intelligence Network
// 
// This module provides schema-enforced agent memory operations that enable:
// - Standardized pattern storage with automatic metadata validation
// - Cross-agent pattern discovery with intelligent search
// - Quality validation and peer review integration
// - Seamless integration with existing cipher memory infrastructure

/**
 * Store agent pattern with automatic schema enforcement and metadata validation
 * @param {string} agentId - Agent identifier (e.g., 'serial-comm-specialist')
 * @param {string} domain - Primary domain (e.g., 'transport', 'async', 'quality')  
 * @param {string} patternType - Pattern category (e.g., 'solution_pattern', 'quality_gate')
 * @param {string} specificName - Unique pattern identifier
 * @param {Object} patternData - Pattern content and metadata
 * @returns {Promise<Object>} Storage result with generated entity name
 */
async function storeAgentPattern(agentId, domain, patternType, specificName, patternData) {
  // Enforce Universal Agent Memory Schema naming convention
  const entityName = `${agentId}_${domain}_${patternType}_${specificName}`
  
  // Validate required pattern data
  if (!patternData.description) {
    throw new Error('Pattern description is required')
  }
  if (!patternData.context) {
    throw new Error('Pattern context is required')
  }
  
  // Generate standardized metadata
  const metadata = generatePatternMetadata(agentId, domain, patternData)
  
  // Create schema-compliant observations
  const observations = [
    `PATTERN: ${patternData.description}`,
    `CONTEXT: ${patternData.context}`,
    ...(patternData.implementation ? [`IMPLEMENTATION: ${patternData.implementation}`] : []),
    ...(patternData.validation ? [`VALIDATION: ${patternData.validation}`] : []),
    ...(patternData.performance ? [`PERFORMANCE: ${patternData.performance}`] : []),
    `METADATA: ${JSON.stringify(metadata)}`
  ]
  
  // Store using cipher memory with schema enforcement
  const result = await mcp__cipher_memory__create_entities([{
    name: entityName,
    entityType: patternType,
    observations: observations
  }])
  
  // Update agent performance metrics
  await updateAgentMetrics(agentId, 'pattern_contribution', entityName)
  
  // Log pattern creation for analytics
  await logPatternEvent('pattern_created', {
    agentId,
    entityName,
    domain,
    patternType,
    metadata
  })
  
  return {
    success: true,
    entityName,
    metadata,
    storageResult: result
  }
}

/**
 * Discover patterns across all agents with intelligent search
 * @param {Object} searchCriteria - Search parameters
 * @param {string[]} searchCriteria.domains - Domains to search (optional)
 * @param {string[]} searchCriteria.agents - Specific agents to search (optional)  
 * @param {string} searchCriteria.problemType - Problem/pattern type (optional)
 * @param {string} searchCriteria.complexity - Required complexity level (optional)
 * @param {number} searchCriteria.minConfidence - Minimum confidence score (optional)
 * @param {number} searchCriteria.maxResults - Maximum results to return (default: 15)
 * @returns {Promise<Object[]>} Discovered patterns with relevance scoring
 */
async function discoverPatterns(searchCriteria) {
  const {
    domains = [],
    agents = [],
    problemType = '',
    complexity = '',
    minConfidence = 0.0,
    maxResults = 15
  } = searchCriteria
  
  // Build intelligent search queries
  const searchQueries = buildSearchQueries(searchCriteria)
  const discoveredPatterns = []
  
  // Execute multiple search strategies
  for (const query of searchQueries) {
    try {
      const searchResults = await mcp__cipher_memory__search_nodes({
        query: query.queryString
      })
      
      // Process and score results
      for (const result of searchResults) {
        const pattern = await parsePatternResult(result)
        const relevanceScore = calculateRelevanceScore(pattern, searchCriteria)
        
        if (relevanceScore >= 0.3) { // Minimum relevance threshold
          discoveredPatterns.push({
            ...pattern,
            relevanceScore,
            searchQuery: query.description
          })
        }
      }
    } catch (error) {
      console.warn(`Search query failed: ${query.description}`, error)
    }
  }
  
  // Remove duplicates and sort by relevance
  const uniquePatterns = deduplicatePatterns(discoveredPatterns)
  const sortedPatterns = uniquePatterns
    .sort((a, b) => b.relevanceScore - a.relevanceScore)
    .slice(0, maxResults)
  
  // Log pattern discovery for analytics
  await logPatternEvent('patterns_discovered', {
    searchCriteria,
    resultsCount: sortedPatterns.length,
    topRelevanceScore: sortedPatterns[0]?.relevanceScore || 0
  })
  
  return sortedPatterns
}

/**
 * Request expertise from specific agent or domain
 * @param {string} requestingAgent - Agent making the request
 * @param {string} targetAgent - Target agent or 'any' for best match
 * @param {string} expertiseDomain - Domain of expertise needed
 * @param {string} context - Context of the request
 * @param {string} urgency - Request urgency: 'low', 'medium', 'high', 'critical'
 * @returns {Promise<Object>} Expertise request result and recommendations
 */
async function requestExpertise(requestingAgent, targetAgent, expertiseDomain, context, urgency = 'medium') {
  // Find patterns from target agent or optimal agents for domain
  let searchQuery
  if (targetAgent === 'any') {
    searchQuery = `*_${expertiseDomain}_* OR ${expertiseDomain} patterns`
  } else {
    searchQuery = `${targetAgent}_${expertiseDomain}_* OR ${targetAgent}_*_${expertiseDomain}_*`
  }
  
  const expertisePatterns = await mcp__cipher_memory__search_nodes({
    query: searchQuery
  })
  
  // Process patterns and find best matches
  const processedPatterns = []
  for (const pattern of expertisePatterns) {
    const parsed = await parsePatternResult(pattern)
    const contextMatch = calculateContextMatch(parsed, context)
    
    if (contextMatch >= 0.4) {
      processedPatterns.push({
        ...parsed,
        contextMatch
      })
    }
  }
  
  // Sort by context match and quality
  const rankedPatterns = processedPatterns
    .sort((a, b) => (b.contextMatch * b.confidence) - (a.contextMatch * a.confidence))
    .slice(0, 5) // Top 5 most relevant patterns
  
  // Create expertise request in coordination system
  await sendCoordinationMessage(requestingAgent, targetAgent, {
    messageType: 'expertise_request',
    domain: expertiseDomain,
    context,
    urgency,
    suggestedPatterns: rankedPatterns.map(p => p.entityName)
  })
  
  // Log expertise request for collaboration analytics
  await logPatternEvent('expertise_requested', {
    requestingAgent,
    targetAgent,
    expertiseDomain,
    context,
    urgency,
    patternsFound: rankedPatterns.length
  })
  
  return {
    success: true,
    patternsFound: rankedPatterns.length,
    recommendedPatterns: rankedPatterns,
    requestSent: targetAgent !== 'any',
    message: `Found ${rankedPatterns.length} relevant patterns for ${expertiseDomain} expertise`
  }
}

/**
 * Validate pattern with Standards Stan and update quality status
 * @param {string} entityName - Pattern entity name to validate
 * @param {string} validationType - Type of validation: 'quality_review', 'excellence_audit', 'standards_compliance'
 * @param {Object} validationContext - Additional context for validation
 * @returns {Promise<Object>} Validation result and quality assessment
 */
async function validateWithStan(entityName, validationType = 'quality_review', validationContext = {}) {
  // Get pattern details for validation
  const patternDetails = await mcp__cipher_memory__open_nodes({
    names: [entityName]
  })
  
  if (!patternDetails || patternDetails.length === 0) {
    throw new Error(`Pattern not found: ${entityName}`)
  }
  
  const pattern = patternDetails[0]
  
  // Send validation request to Standards Stan
  const validationRequest = {
    messageType: 'validation_request',
    entityName,
    validationType,
    patternData: pattern,
    validationContext,
    timestamp: new Date().toISOString()
  }
  
  await sendCoordinationMessage('system', 'standards-stan', validationRequest)
  
  // Create validation tracking entity
  const validationTrackingName = `validation_${validationType}_${entityName}_${Date.now()}`
  await mcp__cipher_memory__create_entities([{
    name: validationTrackingName,
    entityType: 'validation_request',
    observations: [
      `VALIDATION_REQUEST: ${validationType} for ${entityName}`,
      `CONTEXT: ${JSON.stringify(validationContext)}`,
      `STATUS: pending_review`,
      `REQUESTED_AT: ${new Date().toISOString()}`
    ]
  }])
  
  // Create relationship between validation and original pattern
  await mcp__cipher_memory__create_relations([{
    from: validationTrackingName,
    to: entityName,
    relationType: 'validates_against'
  }])
  
  // Log validation request
  await logPatternEvent('validation_requested', {
    entityName,
    validationType,
    validationContext,
    trackingEntity: validationTrackingName
  })
  
  return {
    success: true,
    validationRequested: true,
    trackingEntity: validationTrackingName,
    message: `Validation request sent to Standards Stan for ${entityName}`
  }
}

/**
 * Create relationships between patterns (cross-agent pattern inheritance)
 * @param {string} fromPattern - Source pattern entity name
 * @param {string} toPattern - Target pattern entity name  
 * @param {string} relationshipType - Type of relationship
 * @param {string} description - Description of the relationship
 * @returns {Promise<Object>} Relationship creation result
 */
async function createPatternRelationship(fromPattern, toPattern, relationshipType, description = '') {
  // Validate relationship type against schema
  const validRelationshipTypes = [
    'extends_pattern', 'conflicts_with', 'requires_dependency', 'optimizes_for',
    'replaces_pattern', 'shares_context', 'validates_against', 'collaborates_with'
  ]
  
  if (!validRelationshipTypes.includes(relationshipType)) {
    throw new Error(`Invalid relationship type: ${relationshipType}. Must be one of: ${validRelationshipTypes.join(', ')}`)
  }
  
  // Verify both patterns exist
  const patterns = await mcp__cipher_memory__open_nodes({
    names: [fromPattern, toPattern]
  })
  
  if (patterns.length !== 2) {
    throw new Error(`One or both patterns not found: ${fromPattern}, ${toPattern}`)
  }
  
  // Create relationship
  const relationshipResult = await mcp__cipher_memory__create_relations([{
    from: fromPattern,
    to: toPattern,
    relationType: relationshipType
  }])
  
  // Log relationship creation
  if (description) {
    await mcp__cipher_memory__add_observations([{
      entityName: fromPattern,
      contents: [`RELATIONSHIP: ${relationshipType} with ${toPattern} - ${description}`]
    }])
  }
  
  // Update pattern analytics
  await logPatternEvent('relationship_created', {
    fromPattern,
    toPattern,
    relationshipType,
    description
  })
  
  return {
    success: true,
    relationshipCreated: true,
    relationshipResult
  }
}

/**
 * Enrich existing pattern with new observations and insights
 * @param {string} entityName - Pattern entity name to enrich
 * @param {Object} enrichmentData - New data to add to pattern
 * @param {string} enrichingAgent - Agent adding the enrichment
 * @returns {Promise<Object>} Enrichment result
 */
async function enrichPattern(entityName, enrichmentData, enrichingAgent) {
  const enrichments = []
  
  // Add structured enrichment observations
  if (enrichmentData.newInsight) {
    enrichments.push(`NEW_INSIGHT: ${enrichmentData.newInsight}`)
  }
  if (enrichmentData.validationUpdate) {
    enrichments.push(`VALIDATION_UPDATE: ${enrichmentData.validationUpdate}`)
  }
  if (enrichmentData.performanceUpdate) {
    enrichments.push(`PERFORMANCE_UPDATE: ${enrichmentData.performanceUpdate}`)
  }
  if (enrichmentData.usageUpdate) {
    enrichments.push(`USAGE_UPDATE: ${enrichmentData.usageUpdate}`)
  }
  
  // Add metadata update
  if (enrichmentData.metadataUpdate) {
    enrichments.push(`METADATA_UPDATE: ${JSON.stringify(enrichmentData.metadataUpdate)}`)
  }
  
  // Add enrichment timestamp and source
  enrichments.push(`ENRICHED_BY: ${enrichingAgent} at ${new Date().toISOString()}`)
  
  // Apply enrichment to pattern
  const enrichmentResult = await mcp__cipher_memory__add_observations([{
    entityName,
    contents: enrichments
  }])
  
  // Update pattern usage metrics
  await updatePatternUsageMetrics(entityName, enrichingAgent)
  
  // Log enrichment activity
  await logPatternEvent('pattern_enriched', {
    entityName,
    enrichingAgent,
    enrichmentTypes: Object.keys(enrichmentData),
    enrichmentCount: enrichments.length
  })
  
  return {
    success: true,
    enrichmentApplied: true,
    enrichmentCount: enrichments.length,
    enrichmentResult
  }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Generate standardized metadata for pattern storage
 */
function generatePatternMetadata(agentId, domain, patternData) {
  return {
    agent_source: agentId,
    domain: domain,
    complexity: patternData.complexity || 'moderate',
    confidence: patternData.confidence || 0.8,
    use_cases: patternData.useCases || [],
    performance_impact: patternData.performanceImpact || 'low',
    dependencies: patternData.dependencies || [],
    validation_status: 'untested',
    success_rate: 0.0,
    last_updated: new Date().toISOString().split('T')[0],
    usage_count: 0,
    tags: patternData.tags || []
  }
}

/**
 * Build intelligent search queries based on criteria
 */
function buildSearchQueries(searchCriteria) {
  const queries = []
  
  // Domain-based queries
  if (searchCriteria.domains?.length > 0) {
    for (const domain of searchCriteria.domains) {
      queries.push({
        queryString: `*_${domain}_* OR ${domain} patterns`,
        description: `Domain search: ${domain}`
      })
    }
  }
  
  // Agent-based queries
  if (searchCriteria.agents?.length > 0) {
    for (const agent of searchCriteria.agents) {
      queries.push({
        queryString: `${agent}_*`,
        description: `Agent patterns: ${agent}`
      })
    }
  }
  
  // Problem-type queries
  if (searchCriteria.problemType) {
    queries.push({
      queryString: `*_${searchCriteria.problemType}_* OR ${searchCriteria.problemType}`,
      description: `Problem type: ${searchCriteria.problemType}`
    })
  }
  
  // Fallback general query
  if (queries.length === 0) {
    queries.push({
      queryString: 'solution_pattern OR implementation_technique OR quality_gate',
      description: 'General pattern search'
    })
  }
  
  return queries
}

/**
 * Parse pattern result from cipher memory search
 */
async function parsePatternResult(result) {
  // Extract metadata from observations
  let metadata = {}
  const observations = result.observations || []
  
  for (const obs of observations) {
    if (obs.includes('METADATA:')) {
      try {
        const metadataStr = obs.substring(obs.indexOf('{'))
        metadata = JSON.parse(metadataStr)
      } catch (e) {
        console.warn('Failed to parse metadata:', obs)
      }
    }
  }
  
  return {
    entityName: result.name,
    entityType: result.entityType,
    observations: observations,
    metadata: metadata,
    confidence: metadata.confidence || 0.5,
    domain: metadata.domain || 'unknown',
    agentSource: metadata.agent_source || 'unknown'
  }
}

/**
 * Calculate relevance score for search results
 */
function calculateRelevanceScore(pattern, searchCriteria) {
  let score = 0
  const weights = {
    domain_match: 0.3,
    agent_match: 0.2,
    problem_match: 0.3,
    confidence: 0.2
  }
  
  // Domain matching
  if (searchCriteria.domains?.includes(pattern.domain)) {
    score += weights.domain_match
  }
  
  // Agent matching  
  if (searchCriteria.agents?.includes(pattern.agentSource)) {
    score += weights.agent_match
  }
  
  // Problem type matching
  if (searchCriteria.problemType && pattern.entityName.includes(searchCriteria.problemType)) {
    score += weights.problem_match
  }
  
  // Confidence contribution
  score += weights.confidence * (pattern.confidence || 0.5)
  
  return Math.min(score, 1.0)
}

/**
 * Remove duplicate patterns from search results
 */
function deduplicatePatterns(patterns) {
  const seen = new Set()
  return patterns.filter(pattern => {
    if (seen.has(pattern.entityName)) {
      return false
    }
    seen.add(pattern.entityName)
    return true
  })
}

/**
 * Calculate context match for expertise requests
 */
function calculateContextMatch(pattern, context) {
  // Simple keyword matching - could be enhanced with semantic analysis
  const patternText = pattern.observations.join(' ').toLowerCase()
  const contextWords = context.toLowerCase().split(/\s+/)
  
  let matches = 0
  for (const word of contextWords) {
    if (word.length > 3 && patternText.includes(word)) {
      matches++
    }
  }
  
  return Math.min(matches / Math.max(contextWords.length, 1), 1.0)
}

/**
 * Update agent performance metrics
 */
async function updateAgentMetrics(agentId, metricType, entityName) {
  // This would integrate with the Agent Coordination Registry
  // For now, log the metric update
  await logPatternEvent('agent_metric_update', {
    agentId,
    metricType,
    entityName,
    timestamp: new Date().toISOString()
  })
}

/**
 * Update pattern usage metrics
 */
async function updatePatternUsageMetrics(entityName, usingAgent) {
  // Track pattern usage for analytics
  await logPatternEvent('pattern_usage', {
    entityName,
    usingAgent,
    timestamp: new Date().toISOString()
  })
}

/**
 * Log pattern events for analytics and monitoring
 */
async function logPatternEvent(eventType, eventData) {
  // This would integrate with the Centralized Agent Logging system
  // For now, create a simple log structure
  console.log(`AGENT_MEMORY_EVENT: ${eventType}`, eventData)
  
  // TODO: Implement proper logging to .claude/logs/agent-memory-events.jsonl
}

/**
 * Send coordination message to other agents
 */
async function sendCoordinationMessage(fromAgent, toAgent, messageData) {
  // This would integrate with the Agent Communication Protocols
  // For now, create a placeholder implementation
  console.log(`COORDINATION_MESSAGE: ${fromAgent} -> ${toAgent}`, messageData)
  
  // TODO: Implement actual message sending using communication protocols
}

// ============================================================================
// MODULE EXPORTS
// ============================================================================

module.exports = {
  storeAgentPattern,
  discoverPatterns,
  requestExpertise,
  validateWithStan,
  createPatternRelationship,
  enrichPattern
}