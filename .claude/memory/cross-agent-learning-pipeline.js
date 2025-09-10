/**
 * Cross-Agent Learning Pipeline
 * Enables automatic pattern propagation, knowledge evolution tracking, 
 * and collective intelligence growth measurement across Tyler's agent network.
 * 
 * This system transforms isolated agent expertise into evolving collective intelligence.
 */

const LEARNING_CONFIG = {
    pattern_propagation: {
        success_threshold: 0.85,           // Propagate patterns with >85% success rate
        usage_threshold: 3,                // Minimum 3 successful uses before propagation
        propagation_delay_hours: 24,       // Wait 24 hours before cross-agent sharing
        max_propagation_agents: 10         // Limit propagation to prevent network flooding
    },
    intelligence_measurement: {
        measurement_interval_hours: 168,   // Weekly intelligence assessment
        baseline_establishment_days: 30,   // 30-day baseline before growth tracking
        growth_metrics: [
            'pattern_discovery_rate',
            'cross_agent_collaboration_success', 
            'problem_resolution_speed',
            'knowledge_reuse_efficiency',
            'collective_pattern_quality'
        ]
    },
    knowledge_evolution: {
        pattern_lifecycle_tracking: true,
        evolution_branch_limit: 5,         // Track up to 5 evolution branches per pattern
        deprecation_threshold: 0.3,        // Patterns below 30% success become deprecated
        archive_threshold_days: 180        // Archive unused patterns after 6 months
    }
};

/**
 * AUTOMATIC PATTERN PROPAGATION SYSTEM
 * Identifies successful patterns and propagates them to relevant agents
 */

/**
 * Monitor pattern usage across agents and identify candidates for propagation
 * @param {string} timeWindow - Time window for analysis ('24h', '7d', '30d')
 * @returns {Array} Patterns ready for propagation with metadata
 */
async function identifyPropagationCandidates(timeWindow = '7d') {
    console.log(`🔍 Analyzing pattern usage over ${timeWindow} for propagation candidates...`);
    
    try {
        // Search for patterns with high success metrics
        const allPatterns = await mcp__cipher_memory__search_nodes({
            query: "solution_pattern OR best_practice OR validated_approach"
        });
        
        const candidates = [];
        
        for (const pattern of allPatterns) {
            // Parse pattern metadata for success metrics
            const metadata = extractPatternMetadata(pattern);
            
            if (metadata && 
                metadata.success_rate >= LEARNING_CONFIG.pattern_propagation.success_threshold &&
                metadata.usage_count >= LEARNING_CONFIG.pattern_propagation.usage_threshold &&
                metadata.age_hours >= LEARNING_CONFIG.pattern_propagation.propagation_delay_hours) {
                
                // Identify potential recipient agents based on domain overlap
                const recipients = await identifyPotentialRecipients(pattern, metadata);
                
                if (recipients.length > 0) {
                    candidates.push({
                        pattern_name: pattern.name,
                        source_agent: metadata.source_agent,
                        domain: metadata.domain,
                        success_rate: metadata.success_rate,
                        usage_count: metadata.usage_count,
                        potential_recipients: recipients,
                        propagation_priority: calculatePropagationPriority(metadata, recipients)
                    });
                }
            }
        }
        
        // Sort by propagation priority (highest first)
        candidates.sort((a, b) => b.propagation_priority - a.propagation_priority);
        
        console.log(`✅ Identified ${candidates.length} patterns ready for propagation`);
        return candidates.slice(0, 20); // Limit to top 20 to prevent overload
        
    } catch (error) {
        console.error('❌ Error identifying propagation candidates:', error);
        return [];
    }
}

/**
 * Execute pattern propagation to recipient agents
 * @param {Array} propagationCandidates - Patterns ready for propagation
 * @returns {Object} Propagation results with success/failure tracking
 */
async function executePropagation(propagationCandidates) {
    console.log(`🚀 Executing propagation for ${propagationCandidates.length} patterns...`);
    
    const results = {
        successful_propagations: 0,
        failed_propagations: 0,
        total_recipient_agents: 0,
        propagation_details: []
    };
    
    for (const candidate of propagationCandidates) {
        try {
            // Load full pattern data
            const patternData = await mcp__cipher_memory__open_nodes({
                names: [candidate.pattern_name]
            });
            
            if (!patternData || patternData.length === 0) {
                console.warn(`⚠️  Pattern not found: ${candidate.pattern_name}`);
                results.failed_propagations++;
                continue;
            }
            
            const pattern = patternData[0];
            
            // Propagate to each recipient agent
            for (const recipient of candidate.potential_recipients) {
                try {
                    await propagateToAgent(pattern, candidate.source_agent, recipient);
                    results.successful_propagations++;
                    results.total_recipient_agents++;
                    
                    results.propagation_details.push({
                        pattern: candidate.pattern_name,
                        from: candidate.source_agent,
                        to: recipient.agent_id,
                        timestamp: new Date().toISOString(),
                        success: true,
                        priority: candidate.propagation_priority
                    });
                    
                } catch (propagationError) {
                    console.error(`❌ Failed to propagate ${candidate.pattern_name} to ${recipient.agent_id}:`, propagationError);
                    results.failed_propagations++;
                    
                    results.propagation_details.push({
                        pattern: candidate.pattern_name,
                        from: candidate.source_agent,
                        to: recipient.agent_id,
                        timestamp: new Date().toISOString(),
                        success: false,
                        error: propagationError.message
                    });
                }
            }
            
        } catch (patternError) {
            console.error(`❌ Error processing pattern ${candidate.pattern_name}:`, patternError);
            results.failed_propagations++;
        }
    }
    
    console.log(`✅ Propagation complete: ${results.successful_propagations} successful, ${results.failed_propagations} failed`);
    
    // Store propagation results for analytics
    await storePropagationResults(results);
    
    return results;
}

