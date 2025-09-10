/**
 * Centralized Agent Logging System
 * Comprehensive activity tracking, performance monitoring, and debugging support
 * for Tyler's Universal Agent Integration ecosystem.
 * 
 * Provides complete visibility into agent operations, collaboration patterns,
 * and system health with structured storage in cipher memory.
 */

const LOG_CONFIG = {
    log_levels: {
        DEBUG: 0,
        INFO: 1,
        WARN: 2,
        ERROR: 3,
        CRITICAL: 4
    },
    retention: {
        debug_logs_days: 7,        // Keep debug logs for 1 week
        info_logs_days: 30,        // Keep info logs for 1 month
        warn_logs_days: 90,        // Keep warning logs for 3 months
        error_logs_days: 365,      // Keep error logs for 1 year
        critical_logs_permanent: true // Never delete critical logs
    },
    storage: {
        batch_size: 50,            // Batch log entries for efficient storage
        flush_interval_ms: 30000,  // Flush logs every 30 seconds
        max_memory_entries: 1000,  // Maximum logs in memory before forced flush
        compression_enabled: true   // Compress large log payloads
    },
    monitoring: {
        performance_threshold_ms: 5000,    // Log operations taking >5s
        error_rate_alert_threshold: 0.1,   // Alert if error rate >10%
        collaboration_tracking: true,       // Track cross-agent interactions
        pattern_usage_tracking: true       // Track pattern application success
    }
};

/**
 * In-memory log buffer for batching
 */
let logBuffer = [];
let flushTimer = null;

/**
 * CORE LOGGING API
 * Primary interface for all agent logging operations
 */

/**
 * Log an agent operation with comprehensive context
 * @param {string} agentId - Agent performing the operation
 * @param {string} level - Log level (DEBUG, INFO, WARN, ERROR, CRITICAL)
 * @param {string} operation - Operation being performed
 * @param {Object} context - Detailed context and metadata
 */
async function logAgentOperation(agentId, level, operation, context = {}) {
    const timestamp = new Date().toISOString();
    const logEntry = {
        timestamp,
        agent_id: agentId,
        level: level.toUpperCase(),
        operation,
        context: {
            ...context,
            session_id: context.session_id || generateSessionId(),
            task_id: context.task_id,
            correlation_id: context.correlation_id || generateCorrelationId()
        },
        performance: {
            duration_ms: context.duration_ms,
            memory_usage_mb: context.memory_usage_mb,
            cpu_percent: context.cpu_percent
        },
        metadata: {
            version: '1.0',
            log_source: 'centralized-agent-logging',
            environment: 'tyler-multi-controller-app'
        }
    };
    
    // Add to buffer
    logBuffer.push(logEntry);
    
    // Console output for immediate visibility
    const levelPrefix = getLevelPrefix(level);
    console.log(`${levelPrefix} [${agentId}] ${operation}`, context.message || '');
    
    // Force flush if buffer is getting full or level is ERROR/CRITICAL
    if (logBuffer.length >= LOG_CONFIG.storage.max_memory_entries || 
        ['ERROR', 'CRITICAL'].includes(level.toUpperCase())) {
        await flushLogBuffer();
    } else if (!flushTimer) {
        // Schedule regular flush
        flushTimer = setTimeout(flushLogBuffer, LOG_CONFIG.storage.flush_interval_ms);
    }
    
    return logEntry;
}

/**
 * Log agent collaboration events
 * @param {string} requestingAgent - Agent initiating collaboration
 * @param {string} targetAgent - Agent being asked to collaborate
 * @param {string} collaborationType - Type of collaboration (pattern_request, expertise_request, etc.)
 * @param {Object} details - Collaboration details and outcome
 */
async function logAgentCollaboration(requestingAgent, targetAgent, collaborationType, details = {}) {
    return await logAgentOperation(requestingAgent, 'INFO', 'collaboration', {
        message: `Collaboration: ${collaborationType} with ${targetAgent}`,
        collaboration_type: collaborationType,
        target_agent: targetAgent,
        success: details.success,
        response_time_ms: details.response_time_ms,
        patterns_shared: details.patterns_shared || 0,
        insights_gained: details.insights_gained || [],
        follow_up_required: details.follow_up_required || false,
        collaboration_quality_score: details.quality_score || null
    });
}

