// Universal Agent Pattern Lifecycle Workflows v1.0
// Pattern Evolution and Quality Management for Tyler's Collective Intelligence Network
//
// This module manages the complete pattern lifecycle:
// Draft → Testing → Approved → Standard (via Standards Stan)
// Includes peer review, quality validation, and excellence enforcement

const { enrichPattern, createPatternRelationship, validateWithStan } = require('./agent-memory-interface')

/**
 * Pattern Lifecycle States
 * - draft: New pattern, untested
 * - testing: Pattern being validated through usage
 * - approved: Pattern proven effective, ready for sharing
 * - standard: Standards Stan has validated pattern as excellence standard
 * - deprecated: Pattern superseded by better approach
 */
const LIFECYCLE_STATES = {
  DRAFT: 'draft',
  TESTING: 'testing', 
  APPROVED: 'approved',
  STANDARD: 'standard',
  DEPRECATED: 'deprecated'
}

/**
 * Advance pattern through lifecycle states with validation
 * @param {string} entityName - Pattern entity name
 * @param {string} newState - Target lifecycle state
 * @param {Object} validationData - Evidence supporting state change
 * @param {string} validatingAgent - Agent requesting state change
 * @returns {Promise<Object>} Lifecycle advancement result
 */
async function advancePatternLifecycle(entityName, newState, validationData, validatingAgent) {
  // Validate state transition
  if (!Object.values(LIFECYCLE_STATES).includes(newState)) {
    throw new Error(`Invalid lifecycle state: ${newState}`)
  }
  
  // Get current pattern state
  const currentPattern = await mcp__cipher_memory__open_nodes({
    names: [entityName]
  })
  
  if (!currentPattern || currentPattern.length === 0) {
    throw new Error(`Pattern not found: ${entityName}`)
  }
  
  const pattern = currentPattern[0]
  const currentState = extractCurrentState(pattern)
  
  // Validate state transition rules
  const validTransition = validateStateTransition(currentState, newState)
  if (!validTransition.valid) {
    throw new Error(`Invalid state transition: ${currentState} → ${newState}. ${validTransition.reason}`)
  }
  
  // Apply state-specific validation requirements
  await validateStateRequirements(newState, validationData, pattern)
  
  // Update pattern with new state and validation evidence
  const stateChangeObservations = [
    `LIFECYCLE_CHANGE: ${currentState} → ${newState}`,
    `VALIDATED_BY: ${validatingAgent}`,
    `VALIDATION_EVIDENCE: ${JSON.stringify(validationData)}`,
    `STATE_CHANGED_AT: ${new Date().toISOString()}`
  ]
  
  await mcp__cipher_memory__add_observations([{
    entityName,
    contents: stateChangeObservations
  }])
  
  // Update metadata with new state
  await enrichPattern(entityName, {
    metadataUpdate: {
      validation_status: newState,
      last_state_change: new Date().toISOString(),
      validating_agent: validatingAgent
    }
  }, validatingAgent)
  
  // Handle state-specific actions
  await handleStateSpecificActions(entityName, newState, validationData, validatingAgent)
  
  // Log lifecycle event
  await logLifecycleEvent('state_advanced', {
    entityName,
    fromState: currentState,
    toState: newState,
    validatingAgent,
    validationData
  })
  
  return {
    success: true,
    previousState: currentState,
    newState: newState,
    validatingAgent,
    message: `Pattern ${entityName} advanced from ${currentState} to ${newState}`
  }
}

/**
 * Submit pattern for Standards Stan excellence review
 * @param {string} entityName - Pattern entity name
 * @param {string} submittingAgent - Agent submitting for review
 * @param {Object} submissionContext - Context for excellence review
 * @returns {Promise<Object>} Excellence review submission result
 */