/**
 * Propagate a specific pattern to a target agent
 * @param {Object} pattern - Full pattern data
 * @param {string} sourceAgent - Agent that created the pattern
 * @param {Object} recipient - Target agent information
 */
async function propagateToAgent(pattern, sourceAgent, recipient) {
    // Create propagated pattern entity with clear attribution
    const propagatedPatternName = `${recipient.agent_id}_learned_${pattern.name.replace(`${sourceAgent}_`, '')}`;
    
    await mcp__cipher_memory__create_entities([{
        name: propagatedPatternName,
        entityType: "learned_pattern",
        observations: [
            ...pattern.observations,
            `LEARNED_FROM: ${sourceAgent} via Cross-Agent Learning Pipeline`,
            `PROPAGATION_DATE: ${new Date().toISOString()}`,
            `ADAPTATION_STATUS: Pending agent-specific validation`,
            `ORIGINAL_SUCCESS_RATE: ${extractPatternMetadata(pattern).success_rate}`,
            `LEARNING_PIPELINE_VERSION: 1.0`
        ]
    }]);
    
    // Create learning relationship
    await mcp__cipher_memory__create_relations([{
        from: propagatedPatternName,
        to: pattern.name,
        relationType: "learned_from"
    }]);
    
    // Create cross-agent knowledge transfer relationship
    await mcp__cipher_memory__create_relations([{
        from: recipient.agent_id,
        to: sourceAgent,
        relationType: "learned_from"
    }]);
    
    console.log(`📚 Pattern ${pattern.name} propagated to ${recipient.agent_id} as ${propagatedPatternName}`);
}

/**
 * COLLECTIVE INTELLIGENCE MEASUREMENT SYSTEM
 * Tracks how the agent network's collective intelligence evolves over time
 */

/**
 * Measure current collective intelligence metrics
 * @returns {Object} Comprehensive intelligence metrics
 */
async function measureCollectiveIntelligence() {
    console.log(`🧠 Measuring collective intelligence across agent network...`);
    
    try {
        const metrics = {
            timestamp: new Date().toISOString(),
            network_size: await getActiveAgentCount(),
            pattern_discovery_rate: await calculatePatternDiscoveryRate(),
            cross_agent_collaboration_success: await calculateCollaborationSuccess(),
            problem_resolution_speed: await calculateResolutionSpeed(),
            knowledge_reuse_efficiency: await calculateKnowledgeReuseEfficiency(),
            collective_pattern_quality: await calculateCollectivePatternQuality(),
            learning_velocity: await calculateLearningVelocity(),
            intelligence_distribution: await calculateIntelligenceDistribution()
        };
        
        // Store measurement for historical tracking
        await mcp__cipher_memory__create_entities([{
            name: `collective_intelligence_measurement_${Date.now()}`,
            entityType: "intelligence_metric",
            observations: [
                `MEASUREMENT_TIMESTAMP: ${metrics.timestamp}`,
                `NETWORK_SIZE: ${metrics.network_size} active agents`,
                `PATTERN_DISCOVERY_RATE: ${metrics.pattern_discovery_rate} patterns/day`,
                `COLLABORATION_SUCCESS: ${(metrics.cross_agent_collaboration_success * 100).toFixed(1)}%`,
                `RESOLUTION_SPEED: ${metrics.problem_resolution_speed.toFixed(2)} avg hours`,
                `KNOWLEDGE_REUSE: ${(metrics.knowledge_reuse_efficiency * 100).toFixed(1)}%`,
                `PATTERN_QUALITY: ${metrics.collective_pattern_quality.toFixed(2)}/10 avg score`,
                `LEARNING_VELOCITY: ${metrics.learning_velocity.toFixed(2)} new concepts/week`,
                `INTELLIGENCE_DISTRIBUTION: ${JSON.stringify(metrics.intelligence_distribution)}`
            ]
        }]);
        
        console.log(`✅ Collective intelligence measured and stored`);
        return metrics;
        
    } catch (error) {
        console.error('❌ Error measuring collective intelligence:', error);
        return null;
    }
}

/**
 * Calculate intelligence growth over time by comparing with historical data
 * @param {Object} currentMetrics - Current intelligence measurement
 * @returns {Object} Growth analysis with trends and predictions
 */
