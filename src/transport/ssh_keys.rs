// SSH Key Management Module
// Provides utilities for discovering, loading, and managing SSH keys

use std::path::{Path, PathBuf};
use std::fs;
use crate::transport::{TransportError, TransportResult};

/// Common SSH key file names to search for
const SSH_KEY_NAMES: &[&str] = &[
    "id_ed25519",
    "id_rsa",
    "id_ecdsa",
    "id_dsa",
];

/// SSH key type
#[derive(Debug, Clone, PartialEq)]
pub enum SshKeyType {
    Ed25519,
    Rsa,
    Ecdsa,
    Dsa,
    Unknown,
}

impl SshKeyType {
    /// Detect key type from file path
    pub fn from_path(path: &Path) -> Self {
        let filename = path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        
        if filename.contains("ed25519") {
            SshKeyType::Ed25519
        } else if filename.contains("rsa") {
            SshKeyType::Rsa
        } else if filename.contains("ecdsa") {
            SshKeyType::Ecdsa
        } else if filename.contains("dsa") {
            SshKeyType::Dsa
        } else {
            SshKeyType::Unknown
        }
    }
}

/// SSH key information
#[derive(Debug, Clone)]
pub struct SshKeyInfo {
    pub path: PathBuf,
    pub key_type: SshKeyType,
    pub is_encrypted: bool,
}

/// SSH key discovery and management
pub struct SshKeyManager {
    search_paths: Vec<PathBuf>,
}

impl SshKeyManager {
    /// Create a new SSH key manager
    pub fn new() -> Self {
        let mut search_paths = Vec::new();
        
        // Add default SSH directory
        if let Some(home) = dirs::home_dir() {
            search_paths.push(home.join(".ssh"));
        }
        
        // On Windows, also check common paths
        #[cfg(windows)]
        {
            if let Ok(userprofile) = std::env::var("USERPROFILE") {
                search_paths.push(PathBuf::from(userprofile).join(".ssh"));
            }
            if let Ok(home) = std::env::var("HOME") {
                search_paths.push(PathBuf::from(home).join(".ssh"));
            }
        }
        
        SshKeyManager { search_paths }
    }
    
    /// Add a custom search path
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }
    
    /// Discover SSH keys in standard locations
    pub fn discover_keys(&self) -> Vec<SshKeyInfo> {
        let mut keys = Vec::new();
        
        for search_path in &self.search_paths {
            if !search_path.exists() || !search_path.is_dir() {
                continue;
            }
            
            for key_name in SSH_KEY_NAMES {
                let key_path = search_path.join(key_name);
                if key_path.exists() && key_path.is_file() {
                    if let Some(info) = self.analyze_key_file(&key_path) {
                        keys.push(info);
                    }
                }
            }
        }
        
        keys
    }
    
    /// Find the best available SSH key
    pub fn find_best_key(&self) -> Option<SshKeyInfo> {
        let keys = self.discover_keys();
        
        // Prefer Ed25519 keys (modern and secure)
        if let Some(key) = keys.iter().find(|k| k.key_type == SshKeyType::Ed25519) {
            return Some(key.clone());
        }
        
        // Then RSA keys (widely compatible)
        if let Some(key) = keys.iter().find(|k| k.key_type == SshKeyType::Rsa) {
            return Some(key.clone());
        }
        
        // Then ECDSA keys
        if let Some(key) = keys.iter().find(|k| k.key_type == SshKeyType::Ecdsa) {
            return Some(key.clone());
        }
        
        // Finally DSA (deprecated but still supported)
        if let Some(key) = keys.iter().find(|k| k.key_type == SshKeyType::Dsa) {
            return Some(key.clone());
        }
        
        // Return any key if available
        keys.into_iter().next()
    }
    
    /// Load a specific SSH key file
    pub fn load_key(&self, path: &Path) -> TransportResult<SshKeyInfo> {
        if !path.exists() {
            return Err(TransportError::ConfigError(
                format!("SSH key file not found: {}", path.display())
            ));
        }
        
        if !path.is_file() {
            return Err(TransportError::ConfigError(
                format!("SSH key path is not a file: {}", path.display())
            ));
        }
        
        self.analyze_key_file(path)
            .ok_or_else(|| TransportError::ConfigError(
                format!("Failed to analyze SSH key file: {}", path.display())
            ))
    }
    
    /// Analyze a key file to determine its properties
    fn analyze_key_file(&self, path: &Path) -> Option<SshKeyInfo> {
        let content = fs::read_to_string(path).ok()?;
        
        // Check if the key is encrypted
        let is_encrypted = content.contains("ENCRYPTED") || 
                          content.contains("Proc-Type: 4,ENCRYPTED");
        
        // Detect key type
        let key_type = SshKeyType::from_path(path);
        
        Some(SshKeyInfo {
            path: path.to_path_buf(),
            key_type,
            is_encrypted,
        })
    }
    
    /// Validate SSH key permissions (important on Unix-like systems)
    #[cfg(unix)]
    pub fn validate_key_permissions(&self, path: &Path) -> TransportResult<()> {
        use std::os::unix::fs::PermissionsExt;
        
        let metadata = fs::metadata(path)
            .map_err(|e| TransportError::ConfigError(
                format!("Failed to read key file metadata: {}", e)
            ))?;
        
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        
        // Check that the key is only readable by the owner (0600 or 0400)
        if mode & 0o077 != 0 {
            return Err(TransportError::PermissionDenied(
                format!("SSH key file {} has incorrect permissions. Should be 0600 or 0400", path.display())
            ));
        }
        
        Ok(())
    }
    
    /// Validate SSH key permissions (no-op on Windows)
    #[cfg(not(unix))]
    pub fn validate_key_permissions(&self, _path: &Path) -> TransportResult<()> {
        // Windows doesn't have the same permission model
        Ok(())
    }
    
    /// Get the public key path for a private key
    pub fn get_public_key_path(&self, private_key_path: &Path) -> PathBuf {
        let mut public_path = private_key_path.to_path_buf();
        let filename = private_key_path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        public_path.set_file_name(format!("{}.pub", filename));
        public_path
    }
}