async function submitForExcellenceReview(entityName, submittingAgent, submissionContext = {}) {
  // Verify pattern is in approved state
  const pattern = await mcp__cipher_memory__open_nodes({
    names: [entityName]
  })
  
  if (!pattern || pattern.length === 0) {
    throw new Error(`Pattern not found: ${entityName}`)
  }
  
  const currentState = extractCurrentState(pattern[0])
  if (currentState !== LIFECYCLE_STATES.APPROVED) {
    throw new Error(`Pattern must be in 'approved' state for excellence review. Current state: ${currentState}`)
  }
  
  // Create excellence review request
  const reviewRequest = {
    entityName,
    submittingAgent,
    reviewType: 'excellence_standards_validation',
    submissionContext,
    criteria: [
      'zero_tolerance_quality',
      'craftsmanship_standards', 
      'completeness_validation',
      'robustness_assessment',
      'excellence_propagation_potential'
    ],
    timestamp: new Date().toISOString()
  }
  
  // Submit to Standards Stan via validation system
  const validationResult = await validateWithStan(
    entityName, 
    'excellence_audit',
    reviewRequest
  )
  
  // Create excellence review tracking
  const reviewTrackingName = `excellence_review_${entityName}_${Date.now()}`
  await mcp__cipher_memory__create_entities([{
    name: reviewTrackingName,
    entityType: 'excellence_review',
    observations: [
      `EXCELLENCE_REVIEW: Submitted for Standards Stan validation`,
      `PATTERN: ${entityName}`,
      `SUBMITTING_AGENT: ${submittingAgent}`,
      `CRITERIA: ${reviewRequest.criteria.join(', ')}`,
      `CONTEXT: ${JSON.stringify(submissionContext)}`,
      `STATUS: pending_standards_review`,
      `SUBMITTED_AT: ${new Date().toISOString()}`
    ]
  }])
  
  // Link review to pattern
  await createPatternRelationship(
    reviewTrackingName,
    entityName, 
    'validates_against',
    'Standards Stan excellence review'
  )
  
  // Log excellence review submission
  await logLifecycleEvent('excellence_review_submitted', {
    entityName,
    submittingAgent,
    reviewTrackingName,
    criteria: reviewRequest.criteria
  })
  
  return {
    success: true,
    reviewSubmitted: true,
    reviewTrackingName,
    validationResult,
    message: `Excellence review submitted to Standards Stan for ${entityName}`
  }
}

/**
 * Handle peer review process for pattern validation
 * @param {string} entityName - Pattern entity name  
 * @param {string} reviewingAgent - Agent conducting peer review
 * @param {Object} reviewData - Review findings and assessment
 * @returns {Promise<Object>} Peer review result
 */
async function conductPeerReview(entityName, reviewingAgent, reviewData) {
  // Validate review data structure
  if (!reviewData.assessment || !reviewData.evidence) {
    throw new Error('Peer review must include assessment and evidence')
  }
  
  const {
    assessment, // 'approved', 'needs_improvement', 'rejected'
    evidence,   // Evidence supporting the assessment
    suggestions = [], // Improvement suggestions
    confidence = 0.8, // Reviewer confidence in assessment
    domains_validated = [] // Domains the reviewer can validate
  } = reviewData
  
  // Create peer review entity
  const reviewEntityName = `peer_review_${entityName}_${reviewingAgent}_${Date.now()}`
  await mcp__cipher_memory__create_entities([{
    name: reviewEntityName,
    entityType: 'peer_review',
    observations: [
      `PEER_REVIEW: ${assessment}`,
      `REVIEWING_AGENT: ${reviewingAgent}`,
      `PATTERN_REVIEWED: ${entityName}`,
      `EVIDENCE: ${evidence}`,
      `CONFIDENCE: ${confidence}`,
      `DOMAINS_VALIDATED: ${domains_validated.join(', ')}`,
      ...(suggestions.length > 0 ? [`SUGGESTIONS: ${suggestions.join('; ')}`] : []),
      `REVIEWED_AT: ${new Date().toISOString()}`
    ]
  }])
  
  // Link review to pattern
  await createPatternRelationship(
    reviewEntityName,
    entityName,
    'peer_reviewed_by',
    `Peer review by ${reviewingAgent}: ${assessment}`
  )
  
  // Update pattern with peer review results
  await enrichPattern(entityName, {
    usageUpdate: `Peer reviewed by ${reviewingAgent}: ${assessment}`,
    metadataUpdate: {
      peer_validations: [reviewingAgent], // This would append to existing array
      peer_review_count: 1, // This would increment existing count
      last_peer_review: new Date().toISOString()
    }
  }, reviewingAgent)
  
  // If approved by peer, consider lifecycle advancement
  if (assessment === 'approved' && confidence >= 0.8) {
    await considerLifecycleAdvancement(entityName, reviewingAgent, {
      trigger: 'peer_review_approved',
      evidence: evidence,
      reviewer: reviewingAgent,
      confidence: confidence
    })
  }
  
  // Log peer review completion
  await logLifecycleEvent('peer_review_completed', {
    entityName,
    reviewingAgent,
    assessment,
    confidence,
    reviewEntityName
  })
  
  return {
    success: true,
    reviewCompleted: true,
    assessment,
    reviewEntityName,
    confidenceScore: confidence,
    message: `Peer review completed by ${reviewingAgent}: ${assessment}`
  }
}