async function calculateIntelligenceGrowth(currentMetrics) {
    console.log(`📈 Calculating intelligence growth trends...`);
    
    try {
        // Retrieve historical measurements
        const historicalData = await mcp__cipher_memory__search_nodes({
            query: "collective_intelligence_measurement_*"
        });
        
        if (historicalData.length < 2) {
            console.log(`ℹ️  Insufficient historical data for growth calculation (need 2+, have ${historicalData.length})`);
            return {
                growth_available: false,
                reason: "Insufficient historical data",
                recommendation: "Continue collecting measurements for growth analysis"
            };
        }
        
        // Parse and sort historical measurements
        const measurements = historicalData
            .map(parseIntelligenceMeasurement)
            .filter(m => m !== null)
            .sort((a, b) => new Date(a.timestamp) - new Date(b.timestamp));
        
        const previousMeasurement = measurements[measurements.length - 1];
        const growth = {};
        
        // Calculate growth for each metric
        for (const metric of LEARNING_CONFIG.intelligence_measurement.growth_metrics) {
            if (currentMetrics[metric] !== undefined && previousMeasurement[metric] !== undefined) {
                const current = parseFloat(currentMetrics[metric]);
                const previous = parseFloat(previousMeasurement[metric]);
                const change = current - previous;
                const percentChange = previous !== 0 ? (change / previous) * 100 : 0;
                
                growth[metric] = {
                    current: current,
                    previous: previous,
                    absolute_change: change,
                    percent_change: percentChange,
                    trend: change > 0 ? 'improving' : change < 0 ? 'declining' : 'stable'
                };
            }
        }
        
        // Calculate overall intelligence growth score
        const overallGrowth = calculateOverallGrowthScore(growth);
        
        // Store growth analysis
        await mcp__cipher_memory__create_entities([{
            name: `intelligence_growth_analysis_${Date.now()}`,
            entityType: "growth_analysis",
            observations: [
                `ANALYSIS_TIMESTAMP: ${currentMetrics.timestamp}`,
                `OVERALL_GROWTH_SCORE: ${overallGrowth.toFixed(3)}`,
                `GROWTH_DETAILS: ${JSON.stringify(growth, null, 2)}`,
                `BASELINE_PERIOD: ${measurements.length} measurements`,
                `INTELLIGENCE_TRAJECTORY: ${overallGrowth > 0 ? 'Positive' : overallGrowth < 0 ? 'Negative' : 'Stable'}`
            ]
        }]);
        
        console.log(`✅ Intelligence growth calculated: ${overallGrowth > 0 ? '📈 Improving' : overallGrowth < 0 ? '📉 Declining' : '➡️ Stable'}`);
        
        return {
            growth_available: true,
            overall_score: overallGrowth,
            metric_details: growth,
            trend_direction: overallGrowth > 0 ? 'improving' : overallGrowth < 0 ? 'declining' : 'stable',
            measurement_count: measurements.length
        };
        
    } catch (error) {
        console.error('❌ Error calculating intelligence growth:', error);
        return {
            growth_available: false,
            reason: `Error: ${error.message}`,
            recommendation: "Check measurement data integrity"
        };
    }
}

/**
 * KNOWLEDGE EVOLUTION TRACKING SYSTEM
 * Tracks how patterns evolve, improve, and adapt across agents
 */

/**
 * Track the evolution of a specific pattern across the agent network
 * @param {string} patternName - Name of pattern to track
 * @returns {Object} Evolution tree with branches and adaptations
 */
async function trackPatternEvolution(patternName) {
    console.log(`🌱 Tracking evolution of pattern: ${patternName}`);
    
    try {
        // Find the original pattern and all its derivatives
        const relatedPatterns = await mcp__cipher_memory__search_nodes({
            query: `${patternName} OR learned_from:${patternName} OR extends:${patternName}`
        });
        
        // Build evolution tree
        const evolutionTree = {
            root_pattern: patternName,
            evolution_branches: [],
            total_adaptations: 0,
            active_branches: 0,
            deprecated_branches: 0,
            success_distribution: {}
        };
        
        for (const pattern of relatedPatterns) {
            const metadata = extractPatternMetadata(pattern);
            const relationships = await getPatternRelationships(pattern.name);
            
            if (isEvolutionBranch(pattern.name, patternName, relationships)) {
                const branch = {
                    branch_name: pattern.name,
                    source_agent: metadata.source_agent,
                    adaptation_type: determineAdaptationType(pattern, relationships),
                    success_rate: metadata.success_rate || 0,
                    usage_count: metadata.usage_count || 0,
                    age_days: calculatePatternAge(metadata.creation_date),
                    status: determinePatternStatus(metadata),
                    adaptations: []
                };
                
                // Find further adaptations of this branch
                branch.adaptations = await findPatternAdaptations(pattern.name);
                
                evolutionTree.evolution_branches.push(branch);
                evolutionTree.total_adaptations += 1 + branch.adaptations.length;
                
                if (branch.status === 'active') evolutionTree.active_branches++;
                if (branch.status === 'deprecated') evolutionTree.deprecated_branches++;
                
                evolutionTree.success_distribution[branch.source_agent] = 
                    (evolutionTree.success_distribution[branch.source_agent] || 0) + branch.success_rate;
            }
        }
        
        // Store evolution tracking data
        await mcp__cipher_memory__create_entities([{
            name: `pattern_evolution_${patternName}_${Date.now()}`,
            entityType: "evolution_tracking",
            observations: [
                `TRACKING_TIMESTAMP: ${new Date().toISOString()}`,
                `ROOT_PATTERN: ${patternName}`,
                `TOTAL_BRANCHES: ${evolutionTree.evolution_branches.length}`,
                `TOTAL_ADAPTATIONS: ${evolutionTree.total_adaptations}`,
                `ACTIVE_BRANCHES: ${evolutionTree.active_branches}`,
                `DEPRECATED_BRANCHES: ${evolutionTree.deprecated_branches}`,
                `SUCCESS_DISTRIBUTION: ${JSON.stringify(evolutionTree.success_distribution)}`,
                `EVOLUTION_TREE: ${JSON.stringify(evolutionTree, null, 2)}`
            ]
        }]);
        
        console.log(`✅ Pattern evolution tracked: ${evolutionTree.total_adaptations} total adaptations across ${evolutionTree.evolution_branches.length} branches`);
        return evolutionTree;
        
    } catch (error) {
        console.error(`❌ Error tracking pattern evolution for ${patternName}:`, error);
        return null;
    }
}

