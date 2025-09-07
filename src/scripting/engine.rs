use rhai::{Engine, Scope, AST, Dynamic, EvalAltResult};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Mutex};
use std::collections::HashMap;
use super::{SandboxConfig, DeviceApi, ScriptError, ScriptResult};
use super::async_bridge::{AsyncBridge, register_sync_api};

/// Context for script execution
pub struct ScriptContext {
    pub script_id: String,
    pub variables: HashMap<String, Dynamic>,
    pub start_time: Instant,
    pub operations_count: u64,
}

impl ScriptContext {
    pub fn new(script_id: String) -> Self {
        Self {
            script_id,
            variables: HashMap::new(),
            start_time: Instant::now(),
            operations_count: 0,
        }
    }
    
    /// Check if execution time has exceeded limit
    pub fn check_timeout(&self, limit: Duration) -> ScriptResult<()> {
        if self.start_time.elapsed() > limit {
            Err(ScriptError::Timeout(limit))
        } else {
            Ok(())
        }
    }
    
    /// Increment and check operation count
    pub fn check_operations(&mut self, limit: u64) -> ScriptResult<()> {
        self.operations_count += 1;
        if self.operations_count > limit {
            Err(ScriptError::ResourceLimit(
                format!("Operation limit {} exceeded", limit)
            ))
        } else {
            Ok(())
        }
    }
}

/// Main scripting engine with sandboxing
pub struct ScriptEngine {
    engine: Arc<Mutex<Engine>>,
    sandbox_config: SandboxConfig,
    device_api: Arc<DeviceApi>,
    compiled_scripts: Arc<RwLock<HashMap<String, AST>>>,
    active_contexts: Arc<RwLock<HashMap<String, ScriptContext>>>,
}

