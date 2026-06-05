//! Driver registry for storing and managing converted drivers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::error::Result;

/// Version history entry for a driver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverVersion {
    pub version: String,
    pub timestamp: String,
    pub source_hash: String,
    pub checksum: String,
    pub notes: String,
}

/// Installed driver entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledDriver {
    pub name: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub target_os: String,
    pub current_version: String,
    pub installation_path: String,
    pub installed_at: String,
    pub versions: Vec<DriverVersion>,
}

impl InstalledDriver {
    pub fn new(
        name: String,
        vendor_id: u16,
        device_id: u16,
        target_os: String,
        path: String,
    ) -> Self {
        Self {
            name,
            vendor_id,
            device_id,
            target_os,
            current_version: "1.0.0".to_string(),
            installation_path: path,
            installed_at: Utc::now().to_rfc3339(),
            versions: vec![DriverVersion {
                version: "1.0.0".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                source_hash: String::new(),
                checksum: String::new(),
                notes: "Initial installation".to_string(),
            }],
        }
    }

    pub fn add_version(&mut self, version: DriverVersion) {
        self.versions.push(version);
        self.current_version = version.version.clone();
    }

    pub fn get_version(&self, version: &str) -> Option<&DriverVersion> {
        self.versions.iter().find(|v| v.version == version)
    }

    pub fn get_latest_version(&self) -> Option<&DriverVersion> {
        self.versions.last()
    }
}

/// Driver registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverRegistry {
    #[serde(skip)]
    file_path: String,
    drivers: HashMap<String, InstalledDriver>,
}

impl DriverRegistry {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            drivers: HashMap::new(),
        }
    }

    /// Create a key for the registry
    fn make_key(vendor_id: u16, device_id: u16, target_os: &str) -> String {
        format!("{:04x}_{:04x}_{}", vendor_id, device_id, target_os)
    }

    /// Register a driver
    pub fn register(
        &mut self,
        driver: InstalledDriver,
    ) -> Result<()> {
        let key = Self::make_key(driver.vendor_id, driver.device_id, &driver.target_os);
        self.drivers.insert(key, driver);
        Ok(())
    }

    /// Unregister a driver
    pub fn unregister(&mut self, vendor_id: u16, device_id: u16, target_os: &str) -> Result<()> {
        let key = Self::make_key(vendor_id, device_id, target_os);
        self.drivers.remove(&key);
        Ok(())
    }

    /// Get a registered driver
    pub fn get(
        &self,
        vendor_id: u16,
        device_id: u16,
        target_os: &str,
    ) -> Option<&InstalledDriver> {
        let key = Self::make_key(vendor_id, device_id, target_os);
        self.drivers.get(&key)
    }

    /// Get a mutable reference to a registered driver
    pub fn get_mut(
        &mut self,
        vendor_id: u16,
        device_id: u16,
        target_os: &str,
    ) -> Option<&mut InstalledDriver> {
        let key = Self::make_key(vendor_id, device_id, target_os);
        self.drivers.get_mut(&key)
    }

    /// List all registered drivers
    pub fn list_all(&self) -> Vec<&InstalledDriver> {
        self.drivers.values().collect()
    }

    /// List drivers for a specific target OS
    pub fn list_by_os(&self, target_os: &str) -> Vec<&InstalledDriver> {
        self.drivers
            .values()
            .filter(|d| d.target_os == target_os)
            .collect()
    }

    /// List drivers for a specific device
    pub fn list_by_device(
        &self,
        vendor_id: u16,
        device_id: u16,
    ) -> Vec<&InstalledDriver> {
        self.drivers
            .values()
            .filter(|d| d.vendor_id == vendor_id && d.device_id == device_id)
            .collect()
    }

    /// Rollback a driver to a specific version
    pub fn rollback(
        &mut self,
        vendor_id: u16,
        device_id: u16,
        target_os: &str,
        version: &str,
    ) -> Result<()> {
        let key = Self::make_key(vendor_id, device_id, target_os);
        if let Some(driver) = self.drivers.get_mut(&key) {
            if driver.get_version(version).is_some() {
                driver.current_version = version.to_string();
                Ok(())
            } else {
                Err(crate::UdcError::ValidationError(format!(
                    "Version {} not found",
                    version
                )))
            }
        } else {
            Err(crate::UdcError::ValidationError(
                "Driver not found".to_string(),
            ))
        }
    }

    /// Load registry from JSON file
    pub fn load(file_path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;
        let mut registry: DriverRegistry = serde_json::from_str(&content)?;
        registry.file_path = file_path.to_string();
        Ok(registry)
    }

    /// Save registry to JSON file
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.drivers)?;
        std::fs::write(&self.file_path, json).map_err(|e| {
            crate::UdcError::IoError(e)
        })?;
        Ok(())
    }

    /// Create or load registry with default path
    pub fn with_default_path() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = format!("{}/.udc/drivers.json", home);

        // Create directory if it doesn't exist
        let dir = std::path::Path::new(&path).parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir).ok();
        }

        if std::path::Path::new(&path).exists() {
            Self::load(&path)
        } else {
            Ok(Self::new(path))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_registry_operations() {
        let mut registry = DriverRegistry::new(":memory:".to_string());

        let driver = InstalledDriver::new(
            "Test Driver".to_string(),
            0x1234,
            0x5678,
            "linux_kernel".to_string(),
            "/opt/drivers/test".to_string(),
        );

        registry.register(driver.clone()).unwrap();

        // Verify we can retrieve it
        let retrieved = registry.get(0x1234, 0x5678, "linux_kernel");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Driver");
    }

    #[test]
    fn test_list_by_os() {
        let mut registry = DriverRegistry::new(":memory:".to_string());

        let driver1 = InstalledDriver::new(
            "Driver1".to_string(),
            0x1234,
            0x5678,
            "linux_kernel".to_string(),
            "/opt/drivers/1".to_string(),
        );

        let driver2 = InstalledDriver::new(
            "Driver2".to_string(),
            0x1234,
            0x5679,
            "macos_driverkit".to_string(),
            "/opt/drivers/2".to_string(),
        );

        registry.register(driver1).unwrap();
        registry.register(driver2).unwrap();

        let linux_drivers = registry.list_by_os("linux_kernel");
        assert_eq!(linux_drivers.len(), 1);
    }
}