/**
 * LEARNING PIPELINE ORCHESTRATION
 * Main orchestration function that runs the complete learning pipeline
 */

/**
 * Execute the complete cross-agent learning pipeline
 * @param {Object} options - Pipeline execution options
 * @returns {Object} Comprehensive pipeline execution results
 */
async function executeLearningPipeline(options = {}) {
    const startTime = Date.now();
    console.log(`🚀 Executing Cross-Agent Learning Pipeline...`);
    
    const results = {
        execution_id: `pipeline_${startTime}`,
        start_time: new Date().toISOString(),
        stages: {},
        overall_success: false,
        intelligence_growth: null,
        recommendations: []
    };
    
    try {
        // Stage 1: Pattern Propagation
        console.log(`\n📡 Stage 1: Pattern Propagation`);
        const propagationCandidates = await identifyPropagationCandidates(options.timeWindow || '7d');
        const propagationResults = await executePropagation(propagationCandidates);
        
        results.stages.pattern_propagation = {
            success: true,
            candidates_identified: propagationCandidates.length,
            successful_propagations: propagationResults.successful_propagations,
            failed_propagations: propagationResults.failed_propagations,
            agents_reached: propagationResults.total_recipient_agents
        };
        
        // Stage 2: Intelligence Measurement
        console.log(`\n🧠 Stage 2: Collective Intelligence Measurement`);
        const currentIntelligence = await measureCollectiveIntelligence();
        
        if (currentIntelligence) {
            results.stages.intelligence_measurement = {
                success: true,
                network_size: currentIntelligence.network_size,
                pattern_discovery_rate: currentIntelligence.pattern_discovery_rate,
                collaboration_success: currentIntelligence.cross_agent_collaboration_success
            };
            
            // Stage 3: Growth Calculation
            console.log(`\n📈 Stage 3: Intelligence Growth Analysis`);
            const growthAnalysis = await calculateIntelligenceGrowth(currentIntelligence);
            results.intelligence_growth = growthAnalysis;
            
            results.stages.growth_analysis = {
                success: growthAnalysis.growth_available,
                overall_trend: growthAnalysis.trend_direction || 'unknown',
                measurements_available: growthAnalysis.measurement_count || 0
            };
        } else {
            results.stages.intelligence_measurement = { success: false, error: "Measurement failed" };
            results.stages.growth_analysis = { success: false, error: "No current metrics available" };
        }
        
        // Stage 4: Evolution Tracking (sample of top patterns)
        console.log(`\n🌱 Stage 4: Pattern Evolution Tracking`);
        const topPatterns = propagationCandidates.slice(0, 3); // Track evolution of top 3 patterns
        const evolutionResults = [];
        
        for (const candidate of topPatterns) {
            const evolution = await trackPatternEvolution(candidate.pattern_name);
            if (evolution) evolutionResults.push(evolution);
        }
        
        results.stages.evolution_tracking = {
            success: true,
            patterns_tracked: topPatterns.length,
            total_adaptations: evolutionResults.reduce((sum, e) => sum + e.total_adaptations, 0),
            active_branches: evolutionResults.reduce((sum, e) => sum + e.active_branches, 0)
        };
        
        // Generate recommendations based on pipeline results
        results.recommendations = generatePipelineRecommendations(results);
        
        // Mark overall success
        results.overall_success = Object.values(results.stages).every(stage => stage.success);
        results.end_time = new Date().toISOString();
        results.duration_ms = Date.now() - startTime;
        
        // Store complete pipeline execution results
        await mcp__cipher_memory__create_entities([{
            name: `learning_pipeline_execution_${startTime}`,
            entityType: "pipeline_execution",
            observations: [
                `EXECUTION_ID: ${results.execution_id}`,
                `START_TIME: ${results.start_time}`,
                `END_TIME: ${results.end_time}`,
                `DURATION_MS: ${results.duration_ms}`,
                `OVERALL_SUCCESS: ${results.overall_success}`,
                `STAGE_RESULTS: ${JSON.stringify(results.stages, null, 2)}`,
                `INTELLIGENCE_GROWTH: ${JSON.stringify(results.intelligence_growth, null, 2)}`,
                `RECOMMENDATIONS: ${JSON.stringify(results.recommendations, null, 2)}`,
                `PIPELINE_VERSION: 1.0`
            ]
        }]);
        
        console.log(`\n✅ Cross-Agent Learning Pipeline completed in ${(results.duration_ms / 1000).toFixed(2)}s`);
        console.log(`📊 Results: ${results.overall_success ? '✅ Success' : '❌ Partial failure'}`);
        
        return results;
        
    } catch (error) {
        console.error('❌ Learning pipeline execution failed:', error);
        results.overall_success = false;
        results.error = error.message;
        results.end_time = new Date().toISOString();
        results.duration_ms = Date.now() - startTime;
        return results;
    }
}