/**
 * Process pattern usage feedback and update lifecycle accordingly
 * @param {string} entityName - Pattern entity name
 * @param {string} usingAgent - Agent using the pattern
 * @param {Object} usageFeedback - Usage results and feedback
 * @returns {Promise<Object>} Usage feedback processing result
 */
async function processUsageFeedback(entityName, usingAgent, usageFeedback) {
  const {
    outcome, // 'success', 'partial_success', 'failure'
    effectiveness = 0.5, // 0.0 - 1.0 effectiveness score
    performance_impact = 'unknown', // 'positive', 'negative', 'neutral', 'unknown'
    adaptations_made = [], // Changes made to apply pattern
    lessons_learned = '', // Insights gained from usage
    would_recommend = true, // Whether agent would recommend pattern
    usage_context = '' // Context where pattern was applied
  } = usageFeedback
  
  // Create usage feedback entity
  const feedbackEntityName = `usage_feedback_${entityName}_${usingAgent}_${Date.now()}`
  await mcp__cipher_memory__create_entities([{
    name: feedbackEntityName,
    entityType: 'usage_feedback',
    observations: [
      `USAGE_FEEDBACK: ${outcome}`,
      `USING_AGENT: ${usingAgent}`,
      `PATTERN_USED: ${entityName}`,
      `EFFECTIVENESS: ${effectiveness}`,
      `PERFORMANCE_IMPACT: ${performance_impact}`,
      `RECOMMENDATION: ${would_recommend ? 'recommended' : 'not_recommended'}`,
      `CONTEXT: ${usage_context}`,
      ...(adaptations_made.length > 0 ? [`ADAPTATIONS: ${adaptations_made.join('; ')}`] : []),
      ...(lessons_learned ? [`LESSONS: ${lessons_learned}`] : []),
      `USED_AT: ${new Date().toISOString()}`
    ]
  }])
  
  // Link feedback to pattern
  await createPatternRelationship(
    feedbackEntityName,
    entityName,
    'provides_feedback_for',
    `Usage feedback: ${outcome} (effectiveness: ${effectiveness})`
  )
  
  // Update pattern with usage statistics
  await enrichPattern(entityName, {
    usageUpdate: `Used by ${usingAgent}: ${outcome} (effectiveness: ${effectiveness})`,
    performanceUpdate: `Performance impact: ${performance_impact}`,
    metadataUpdate: {
      usage_count: 1, // This would increment existing count
      success_rate: effectiveness, // This would be calculated as running average
      last_used: new Date().toISOString(),
      last_using_agent: usingAgent
    }
  }, usingAgent)
  
  // Aggregate usage feedback for pattern quality assessment
  await updatePatternQualityMetrics(entityName, usageFeedback)
  
  // Consider lifecycle changes based on usage patterns
  if (outcome === 'success' && effectiveness >= 0.8) {
    await considerLifecycleAdvancement(entityName, usingAgent, {
      trigger: 'successful_usage',
      evidence: `Successful usage with ${effectiveness} effectiveness`,
      usage_context: usage_context
    })
  }
  
  // Log usage feedback
  await logLifecycleEvent('usage_feedback_processed', {
    entityName,
    usingAgent,
    outcome,
    effectiveness,
    feedbackEntityName
  })
  
  return {
    success: true,
    feedbackProcessed: true,
    feedbackEntityName,
    patternQualityImpact: calculateQualityImpact(usageFeedback),
    message: `Usage feedback processed for ${entityName}: ${outcome}`
  }
}

/**
 * Deprecate pattern when superseded by better approach
 * @param {string} entityName - Pattern entity name to deprecate
 * @param {string} replacementPattern - New pattern that replaces this one (optional)
 * @param {string} deprecatingAgent - Agent initiating deprecation
 * @param {string} reason - Reason for deprecation
 * @returns {Promise<Object>} Deprecation result
 */