/**
 * Log pattern usage and effectiveness
 * @param {string} agentId - Agent using the pattern
 * @param {string} patternName - Name of pattern being used
 * @param {Object} usage - Usage context and results
 */
async function logPatternUsage(agentId, patternName, usage = {}) {
    return await logAgentOperation(agentId, 'INFO', 'pattern_usage', {
        message: `Applied pattern: ${patternName}`,
        pattern_name: patternName,
        pattern_source: usage.pattern_source || 'unknown',
        application_context: usage.context || '',
        success: usage.success,
        effectiveness_score: usage.effectiveness_score,
        adaptations_made: usage.adaptations || [],
        time_saved_minutes: usage.time_saved_minutes,
        quality_improvement: usage.quality_improvement,
        would_reuse: usage.would_reuse !== false, // Default to true unless explicitly false
        pattern_evolution_candidate: usage.effectiveness_score > 0.8 // High-performing patterns are evolution candidates
    });
}

/**
 * Log task execution with complete lifecycle tracking
 * @param {string} agentId - Agent executing the task
 * @param {string} taskId - Task identifier
 * @param {string} phase - Task phase (started, in_progress, completed, failed)
 * @param {Object} taskData - Task execution data and metrics
 */
async function logTaskExecution(agentId, taskId, phase, taskData = {}) {
    const level = phase === 'failed' ? 'ERROR' : 'INFO';
    
    return await logAgentOperation(agentId, level, 'task_execution', {
        message: `Task ${taskId}: ${phase}`,
        task_id: taskId,
        task_phase: phase,
        task_title: taskData.title,
        task_complexity: taskData.complexity,
        estimated_duration_minutes: taskData.estimated_duration_minutes,
        actual_duration_minutes: taskData.actual_duration_minutes,
        patterns_used: taskData.patterns_used || [],
        collaborations_required: taskData.collaborations_required || [],
        tools_used: taskData.tools_used || [],
        quality_score: taskData.quality_score,
        standards_compliance: taskData.standards_compliance,
        stan_review_passed: taskData.stan_review_passed,
        lessons_learned: taskData.lessons_learned || [],
        blocked_by: taskData.blocked_by || [],
        next_actions: taskData.next_actions || []
    });
}

/**
 * Log errors with comprehensive debugging context
 * @param {string} agentId - Agent where error occurred
 * @param {Error|string} error - Error object or error message
 * @param {Object} context - Error context and debugging information
 */
async function logAgentError(agentId, error, context = {}) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    const errorStack = error instanceof Error ? error.stack : null;
    
    return await logAgentOperation(agentId, 'ERROR', 'error_occurred', {
        message: `Error: ${errorMessage}`,
        error_message: errorMessage,
        error_stack: errorStack,
        error_type: error.constructor?.name || 'Unknown',
        operation_being_performed: context.operation,
        task_context: context.task_id,
        patterns_in_use: context.patterns_in_use || [],
        system_state: context.system_state || {},
        recovery_action: context.recovery_action,
        user_impact: context.user_impact || 'unknown',
        reproducible: context.reproducible !== false, // Default to true unless explicitly false
        debugging_data: context.debugging_data || {}
    });
}

/**
 * Log system health and performance metrics
 * @param {string} agentId - Agent reporting health
 * @param {Object} healthMetrics - Comprehensive health and performance data
 */
async function logAgentHealth(agentId, healthMetrics = {}) {
    return await logAgentOperation(agentId, 'INFO', 'health_report', {
        message: `Health report for ${agentId}`,
        cpu_usage_percent: healthMetrics.cpu_usage_percent,
        memory_usage_mb: healthMetrics.memory_usage_mb,
        memory_peak_mb: healthMetrics.memory_peak_mb,
        operations_per_minute: healthMetrics.operations_per_minute,
        success_rate_percent: healthMetrics.success_rate_percent,
        average_response_time_ms: healthMetrics.average_response_time_ms,
        patterns_in_memory: healthMetrics.patterns_in_memory || 0,
        collaborations_active: healthMetrics.collaborations_active || 0,
        tasks_queued: healthMetrics.tasks_queued || 0,
        last_error_time: healthMetrics.last_error_time,
        uptime_minutes: healthMetrics.uptime_minutes,
        health_status: determineHealthStatus(healthMetrics)
    });
}