/**
 * UTILITY FUNCTIONS
 */

function extractPatternMetadata(pattern) {
    // Extract metadata from pattern observations
    try {
        const metadata = {};
        for (const obs of pattern.observations || []) {
            if (obs.includes('METADATA:')) {
                const metadataStr = obs.split('METADATA:')[1].trim();
                Object.assign(metadata, JSON.parse(metadataStr));
                break;
            }
        }
        return metadata;
    } catch (error) {
        return null;
    }
}

async function identifyPotentialRecipients(pattern, metadata) {
    // Identify agents that could benefit from this pattern
    const recipients = [];
    
    // Search for agents working in similar domains
    const relatedWork = await mcp__cipher_memory__search_nodes({
        query: `${metadata.domain} OR ${metadata.pattern_type}`
    });
    
    const agentSet = new Set();
    for (const work of relatedWork) {
        const workMetadata = extractPatternMetadata(work);
        if (workMetadata && workMetadata.source_agent && workMetadata.source_agent !== metadata.source_agent) {
            agentSet.add(workMetadata.source_agent);
        }
    }
    
    // Convert to recipient objects with relevance scoring
    for (const agentId of agentSet) {
        recipients.push({
            agent_id: agentId,
            relevance_score: calculateDomainRelevance(agentId, metadata.domain, metadata.pattern_type),
            domain_overlap: await calculateDomainOverlap(agentId, metadata.source_agent)
        });
    }
    
    return recipients
        .filter(r => r.relevance_score > 0.3) // Only include relevant agents
        .sort((a, b) => b.relevance_score - a.relevance_score)
        .slice(0, LEARNING_CONFIG.pattern_propagation.max_propagation_agents);
}

function calculatePropagationPriority(metadata, recipients) {
    // Calculate priority based on success rate, usage, and recipient relevance
    const successWeight = metadata.success_rate || 0;
    const usageWeight = Math.min(metadata.usage_count / 10, 1); // Normalize to 0-1
    const recipientWeight = recipients.reduce((sum, r) => sum + r.relevance_score, 0) / recipients.length;
    
    return (successWeight * 0.4) + (usageWeight * 0.3) + (recipientWeight * 0.3);
}

async function getActiveAgentCount() {
    // Count unique agents that have been active in the last 30 days
    const recentActivity = await mcp__cipher_memory__search_nodes({
        query: "solution_pattern OR best_practice OR decision"
    });
    
    const agentSet = new Set();
    for (const activity of recentActivity) {
        const metadata = extractPatternMetadata(activity);
        if (metadata && metadata.source_agent) {
            agentSet.add(metadata.source_agent);
        }
    }
    
    return agentSet.size;
}

async function calculatePatternDiscoveryRate() {
    // Calculate patterns discovered per day over the last 30 days
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    
    const recentPatterns = await mcp__cipher_memory__search_nodes({
        query: "solution_pattern OR best_practice"
    });
    
    // Filter patterns created in the last 30 days
    let recentCount = 0;
    for (const pattern of recentPatterns) {
        const metadata = extractPatternMetadata(pattern);
        if (metadata && metadata.creation_date) {
            const creationDate = new Date(metadata.creation_date);
            if (creationDate >= thirtyDaysAgo) {
                recentCount++;
            }
        }
    }
    
    return recentCount / 30; // Patterns per day
}

async function calculateCollaborationSuccess() {
    // Calculate success rate of cross-agent collaborations
    const collaborations = await mcp__cipher_memory__search_nodes({
        query: "collaboration OR learned_from OR extends"
    });
    
    if (collaborations.length === 0) return 0;
    
    let successfulCollaborations = 0;
    for (const collab of collaborations) {
        const metadata = extractPatternMetadata(collab);
        if (metadata && (metadata.success_rate || 0) > 0.7) {
            successfulCollaborations++;
        }
    }
    
    return successfulCollaborations / collaborations.length;
}