async function deprecatePattern(entityName, replacementPattern, deprecatingAgent, reason) {
  // Advance pattern to deprecated state
  await advancePatternLifecycle(entityName, LIFECYCLE_STATES.DEPRECATED, {
    reason: reason,
    replacement_pattern: replacementPattern,
    deprecation_evidence: reason
  }, deprecatingAgent)
  
  // Create replacement relationship if specified
  if (replacementPattern) {
    await createPatternRelationship(
      replacementPattern,
      entityName,
      'replaces_pattern',
      `Replaces deprecated pattern: ${reason}`
    )
  }
  
  // Update pattern with deprecation notice
  await enrichPattern(entityName, {
    usageUpdate: `DEPRECATED: ${reason}`,
    metadataUpdate: {
      deprecated: true,
      deprecated_at: new Date().toISOString(),
      deprecated_by: deprecatingAgent,
      replacement_pattern: replacementPattern
    }
  }, deprecatingAgent)
  
  // Log deprecation
  await logLifecycleEvent('pattern_deprecated', {
    entityName,
    replacementPattern,
    deprecatingAgent,
    reason
  })
  
  return {
    success: true,
    patternDeprecated: true,
    replacementPattern,
    message: `Pattern ${entityName} deprecated: ${reason}`
  }
}

// ============================================================================
// HELPER FUNCTIONS  
// ============================================================================

/**
 * Extract current lifecycle state from pattern observations
 */
function extractCurrentState(pattern) {
  const observations = pattern.observations || []
  
  // Look for most recent lifecycle state
  let currentState = LIFECYCLE_STATES.DRAFT // Default to draft
  
  for (const obs of observations.reverse()) { // Check most recent first
    if (obs.includes('LIFECYCLE_CHANGE:')) {
      const stateMatch = obs.match(/LIFECYCLE_CHANGE: \w+ → (\w+)/)
      if (stateMatch) {
        currentState = stateMatch[1]
        break
      }
    }
  }
  
  // Check metadata for validation_status
  try {
    for (const obs of observations) {
      if (obs.includes('METADATA:')) {
        const metadataStr = obs.substring(obs.indexOf('{'))
        const metadata = JSON.parse(metadataStr)
        if (metadata.validation_status) {
          currentState = metadata.validation_status
          break
        }
      }
    }
  } catch (e) {
    // Ignore metadata parsing errors
  }
  
  return currentState
}

/**
 * Validate state transition rules
 */
function validateStateTransition(fromState, toState) {
  const validTransitions = {
    [LIFECYCLE_STATES.DRAFT]: [LIFECYCLE_STATES.TESTING, LIFECYCLE_STATES.DEPRECATED],
    [LIFECYCLE_STATES.TESTING]: [LIFECYCLE_STATES.APPROVED, LIFECYCLE_STATES.DRAFT, LIFECYCLE_STATES.DEPRECATED],
    [LIFECYCLE_STATES.APPROVED]: [LIFECYCLE_STATES.STANDARD, LIFECYCLE_STATES.TESTING, LIFECYCLE_STATES.DEPRECATED],
    [LIFECYCLE_STATES.STANDARD]: [LIFECYCLE_STATES.DEPRECATED],
    [LIFECYCLE_STATES.DEPRECATED]: [] // No transitions from deprecated
  }
  
  if (!validTransitions[fromState]?.includes(toState)) {
    return {
      valid: false,
      reason: `Invalid transition from ${fromState} to ${toState}. Valid transitions: ${validTransitions[fromState]?.join(', ') || 'none'}`
    }
  }
  
  return { valid: true }
}

/**
 * Validate requirements for entering specific states
 */
async function validateStateRequirements(newState, validationData, pattern) {
  switch (newState) {
    case LIFECYCLE_STATES.TESTING:
      if (!validationData.test_plan) {
        throw new Error('Testing state requires test_plan in validation data')
      }
      break
      
    case LIFECYCLE_STATES.APPROVED:
      if (!validationData.evidence || !validationData.success_metrics) {
        throw new Error('Approved state requires evidence and success_metrics')
      }
      if (validationData.success_rate && validationData.success_rate < 0.7) {
        throw new Error('Approved state requires success_rate >= 0.7')
      }
      break
      
    case LIFECYCLE_STATES.STANDARD:
      if (!validationData.standards_review_passed) {
        throw new Error('Standard state requires Standards Stan approval')
      }
      break
      
    case LIFECYCLE_STATES.DEPRECATED:
      if (!validationData.reason) {
        throw new Error('Deprecated state requires deprecation reason')
      }
      break
  }
}