/**
 * SPECIALIZED LOGGING FUNCTIONS
 */

/**
 * Log Standards Stan review activities
 * @param {string} reviewedAgent - Agent being reviewed
 * @param {string} reviewType - Type of review (code_quality, excellence_standard, pattern_validation)
 * @param {Object} reviewResults - Review findings and scores
 */
async function logStandardsReview(reviewedAgent, reviewType, reviewResults = {}) {
    return await logAgentOperation('standards-stan', 'INFO', 'quality_review', {
        message: `${reviewType} review of ${reviewedAgent}`,
        reviewed_agent: reviewedAgent,
        review_type: reviewType,
        excellence_score: reviewResults.excellence_score,
        passed_standards: reviewResults.passed_standards !== false,
        critical_issues: reviewResults.critical_issues || [],
        recommendations: reviewResults.recommendations || [],
        follow_up_required: reviewResults.follow_up_required || false,
        enforcement_actions: reviewResults.enforcement_actions || [],
        zero_tolerance_violations: reviewResults.zero_tolerance_violations || [],
        review_duration_minutes: reviewResults.review_duration_minutes,
        historical_improvement: reviewResults.historical_improvement || null
    });
}

/**
 * Log task-orchestrator decisions and deployments
 * @param {Object} deploymentDecision - Orchestrator's agent deployment decision
 */
async function logOrchestratorDecision(deploymentDecision = {}) {
    return await logAgentOperation('task-orchestrator', 'INFO', 'deployment_decision', {
        message: `Deployed ${deploymentDecision.agents_deployed} agents for ${deploymentDecision.task_count} tasks`,
        task_count: deploymentDecision.task_count,
        agents_available: deploymentDecision.agents_available,
        agents_deployed: deploymentDecision.agents_deployed,
        deployment_strategy: deploymentDecision.strategy || 'optimal',
        parallel_execution: deploymentDecision.parallel_execution || false,
        expected_completion_time: deploymentDecision.expected_completion_time,
        resource_allocation: deploymentDecision.resource_allocation || {},
        priority_adjustments: deploymentDecision.priority_adjustments || [],
        blocked_tasks: deploymentDecision.blocked_tasks || [],
        orchestration_confidence: deploymentDecision.confidence_score || null
    });
}

/**
 * LOG ANALYTICS AND QUERYING
 */

/**
 * Query logs with flexible filtering
 * @param {Object} filters - Query filters
 * @returns {Array} Matching log entries
 */
async function queryLogs(filters = {}) {
    try {
        // Build search query based on filters
        let searchQuery = 'agent_operation OR collaboration OR pattern_usage OR task_execution';
        
        if (filters.agent_id) {
            searchQuery += ` AND agent_id:${filters.agent_id}`;
        }
        
        if (filters.level) {
            searchQuery += ` AND level:${filters.level.toUpperCase()}`;
        }
        
        if (filters.operation) {
            searchQuery += ` AND operation:${filters.operation}`;
        }
        
        if (filters.time_range) {
            // Add time range filtering (simplified - would need proper timestamp filtering)
            searchQuery += ` AND timestamp:${filters.time_range}`;
        }
        
        // Search cipher memory for log entries
        const logEntries = await mcp__cipher_memory__search_nodes({
            query: searchQuery
        });
        
        // Parse and filter results
        const parsedLogs = logEntries
            .map(parseLogEntry)
            .filter(log => log !== null)
            .sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp)); // Most recent first
        
        // Apply additional filters
        let filteredLogs = parsedLogs;
        
        if (filters.limit) {
            filteredLogs = filteredLogs.slice(0, filters.limit);
        }
        
        if (filters.task_id) {
            filteredLogs = filteredLogs.filter(log => log.context?.task_id === filters.task_id);
        }
        
        return filteredLogs;
        
    } catch (error) {
        console.error('❌ Error querying logs:', error);
        return [];
    }
}

/**
 * Generate agent performance analytics
 * @param {string} agentId - Agent to analyze (or 'all' for system-wide)
 * @param {string} timeWindow - Analysis time window ('24h', '7d', '30d')
 * @returns {Object} Comprehensive performance analytics
 */