async function calculateResolutionSpeed() {
    // Calculate average problem resolution time in hours
    const problems = await mcp__cipher_memory__search_nodes({
        query: "solution OR resolution OR fixed"
    });
    
    let totalHours = 0;
    let validMeasurements = 0;
    
    for (const problem of problems) {
        const metadata = extractPatternMetadata(problem);
        if (metadata && metadata.resolution_time_hours) {
            totalHours += metadata.resolution_time_hours;
            validMeasurements++;
        }
    }
    
    return validMeasurements > 0 ? totalHours / validMeasurements : 24; // Default to 24 hours if no data
}

async function calculateKnowledgeReuseEfficiency() {
    // Calculate what percentage of new problems use existing patterns
    const recentProblems = await mcp__cipher_memory__search_nodes({
        query: "solution_pattern OR decision"
    });
    
    if (recentProblems.length === 0) return 0;
    
    let reusedPatterns = 0;
    for (const problem of recentProblems) {
        // Check if this problem references existing patterns
        const relationships = await getPatternRelationships(problem.name);
        if (relationships.some(r => r.relationType === 'uses' || r.relationType === 'extends')) {
            reusedPatterns++;
        }
    }
    
    return reusedPatterns / recentProblems.length;
}

async function calculateCollectivePatternQuality() {
    // Calculate average quality score of all patterns (based on success rate and validation)
    const allPatterns = await mcp__cipher_memory__search_nodes({
        query: "solution_pattern OR best_practice"
    });
    
    if (allPatterns.length === 0) return 5; // Default neutral score
    
    let totalQuality = 0;
    let validPatterns = 0;
    
    for (const pattern of allPatterns) {
        const metadata = extractPatternMetadata(pattern);
        if (metadata) {
            let qualityScore = 5; // Base score
            
            // Adjust based on success rate
            if (metadata.success_rate) qualityScore += (metadata.success_rate - 0.5) * 6;
            
            // Adjust based on validation status
            if (metadata.validation_status === 'production_proven') qualityScore += 2;
            if (metadata.validation_status === 'tested') qualityScore += 1;
            
            // Adjust based on usage count (popular patterns are likely higher quality)
            if (metadata.usage_count > 5) qualityScore += 1;
            if (metadata.usage_count > 10) qualityScore += 1;
            
            // Clamp to 0-10 range
            qualityScore = Math.max(0, Math.min(10, qualityScore));
            
            totalQuality += qualityScore;
            validPatterns++;
        }
    }
    
    return validPatterns > 0 ? totalQuality / validPatterns : 5;
}

async function calculateLearningVelocity() {
    // Calculate how many new concepts/patterns are learned per week
    const sevenDaysAgo = new Date();
    sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);
    
    const recentLearning = await mcp__cipher_memory__search_nodes({
        query: "learned_pattern OR new_concept OR discovered"
    });
    
    let recentCount = 0;
    for (const learning of recentLearning) {
        const metadata = extractPatternMetadata(learning);
        if (metadata && metadata.creation_date) {
            const creationDate = new Date(metadata.creation_date);
            if (creationDate >= sevenDaysAgo) {
                recentCount++;
            }
        }
    }
    
    return recentCount; // New concepts per week
}

async function calculateIntelligenceDistribution() {
    // Calculate how intelligence/expertise is distributed across agents
    const allPatterns = await mcp__cipher_memory__search_nodes({
        query: "solution_pattern OR best_practice OR expertise"
    });
    
    const agentContributions = {};
    
    for (const pattern of allPatterns) {
        const metadata = extractPatternMetadata(pattern);
        if (metadata && metadata.source_agent) {
            const agent = metadata.source_agent;
            if (!agentContributions[agent]) {
                agentContributions[agent] = { count: 0, totalQuality: 0 };
            }
            
            agentContributions[agent].count++;
            agentContributions[agent].totalQuality += (metadata.success_rate || 0.5) * 10;
        }
    }
    
    // Calculate contribution percentage and average quality per agent
    const totalContributions = Object.values(agentContributions).reduce((sum, a) => sum + a.count, 0);
    const distribution = {};
    
    for (const [agent, data] of Object.entries(agentContributions)) {
        distribution[agent] = {
            contribution_percentage: totalContributions > 0 ? (data.count / totalContributions) * 100 : 0,
            average_quality: data.count > 0 ? data.totalQuality / data.count : 5,
            total_patterns: data.count
        };
    }
    
    return distribution;
}