/**
 * Handle actions specific to lifecycle states
 */
async function handleStateSpecificActions(entityName, newState, validationData, validatingAgent) {
  switch (newState) {
    case LIFECYCLE_STATES.APPROVED:
      // Broadcast pattern availability to other agents
      await broadcastPatternAvailability(entityName, validatingAgent)
      break
      
    case LIFECYCLE_STATES.STANDARD:
      // Propagate as excellence standard across agent network
      await propagateExcellenceStandard(entityName, validationData)
      break
      
    case LIFECYCLE_STATES.DEPRECATED:
      // Notify agents using this pattern about deprecation
      await notifyPatternDeprecation(entityName, validationData)
      break
  }
}

/**
 * Consider advancing pattern lifecycle based on usage/review
 */
async function considerLifecycleAdvancement(entityName, triggeringAgent, advancementData) {
  const pattern = await mcp__cipher_memory__open_nodes({
    names: [entityName]
  })
  
  if (!pattern || pattern.length === 0) return
  
  const currentState = extractCurrentState(pattern[0])
  
  // Logic for automatic advancement based on usage patterns
  if (currentState === LIFECYCLE_STATES.TESTING && 
      advancementData.trigger === 'successful_usage' &&
      advancementData.evidence) {
    
    // Could advance to approved if sufficient success evidence
    // This would need more sophisticated logic in real implementation
    console.log(`Considering advancement of ${entityName} from testing to approved based on usage`)
  }
}

/**
 * Update pattern quality metrics based on usage feedback
 */
async function updatePatternQualityMetrics(entityName, usageFeedback) {
  // This would integrate with the coordination registry for analytics
  // For now, log the quality metric update
  await logLifecycleEvent('quality_metrics_updated', {
    entityName,
    effectiveness: usageFeedback.effectiveness,
    outcome: usageFeedback.outcome,
    performance_impact: usageFeedback.performance_impact
  })
}

/**
 * Calculate quality impact of usage feedback
 */
function calculateQualityImpact(usageFeedback) {
  let impact = 0
  
  if (usageFeedback.outcome === 'success') impact += 0.3
  if (usageFeedback.effectiveness >= 0.8) impact += 0.3
  if (usageFeedback.would_recommend) impact += 0.2
  if (usageFeedback.performance_impact === 'positive') impact += 0.2
  
  return Math.min(impact, 1.0)
}

/**
 * Broadcast pattern availability to agent ecosystem
 */
async function broadcastPatternAvailability(entityName, validatingAgent) {
  // This would integrate with communication protocols
  console.log(`Broadcasting pattern availability: ${entityName} validated by ${validatingAgent}`)
  
  // TODO: Implement actual broadcast to coordination system
}

/**
 * Propagate pattern as excellence standard
 */
async function propagateExcellenceStandard(entityName, validationData) {
  // This would integrate with Standards Stan and coordination registry
  console.log(`Propagating excellence standard: ${entityName}`)
  
  // TODO: Implement excellence standard propagation
}

/**
 * Notify agents about pattern deprecation
 */
async function notifyPatternDeprecation(entityName, validationData) {
  // This would notify agents that have used this pattern
  console.log(`Notifying pattern deprecation: ${entityName}`)
  
  // TODO: Implement deprecation notifications
}

/**
 * Log lifecycle events for analytics
 */
async function logLifecycleEvent(eventType, eventData) {
  console.log(`PATTERN_LIFECYCLE_EVENT: ${eventType}`, eventData)
  
  // TODO: Implement proper logging to .claude/logs/pattern-lifecycle-events.jsonl
}

// ============================================================================
// MODULE EXPORTS
// ============================================================================

module.exports = {
  LIFECYCLE_STATES,
  advancePatternLifecycle,
  submitForExcellenceReview,
  conductPeerReview,
  processUsageFeedback,
  deprecatePattern
}