async function generatePerformanceAnalytics(agentId = 'all', timeWindow = '7d') {
    console.log(`📊 Generating performance analytics for ${agentId} over ${timeWindow}...`);
    
    try {
        const filters = { time_range: timeWindow };
        if (agentId !== 'all') filters.agent_id = agentId;
        
        const logs = await queryLogs(filters);
        
        if (logs.length === 0) {
            return {
                agent_id: agentId,
                time_window: timeWindow,
                no_data: true,
                message: 'No log data available for analysis'
            };
        }
        
        const analytics = {
            agent_id: agentId,
            time_window: timeWindow,
            analysis_timestamp: new Date().toISOString(),
            
            // Operation metrics
            total_operations: logs.length,
            operations_by_type: {},
            operations_by_level: {},
            
            // Performance metrics
            average_response_time_ms: 0,
            operations_per_hour: 0,
            error_rate_percent: 0,
            
            // Collaboration metrics
            collaborations_initiated: 0,
            collaborations_received: 0,
            collaboration_success_rate: 0,
            
            // Pattern usage metrics
            patterns_used: 0,
            unique_patterns: new Set(),
            pattern_success_rate: 0,
            
            // Task execution metrics
            tasks_completed: 0,
            tasks_failed: 0,
            task_completion_rate: 0,
            average_task_duration_minutes: 0,
            
            // Quality metrics
            standards_reviews_passed: 0,
            standards_reviews_failed: 0,
            excellence_score_average: 0,
            
            // Health metrics
            health_reports: 0,
            average_cpu_usage: 0,
            average_memory_usage_mb: 0,
            
            // Trends and insights
            trends: {},
            insights: [],
            recommendations: []
        };
        
        // Process logs to generate analytics
        let totalResponseTime = 0;
        let responseTimeCount = 0;
        let totalTaskDuration = 0;
        let taskDurationCount = 0;
        let totalExcellenceScore = 0;
        let excellenceScoreCount = 0;
        let totalCpuUsage = 0;
        let cpuUsageCount = 0;
        let totalMemoryUsage = 0;
        let memoryUsageCount = 0;
        
        for (const log of logs) {
            // Operation type counting
            analytics.operations_by_type[log.operation] = 
                (analytics.operations_by_type[log.operation] || 0) + 1;
            
            // Level counting
            analytics.operations_by_level[log.level] = 
                (analytics.operations_by_level[log.level] || 0) + 1;
            
            // Response time aggregation
            if (log.context?.response_time_ms) {
                totalResponseTime += log.context.response_time_ms;
                responseTimeCount++;
            }
            
            // Collaboration metrics
            if (log.operation === 'collaboration') {
                analytics.collaborations_initiated++;
                if (log.context?.success) {
                    // Track successful collaborations for success rate
                }
            }
            
            // Pattern usage metrics
            if (log.operation === 'pattern_usage') {
                analytics.patterns_used++;
                if (log.context?.pattern_name) {
                    analytics.unique_patterns.add(log.context.pattern_name);
                }
            }
            
            // Task execution metrics
            if (log.operation === 'task_execution') {
                if (log.context?.task_phase === 'completed') {
                    analytics.tasks_completed++;
                }
                if (log.context?.task_phase === 'failed') {
                    analytics.tasks_failed++;
                }
                
                if (log.context?.actual_duration_minutes) {
                    totalTaskDuration += log.context.actual_duration_minutes;
                    taskDurationCount++;
                }
            }
            
            // Standards review metrics
            if (log.operation === 'quality_review' && log.agent_id === 'standards-stan') {
                if (log.context?.passed_standards) {
                    analytics.standards_reviews_passed++;
                } else {
                    analytics.standards_reviews_failed++;
                }
                
                if (log.context?.excellence_score) {
                    totalExcellenceScore += log.context.excellence_score;
                    excellenceScoreCount++;
                }
            }
            
            // Health metrics
            if (log.operation === 'health_report') {
                analytics.health_reports++;
                
                if (log.context?.cpu_usage_percent) {
                    totalCpuUsage += log.context.cpu_usage_percent;
                    cpuUsageCount++;
                }
                
                if (log.context?.memory_usage_mb) {
                    totalMemoryUsage += log.context.memory_usage_mb;
                    memoryUsageCount++;
                }
            }
        }
        
        // Calculate averages and rates
        analytics.average_response_time_ms = responseTimeCount > 0 ? totalResponseTime / responseTimeCount : 0;
        analytics.operations_per_hour = logs.length / (getTimeWindowHours(timeWindow) || 1);
        analytics.error_rate_percent = ((analytics.operations_by_level.ERROR || 0) + (analytics.operations_by_level.CRITICAL || 0)) / logs.length * 100;
        
        const totalTasks = analytics.tasks_completed + analytics.tasks_failed;
        analytics.task_completion_rate = totalTasks > 0 ? (analytics.tasks_completed / totalTasks) * 100 : 0;
        analytics.average_task_duration_minutes = taskDurationCount > 0 ? totalTaskDuration / taskDurationCount : 0;
        
        const totalReviews = analytics.standards_reviews_passed + analytics.standards_reviews_failed;
        analytics.standards_review_pass_rate = totalReviews > 0 ? (analytics.standards_reviews_passed / totalReviews) * 100 : 0;
        
        analytics.excellence_score_average = excellenceScoreCount > 0 ? totalExcellenceScore / excellenceScoreCount : 0;
        analytics.average_cpu_usage = cpuUsageCount > 0 ? totalCpuUsage / cpuUsageCount : 0;
        analytics.average_memory_usage_mb = memoryUsageCount > 0 ? totalMemoryUsage / memoryUsageCount : 0;
        
        analytics.unique_patterns = analytics.unique_patterns.size;
        
        // Generate insights and recommendations
        analytics.insights = generatePerformanceInsights(analytics);
        analytics.recommendations = generatePerformanceRecommendations(analytics);
        
        // Store analytics results
        await mcp__cipher_memory__create_entities([{
            name: `performance_analytics_${agentId}_${timeWindow}_${Date.now()}`,
            entityType: "performance_analytics",
            observations: [
                `ANALYSIS_TIMESTAMP: ${analytics.analysis_timestamp}`,
                `AGENT_ID: ${analytics.agent_id}`,
                `TIME_WINDOW: ${analytics.time_window}`,
                `TOTAL_OPERATIONS: ${analytics.total_operations}`,
                `ERROR_RATE: ${analytics.error_rate_percent.toFixed(2)}%`,
                `TASK_COMPLETION_RATE: ${analytics.task_completion_rate.toFixed(2)}%`,
                `EXCELLENCE_SCORE: ${analytics.excellence_score_average.toFixed(2)}`,
                `ANALYTICS_DATA: ${JSON.stringify(analytics, null, 2)}`
            ]
        }]);
        
        console.log(`✅ Performance analytics generated for ${agentId}`);
        return analytics;
        
    } catch (error) {
        console.error(`❌ Error generating performance analytics for ${agentId}:`, error);
        return {
            agent_id: agentId,
            time_window: timeWindow,
            error: error.message,
            analysis_failed: true
        };
    }
}