function parseIntelligenceMeasurement(measurementNode) {
    // Parse intelligence measurement from stored node
    try {
        const measurement = { timestamp: null };
        
        for (const obs of measurementNode.observations || []) {
            if (obs.includes('MEASUREMENT_TIMESTAMP:')) {
                measurement.timestamp = obs.split('MEASUREMENT_TIMESTAMP:')[1].trim();
            } else if (obs.includes('PATTERN_DISCOVERY_RATE:')) {
                measurement.pattern_discovery_rate = parseFloat(obs.split('PATTERN_DISCOVERY_RATE:')[1].split(' ')[0]);
            } else if (obs.includes('COLLABORATION_SUCCESS:')) {
                measurement.cross_agent_collaboration_success = parseFloat(obs.split('COLLABORATION_SUCCESS:')[1].replace('%', '')) / 100;
            } else if (obs.includes('RESOLUTION_SPEED:')) {
                measurement.problem_resolution_speed = parseFloat(obs.split('RESOLUTION_SPEED:')[1].split(' ')[0]);
            } else if (obs.includes('KNOWLEDGE_REUSE:')) {
                measurement.knowledge_reuse_efficiency = parseFloat(obs.split('KNOWLEDGE_REUSE:')[1].replace('%', '')) / 100;
            } else if (obs.includes('PATTERN_QUALITY:')) {
                measurement.collective_pattern_quality = parseFloat(obs.split('PATTERN_QUALITY:')[1].split('/')[0]);
            }
        }
        
        return measurement.timestamp ? measurement : null;
    } catch (error) {
        return null;
    }
}

function calculateOverallGrowthScore(growthMetrics) {
    // Calculate weighted overall growth score from individual metric growth
    const weights = {
        pattern_discovery_rate: 0.2,
        cross_agent_collaboration_success: 0.25,
        problem_resolution_speed: 0.2, // Note: lower is better, so we invert
        knowledge_reuse_efficiency: 0.15,
        collective_pattern_quality: 0.2
    };
    
    let weightedSum = 0;
    let totalWeight = 0;
    
    for (const [metric, growth] of Object.entries(growthMetrics)) {
        if (weights[metric] && growth.percent_change !== undefined) {
            let change = growth.percent_change;
            
            // Invert resolution speed (lower is better)
            if (metric === 'problem_resolution_speed') {
                change = -change;
            }
            
            weightedSum += change * weights[metric];
            totalWeight += weights[metric];
        }
    }
    
    return totalWeight > 0 ? weightedSum / totalWeight : 0;
}

async function getPatternRelationships(patternName) {
    // Get all relationships for a pattern (simplified - would use cipher memory relationships in real implementation)
    try {
        const relationData = await mcp__cipher_memory__search_nodes({
            query: `relationship:${patternName}`
        });
        
        return relationData.map(r => ({
            from: r.from || patternName,
            to: r.to,
            relationType: r.relationType || 'related_to'
        }));
    } catch (error) {
        return [];
    }
}

function isEvolutionBranch(candidateName, rootPatternName, relationships) {
    // Determine if a pattern is an evolution branch of another pattern
    return candidateName.includes(rootPatternName) || 
           candidateName.includes('learned_') ||
           relationships.some(r => r.relationType === 'learned_from' || r.relationType === 'extends');
}

function determineAdaptationType(pattern, relationships) {
    // Determine what type of adaptation this represents
    if (pattern.name.includes('learned_')) return 'cross_agent_learning';
    if (relationships.some(r => r.relationType === 'extends')) return 'enhancement';
    if (pattern.observations.some(o => o.includes('OPTIMIZED'))) return 'optimization';
    if (pattern.observations.some(o => o.includes('SPECIALIZED'))) return 'specialization';
    return 'adaptation';
}

function calculatePatternAge(creationDate) {
    // Calculate age of pattern in days
    if (!creationDate) return 0;
    
    const created = new Date(creationDate);
    const now = new Date();
    const diffTime = Math.abs(now - created);
    return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
}

function determinePatternStatus(metadata) {
    // Determine current status of a pattern based on metadata
    if (metadata.success_rate < LEARNING_CONFIG.knowledge_evolution.deprecation_threshold) {
        return 'deprecated';
    }
    if (metadata.usage_count > 3 && metadata.success_rate > 0.7) {
        return 'active';
    }
    if (metadata.age_days > LEARNING_CONFIG.knowledge_evolution.archive_threshold_days && metadata.usage_count === 0) {
        return 'archived';
    }
    return 'developing';
}

async function findPatternAdaptations(patternName) {
    // Find patterns that adapted from this pattern
    const adaptations = await mcp__cipher_memory__search_nodes({
        query: `learned_from:${patternName} OR extends:${patternName}`
    });
    
    return adaptations.map(a => ({
        name: a.name,
        adaptation_type: determineAdaptationType(a, []),
        source_agent: extractPatternMetadata(a)?.source_agent || 'unknown'
    }));
}

function calculateDomainRelevance(agentId, domain, patternType) {
    // Calculate how relevant a domain/pattern is for an agent (simplified heuristic)
    // In real implementation, this would analyze agent's historical work patterns
    
    const agentDomainMap = {
        'serial-comm-specialist': ['transport', 'hardware', 'communication'],
        'rust-async-specialist': ['async', 'concurrency', 'performance'],
        'standards-stan': ['quality', 'testing', 'validation'],
        'transport-lifecycle-guardian': ['cleanup', 'lifecycle', 'memory'],
        'task-orchestrator': ['coordination', 'planning', 'workflow']
    };
    
    const agentDomains = agentDomainMap[agentId] || [];
    
    if (agentDomains.includes(domain)) return 0.9;
    if (agentDomains.some(d => domain.includes(d) || d.includes(domain))) return 0.6;
    
    return 0.2; // Baseline relevance for cross-domain learning
}