impl Default for SshKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_key_type_detection() {
        assert_eq!(SshKeyType::from_path(Path::new("id_ed25519")), SshKeyType::Ed25519);
        assert_eq!(SshKeyType::from_path(Path::new("id_rsa")), SshKeyType::Rsa);
        assert_eq!(SshKeyType::from_path(Path::new("id_ecdsa")), SshKeyType::Ecdsa);
        assert_eq!(SshKeyType::from_path(Path::new("id_dsa")), SshKeyType::Dsa);
        assert_eq!(SshKeyType::from_path(Path::new("unknown_key")), SshKeyType::Unknown);
    }
    
    #[test]
    fn test_key_discovery() {
        let temp_dir = tempdir().unwrap();
        let ssh_dir = temp_dir.path().join(".ssh");
        fs::create_dir(&ssh_dir).unwrap();
        
        // Create mock key files
        let ed25519_key = ssh_dir.join("id_ed25519");
        let mut file = File::create(&ed25519_key).unwrap();
        writeln!(file, "-----BEGIN OPENSSH PRIVATE KEY-----").unwrap();
        writeln!(file, "mock key content").unwrap();
        writeln!(file, "-----END OPENSSH PRIVATE KEY-----").unwrap();
        
        let rsa_key = ssh_dir.join("id_rsa");
        let mut file = File::create(&rsa_key).unwrap();
        writeln!(file, "-----BEGIN RSA PRIVATE KEY-----").unwrap();
        writeln!(file, "mock key content").unwrap();
        writeln!(file, "-----END RSA PRIVATE KEY-----").unwrap();
        
        let mut manager = SshKeyManager::new();
        manager.add_search_path(temp_dir.path().join(".ssh"));
        
        let keys = manager.discover_keys();
        assert_eq!(keys.len(), 2);
        
        // Test best key selection (should prefer Ed25519)
        let best_key = manager.find_best_key();
        assert!(best_key.is_some());
        assert_eq!(best_key.unwrap().key_type, SshKeyType::Ed25519);
    }
    
    #[test]
    fn test_encrypted_key_detection() {
        let temp_dir = tempdir().unwrap();
        let key_path = temp_dir.path().join("id_rsa");
        
        let mut file = File::create(&key_path).unwrap();
        writeln!(file, "-----BEGIN RSA PRIVATE KEY-----").unwrap();
        writeln!(file, "Proc-Type: 4,ENCRYPTED").unwrap();
        writeln!(file, "DEK-Info: AES-128-CBC,1234567890ABCDEF").unwrap();
        writeln!(file, "encrypted content").unwrap();
        writeln!(file, "-----END RSA PRIVATE KEY-----").unwrap();
        
        let manager = SshKeyManager::new();
        let key_info = manager.load_key(&key_path).unwrap();
        
        assert!(key_info.is_encrypted);
        assert_eq!(key_info.key_type, SshKeyType::Rsa);
    }
}