/**
 * LOG MANAGEMENT AND MAINTENANCE
 */

/**
 * Flush log buffer to cipher memory storage
 */
async function flushLogBuffer() {
    if (logBuffer.length === 0) return;
    
    console.log(`💾 Flushing ${logBuffer.length} log entries to storage...`);
    
    try {
        // Group logs by agent for efficient storage
        const logsByAgent = {};
        for (const log of logBuffer) {
            if (!logsByAgent[log.agent_id]) {
                logsByAgent[log.agent_id] = [];
            }
            logsByAgent[log.agent_id].push(log);
        }
        
        // Store logs in batches
        for (const [agentId, logs] of Object.entries(logsByAgent)) {
            await storeLogBatch(agentId, logs);
        }
        
        // Clear buffer and timer
        logBuffer = [];
        if (flushTimer) {
            clearTimeout(flushTimer);
            flushTimer = null;
        }
        
        console.log(`✅ Log buffer flushed successfully`);
        
    } catch (error) {
        console.error('❌ Error flushing log buffer:', error);
        // Keep logs in buffer for retry - don't clear on error
    }
}

/**
 * Store a batch of logs for a specific agent
 * @param {string} agentId - Agent ID
 * @param {Array} logs - Log entries to store
 */
async function storeLogBatch(agentId, logs) {
    const batchTimestamp = Date.now();
    const batchId = `log_batch_${agentId}_${batchTimestamp}`;
    
    // Prepare batch summary
    const batchSummary = {
        total_entries: logs.length,
        levels: {},
        operations: {},
        time_range: {
            start: logs[logs.length - 1]?.timestamp, // Oldest (assuming reverse chronological order)
            end: logs[0]?.timestamp // Newest
        }
    };
    
    // Calculate summary statistics
    for (const log of logs) {
        batchSummary.levels[log.level] = (batchSummary.levels[log.level] || 0) + 1;
        batchSummary.operations[log.operation] = (batchSummary.operations[log.operation] || 0) + 1;
    }
    
    // Compress log data if enabled
    const logData = LOG_CONFIG.storage.compression_enabled 
        ? compressLogData(logs) 
        : JSON.stringify(logs, null, 2);
    
    // Store batch entity
    await mcp__cipher_memory__create_entities([{
        name: batchId,
        entityType: "log_batch",
        observations: [
            `BATCH_TIMESTAMP: ${new Date(batchTimestamp).toISOString()}`,
            `AGENT_ID: ${agentId}`,
            `ENTRY_COUNT: ${logs.length}`,
            `LEVEL_DISTRIBUTION: ${JSON.stringify(batchSummary.levels)}`,
            `OPERATION_DISTRIBUTION: ${JSON.stringify(batchSummary.operations)}`,
            `TIME_RANGE: ${batchSummary.time_range.start} to ${batchSummary.time_range.end}`,
            `LOG_DATA: ${logData}`,
            `COMPRESSION_ENABLED: ${LOG_CONFIG.storage.compression_enabled}`,
            `BATCH_VERSION: 1.0`
        ]
    }]);
}