async function calculateDomainOverlap(agent1, agent2) {
    // Calculate domain overlap between two agents
    const agent1Work = await mcp__cipher_memory__search_nodes({
        query: `source_agent:${agent1}`
    });
    
    const agent2Work = await mcp__cipher_memory__search_nodes({
        query: `source_agent:${agent2}`
    });
    
    const agent1Domains = new Set();
    const agent2Domains = new Set();
    
    for (const work of agent1Work) {
        const metadata = extractPatternMetadata(work);
        if (metadata && metadata.domain) agent1Domains.add(metadata.domain);
    }
    
    for (const work of agent2Work) {
        const metadata = extractPatternMetadata(work);
        if (metadata && metadata.domain) agent2Domains.add(metadata.domain);
    }
    
    const intersection = new Set([...agent1Domains].filter(d => agent2Domains.has(d)));
    const union = new Set([...agent1Domains, ...agent2Domains]);
    
    return union.size > 0 ? intersection.size / union.size : 0;
}

async function storePropagationResults(results) {
    // Store propagation results for analytics
    await mcp__cipher_memory__create_entities([{
        name: `propagation_results_${Date.now()}`,
        entityType: "propagation_analytics",
        observations: [
            `PROPAGATION_TIMESTAMP: ${new Date().toISOString()}`,
            `SUCCESSFUL_PROPAGATIONS: ${results.successful_propagations}`,
            `FAILED_PROPAGATIONS: ${results.failed_propagations}`,
            `AGENTS_REACHED: ${results.total_recipient_agents}`,
            `PROPAGATION_DETAILS: ${JSON.stringify(results.propagation_details, null, 2)}`,
            `SUCCESS_RATE: ${(results.successful_propagations / (results.successful_propagations + results.failed_propagations) * 100).toFixed(1)}%`
        ]
    }]);
}

function generatePipelineRecommendations(pipelineResults) {
    // Generate actionable recommendations based on pipeline execution results
    const recommendations = [];
    
    // Pattern propagation recommendations
    if (pipelineResults.stages.pattern_propagation) {
        const prop = pipelineResults.stages.pattern_propagation;
        if (prop.failed_propagations > prop.successful_propagations) {
            recommendations.push({
                type: 'pattern_propagation',
                priority: 'high',
                issue: 'High propagation failure rate',
                recommendation: 'Review propagation criteria and agent compatibility',
                action: 'Audit failed propagations and adjust relevance scoring'
            });
        }
        
        if (prop.candidates_identified === 0) {
            recommendations.push({
                type: 'pattern_propagation',
                priority: 'medium',
                issue: 'No propagation candidates found',
                recommendation: 'Agents may be working in isolation or success metrics need adjustment',
                action: 'Lower success thresholds or increase collaboration incentives'
            });
        }
    }
    
    // Intelligence growth recommendations
    if (pipelineResults.intelligence_growth) {
        const growth = pipelineResults.intelligence_growth;
        if (growth.growth_available && growth.trend_direction === 'declining') {
            recommendations.push({
                type: 'intelligence_growth',
                priority: 'high',
                issue: 'Declining collective intelligence',
                recommendation: 'Investigate bottlenecks in knowledge sharing and collaboration',
                action: 'Focus on improving cross-agent learning mechanisms'
            });
        }
        
        if (!growth.growth_available) {
            recommendations.push({
                type: 'intelligence_growth',
                priority: 'low',
                issue: 'Insufficient historical data for growth analysis',
                recommendation: 'Continue running pipeline to establish baseline',
                action: 'Schedule regular pipeline executions to build measurement history'
            });
        }
    }
    
    // Evolution tracking recommendations
    if (pipelineResults.stages.evolution_tracking) {
        const evo = pipelineResults.stages.evolution_tracking;
        if (evo.active_branches === 0) {
            recommendations.push({
                type: 'evolution_tracking',
                priority: 'medium',
                issue: 'No active pattern evolution detected',
                recommendation: 'Patterns may not be adapting or evolving across agents',
                action: 'Encourage pattern experimentation and cross-agent adaptation'
            });
        }
    }
    
    return recommendations;
}

/**
 * EXPORTED API FUNCTIONS
 */

module.exports = {
    // Main pipeline execution
    executeLearningPipeline,
    
    // Pattern propagation
    identifyPropagationCandidates,
    executePropagation,
    
    // Intelligence measurement
    measureCollectiveIntelligence,
    calculateIntelligenceGrowth,
    
    // Evolution tracking
    trackPatternEvolution,
    
    // Configuration
    LEARNING_CONFIG
};

/**
 * USAGE EXAMPLES:
 * 
 * // Run complete learning pipeline
 * const results = await executeLearningPipeline({ timeWindow: '7d' });
 * 
 * // Measure current intelligence
 * const intelligence = await measureCollectiveIntelligence();
 * 
 * // Track specific pattern evolution
 * const evolution = await trackPatternEvolution('serial-comm-specialist_transport_cleanup_arc_lifecycle');
 * 
 * // Find propagation candidates
 * const candidates = await identifyPropagationCandidates('24h');
 */