impl ScriptEngine {
    /// Create a new scripting engine
    pub fn new(sandbox_config: SandboxConfig, device_api: Arc<DeviceApi>) -> ScriptResult<Self> {
        let mut engine = Engine::new();
        
        // Configure engine based on sandbox settings
        Self::configure_engine(&mut engine, &sandbox_config);
        
        // Register device API
        DeviceApi::register_api(&mut engine, device_api.clone());
        
        // Create and register async bridge
        let manager = Arc::new(crate::device::DeviceManager::new("plugins"));
        let bridge = Arc::new(AsyncBridge::new(manager)?);
        register_sync_api(&mut engine, bridge);
        
        Ok(Self {
            engine: Arc::new(Mutex::new(engine)),
            sandbox_config,
            device_api,
            compiled_scripts: Arc::new(RwLock::new(HashMap::new())),
            active_contexts: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Configure engine with sandbox restrictions
    fn configure_engine(engine: &mut Engine, config: &SandboxConfig) {
        // Set resource limits
        engine.set_max_operations(config.limits.max_operations);
        engine.set_max_expr_depths(
            config.limits.max_call_depth,
            config.limits.max_call_depth
        );
        engine.set_max_string_size(config.limits.max_string_size);
        engine.set_max_array_size(config.limits.max_array_size);
        
        // Disable dangerous features
        if !config.allow_filesystem {
            // Note: In production, would remove file operations
            engine.register_fn("read_file", |_: &str| -> Result<String, Box<EvalAltResult>> {
                Err("File system access is disabled".into())
            });
        }
        
        if !config.allow_network {
            // Note: In production, would remove network operations
            engine.register_fn("fetch", |_: &str| -> Result<String, Box<EvalAltResult>> {
                Err("Network access is disabled".into())
            });
        }
        
        // Remove denied functions
        for func_name in &config.denied_functions {
            // In Rhai, we can't directly remove functions, but we can override them
            let error_msg = format!("Function '{}' is denied", func_name);
            engine.register_fn(func_name.as_str(), move || -> Result<(), Box<EvalAltResult>> {
                Err(error_msg.clone().into())
            });
        }
        
        // Add operation counting hook
        engine.on_progress(move |operations| {
            // This is called periodically during script execution
            // Return None to continue, Some(error) to stop
            if operations > config.limits.max_operations {
                Some(format!("Operation limit {} exceeded", config.limits.max_operations).into())
            } else {
                None
            }
        });
    }
    
    /// Compile a script with validation
    pub async fn compile_script(&self, script_id: &str, source: &str) -> ScriptResult<()> {
        // Basic validation
        if source.is_empty() {
            return Err(ScriptError::Invalid("Script cannot be empty".to_string()));
        }
        
        if source.len() > self.sandbox_config.limits.max_string_size {
            return Err(ScriptError::Invalid("Script exceeds maximum size".to_string()));
        }
        
        // Check for obvious security issues
        let dangerous_patterns = [
            "std::process",
            "std::fs",
            "std::net",
            "unsafe",
            "extern",
        ];
        
        for pattern in &dangerous_patterns {
            if source.contains(pattern) {
                return Err(ScriptError::Security(
                    format!("Script contains forbidden pattern: {}", pattern)
                ));
            }
        }
        
        // Compile the script
        let engine = self.engine.lock().await;
        let ast = engine.compile(source)
            .map_err(|e| ScriptError::Compilation(e.to_string()))?;
        
        // Store compiled script
        let mut scripts = self.compiled_scripts.write().await;
        scripts.insert(script_id.to_string(), ast);
        
        Ok(())
    }
    
    /// Execute a compiled script
    pub async fn execute_script(&self, script_id: &str, scope: &mut Scope<'_>) -> ScriptResult<Dynamic> {
        // Get compiled script
        let scripts = self.compiled_scripts.read().await;
        let ast = scripts.get(script_id)
            .ok_or_else(|| ScriptError::Invalid(format!("Script {} not found", script_id)))?
            .clone();
        drop(scripts);
        
        // Create execution context
        let context = ScriptContext::new(script_id.to_string());
        let context_id = context.script_id.clone();
        
        // Store active context
        {
            let mut contexts = self.active_contexts.write().await;
            contexts.insert(context_id.clone(), context);
        }
        
        // Set up timeout
        let timeout = self.sandbox_config.limits.max_execution_time;
        let start = Instant::now();
        
        // Execute with timeout
        let engine = self.engine.lock().await;
        
        // Note: In production, would use tokio::time::timeout
        // For now, using the engine's built-in operation counter
        let result = engine.eval_ast_with_scope::<Dynamic>(scope, &ast);
        
        // Clean up context
        {
            let mut contexts = self.active_contexts.write().await;
            contexts.remove(&context_id);
        }
        
        // Check timeout
        if start.elapsed() > timeout {
            return Err(ScriptError::Timeout(timeout));
        }
        
        // Return result
        result.map_err(|e| ScriptError::Execution(e.to_string()))
    }
    
    /// Execute a script string directly (compile and run)
    pub async fn eval(&self, source: &str) -> ScriptResult<Dynamic> {
        let script_id = format!("eval_{}", uuid::Uuid::new_v4());
        self.compile_script(&script_id, source).await?;
        
        let mut scope = Scope::new();
        let result = self.execute_script(&script_id, &mut scope).await;
        
        // Clean up compiled script
        let mut scripts = self.compiled_scripts.write().await;
        scripts.remove(&script_id);
        
        result
    }
    
    /// Stop a running script
    pub async fn stop_script(&self, script_id: &str) -> ScriptResult<()> {
        let mut contexts = self.active_contexts.write().await;
        if contexts.remove(script_id).is_some() {
            tracing::info!("Stopped script: {}", script_id);
            Ok(())
        } else {
            Err(ScriptError::Invalid(format!("Script {} not running", script_id)))
        }
    }
    
    /// Get resource usage for a running script
    pub async fn get_script_stats(&self, script_id: &str) -> ScriptResult<ScriptStats> {
        let contexts = self.active_contexts.read().await;
        let context = contexts.get(script_id)
            .ok_or_else(|| ScriptError::Invalid(format!("Script {} not found", script_id)))?;
        
        Ok(ScriptStats {
            script_id: script_id.to_string(),
            runtime: context.start_time.elapsed(),
            operations: context.operations_count,
            variables: context.variables.len(),
        })
    }
}

/// Statistics for a running script
#[derive(Debug, Clone)]
pub struct ScriptStats {
    pub script_id: String,
    pub runtime: Duration,
    pub operations: u64,
    pub variables: usize,
}