/**
 * Clean up old logs based on retention policies
 */
async function cleanupOldLogs() {
    console.log('🧹 Starting log cleanup based on retention policies...');
    
    try {
        const allLogBatches = await mcp__cipher_memory__search_nodes({
            query: "log_batch_*"
        });
        
        const now = new Date();
        const toDelete = [];
        
        for (const batch of allLogBatches) {
            const batchTimestamp = extractBatchTimestamp(batch.name);
            if (!batchTimestamp) continue;
            
            const batchDate = new Date(batchTimestamp);
            const ageInDays = (now - batchDate) / (1000 * 60 * 60 * 24);
            
            // Check if batch should be deleted based on retention policy
            const shouldDelete = shouldDeleteLogBatch(batch, ageInDays);
            
            if (shouldDelete) {
                toDelete.push(batch.name);
            }
        }
        
        if (toDelete.length > 0) {
            console.log(`🗑️  Deleting ${toDelete.length} expired log batches...`);
            await mcp__cipher_memory__delete_entities({ entityNames: toDelete });
            console.log(`✅ Cleanup completed: ${toDelete.length} log batches deleted`);
        } else {
            console.log(`✅ Cleanup completed: No expired log batches found`);
        }
        
    } catch (error) {
        console.error('❌ Error during log cleanup:', error);
    }
}

/**
 * UTILITY FUNCTIONS
 */

function generateSessionId() {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

function generateCorrelationId() {
    return `corr_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

function getLevelPrefix(level) {
    const prefixes = {
        DEBUG: '🔍',
        INFO: 'ℹ️ ',
        WARN: '⚠️ ',
        ERROR: '❌',
        CRITICAL: '🚨'
    };
    return prefixes[level.toUpperCase()] || 'ℹ️ ';
}

function determineHealthStatus(metrics) {
    const cpu = metrics.cpu_usage_percent || 0;
    const memory = metrics.memory_usage_mb || 0;
    const successRate = metrics.success_rate_percent || 100;
    const responseTime = metrics.average_response_time_ms || 0;
    
    if (cpu > 90 || memory > 1000 || successRate < 50 || responseTime > 10000) {
        return 'critical';
    }
    if (cpu > 70 || memory > 500 || successRate < 80 || responseTime > 5000) {
        return 'warning';
    }
    if (cpu < 20 && memory < 200 && successRate > 95 && responseTime < 1000) {
        return 'excellent';
    }
    return 'healthy';
}

function parseLogEntry(logNode) {
    // Parse log entry from stored cipher memory node
    try {
        for (const obs of logNode.observations || []) {
            if (obs.includes('LOG_DATA:')) {
                const logDataStr = obs.split('LOG_DATA:')[1].trim();
                const logData = JSON.parse(logDataStr);
                
                // If it's a batch, return the first entry as example
                if (Array.isArray(logData) && logData.length > 0) {
                    return logData[0];
                }
                return logData;
            }
        }
        return null;
    } catch (error) {
        return null;
    }
}

function getTimeWindowHours(timeWindow) {
    const timeMap = {
        '1h': 1,
        '24h': 24,
        '7d': 168,
        '30d': 720
    };
    return timeMap[timeWindow] || 168; // Default to 7 days
}

function generatePerformanceInsights(analytics) {
    const insights = [];
    
    // Error rate insights
    if (analytics.error_rate_percent > 10) {
        insights.push({
            type: 'error_rate',
            severity: 'high',
            message: `High error rate detected: ${analytics.error_rate_percent.toFixed(1)}%`,
            impact: 'System reliability is compromised'
        });
    } else if (analytics.error_rate_percent > 5) {
        insights.push({
            type: 'error_rate',
            severity: 'medium',
            message: `Elevated error rate: ${analytics.error_rate_percent.toFixed(1)}%`,
            impact: 'Monitor for degrading performance'
        });
    }
    
    // Task completion insights
    if (analytics.task_completion_rate < 80) {
        insights.push({
            type: 'task_completion',
            severity: 'high',
            message: `Low task completion rate: ${analytics.task_completion_rate.toFixed(1)}%`,
            impact: 'Productivity and reliability concerns'
        });
    }
    
    // Excellence score insights
    if (analytics.excellence_score_average < 7) {
        insights.push({
            type: 'excellence_score',
            severity: 'medium',
            message: `Below-target excellence score: ${analytics.excellence_score_average.toFixed(1)}/10`,
            impact: 'Quality standards not being met consistently'
        });
    }
    
    // Collaboration insights
    if (analytics.collaborations_initiated === 0) {
        insights.push({
            type: 'collaboration',
            severity: 'low',
            message: 'No cross-agent collaborations detected',
            impact: 'Agent may be working in isolation'
        });
    }
    
    return insights;
}

function generatePerformanceRecommendations(analytics) {
    const recommendations = [];
    
    // Error rate recommendations
    if (analytics.error_rate_percent > 5) {
        recommendations.push({
            category: 'reliability',
            priority: 'high',
            recommendation: 'Investigate and resolve recurring errors',
            action: 'Query error logs and identify common failure patterns'
        });
    }
    
    // Response time recommendations
    if (analytics.average_response_time_ms > 5000) {
        recommendations.push({
            category: 'performance',
            priority: 'medium',
            recommendation: 'Optimize slow operations',
            action: 'Profile operations taking >5 seconds and implement optimizations'
        });
    }
    
    // Excellence score recommendations
    if (analytics.excellence_score_average < 8) {
        recommendations.push({
            category: 'quality',
            priority: 'high',
            recommendation: 'Improve adherence to excellence standards',
            action: 'Review Standards Stan feedback and implement quality improvements'
        });
    }
    
    // Collaboration recommendations
    if (analytics.collaborations_initiated < 2) {
        recommendations.push({
            category: 'collaboration',
            priority: 'medium',
            recommendation: 'Increase cross-agent collaboration',
            action: 'Identify opportunities for pattern sharing and expertise exchange'
        });
    }
    
    return recommendations;
}

function compressLogData(logs) {
    // Simple compression: stringify and indicate compression (real implementation would use actual compression)
    const jsonString = JSON.stringify(logs, null, 0); // No formatting to save space
    return `[COMPRESSED:${jsonString.length}]${jsonString}`;
}

function extractBatchTimestamp(batchName) {
    // Extract timestamp from batch name like "log_batch_agent-name_1234567890"
    const parts = batchName.split('_');
    const timestamp = parts[parts.length - 1];
    return parseInt(timestamp, 10) || null;
}

function shouldDeleteLogBatch(batch, ageInDays) {
    // Determine if a log batch should be deleted based on retention policy
    
    // Check if batch contains critical logs (never delete)
    if (batch.observations.some(obs => obs.includes('CRITICAL'))) {
        return false;
    }
    
    // Check retention periods based on log levels
    if (batch.observations.some(obs => obs.includes('ERROR')) && ageInDays > LOG_CONFIG.retention.error_logs_days) {
        return true;
    }
    
    if (batch.observations.some(obs => obs.includes('WARN')) && ageInDays > LOG_CONFIG.retention.warn_logs_days) {
        return true;
    }
    
    if (batch.observations.some(obs => obs.includes('INFO')) && ageInDays > LOG_CONFIG.retention.info_logs_days) {
        return true;
    }
    
    if (batch.observations.some(obs => obs.includes('DEBUG')) && ageInDays > LOG_CONFIG.retention.debug_logs_days) {
        return true;
    }
    
    return false;
}

/**
 * INITIALIZATION AND MAINTENANCE
 */

/**
 * Initialize the centralized logging system
 */
async function initializeLogging() {
    console.log('🚀 Initializing Centralized Agent Logging System...');
    
    try {
        // Create logging system entity to track initialization
        await mcp__cipher_memory__create_entities([{
            name: `logging_system_init_${Date.now()}`,
            entityType: "system_initialization",
            observations: [
                `INIT_TIMESTAMP: ${new Date().toISOString()}`,
                `LOGGING_VERSION: 1.0`,
                `BUFFER_SIZE: ${LOG_CONFIG.storage.batch_size}`,
                `FLUSH_INTERVAL: ${LOG_CONFIG.storage.flush_interval_ms}ms`,
                `PERFORMANCE_THRESHOLD: ${LOG_CONFIG.monitoring.performance_threshold_ms}ms`,
                `RETENTION_POLICIES: ${JSON.stringify(LOG_CONFIG.retention, null, 2)}`,
                `MONITORING_ENABLED: ${JSON.stringify(LOG_CONFIG.monitoring, null, 2)}`,
                `STATUS: Initialized and ready for agent logging`
            ]
        }]);
        
        console.log('✅ Centralized Agent Logging System initialized successfully');
        
        // Schedule periodic maintenance
        setInterval(cleanupOldLogs, 24 * 60 * 60 * 1000); // Run cleanup daily
        
    } catch (error) {
        console.error('❌ Failed to initialize logging system:', error);
    }
}

/**
 * EXPORTED API FUNCTIONS
 */

module.exports = {
    // Core logging functions
    logAgentOperation,
    logAgentCollaboration,
    logPatternUsage,
    logTaskExecution,
    logAgentError,
    logAgentHealth,
    
    // Specialized logging
    logStandardsReview,
    logOrchestratorDecision,
    
    // Analytics and querying
    queryLogs,
    generatePerformanceAnalytics,
    
    // System management
    flushLogBuffer,
    cleanupOldLogs,
    initializeLogging,
    
    // Configuration
    LOG_CONFIG
};

/**
 * USAGE EXAMPLES:
 * 
 * // Initialize logging system
 * await initializeLogging();
 * 
 * // Log agent operation
 * await logAgentOperation('serial-comm-specialist', 'INFO', 'device_connection', {
 *   message: 'Successfully connected to Arduino Uno',
 *   device_type: 'arduino_uno',
 *   connection_time_ms: 1250,
 *   task_id: '27.3'
 * });
 * 
 * // Log pattern usage
 * await logPatternUsage('rust-async-specialist', 'exponential_backoff_pattern', {
 *   context: 'Transport reconnection after timeout',
 *   success: true,
 *   effectiveness_score: 0.95,
 *   time_saved_minutes: 15
 * });
 * 
 * // Log collaboration
 * await logAgentCollaboration('task-orchestrator', 'serial-comm-specialist', 'expertise_request', {
 *   success: true,
 *   response_time_ms: 850,
 *   patterns_shared: 2,
 *   quality_score: 9.2
 * });
 * 
 * // Generate analytics
 * const analytics = await generatePerformanceAnalytics('serial-comm-specialist', '7d');
 * 
 * // Query logs
 * const errorLogs = await queryLogs({ level: 'ERROR', time_range: '24h', limit: 50 });
 */