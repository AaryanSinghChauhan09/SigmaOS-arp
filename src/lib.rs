//! SigmaOS-arp: ARP network protocol model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for ARP network protocol.
pub struct ArpEntry {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub enabled: bool,
}

/// Secondary domain entity for ARP network protocol.
pub struct ArpInterface {
    pub id: String,
    pub hwaddr: String,
    pub description: String,
}

/// Errors for the ARP network protocol domain.
#[derive(Debug, PartialEq)]
pub enum ArpError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for ArpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for ArpError {}

/// Registry and manager for ARP network protocol.
pub struct ArpManager {
    pub name: String,
    items: BTreeMap<String, ArpEntry>,
    configs: BTreeMap<String, ArpInterface>,
}

impl ArpManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: ArpEntry) -> Result<(), ArpError> {
        if item.name.is_empty() {
            return Err(ArpError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(ArpError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&ArpEntry> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: ArpInterface) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), ArpError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| ArpError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ArpEntry(id: &str, name: &str) -> ArpEntry {
        ArpEntry {
            id: id.to_string(),
            name: name.to_string(),
            ip_address: "arp-default".to_string(),
            enabled: true,
        }
    }

    fn make_ArpInterface(id: &str, val: &str) -> ArpInterface {
        ArpInterface {
            id: id.to_string(),
            hwaddr: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = ArpManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = ArpManager::new("test");
        let item = make_ArpEntry("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = ArpManager::new("test");
        let item = make_ArpEntry("item1", "");
        assert_eq!(mgr.add_item(item), Err(ArpError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = ArpManager::new("test");
        let item1 = make_ArpEntry("item1", "First");
        let item2 = make_ArpEntry("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(ArpError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = ArpManager::new("test");
        let cfg = make_ArpInterface("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = ArpManager::new("test");
        let item = make_ArpEntry("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = ArpManager::new("test");
        let item = make_ArpEntry("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(ArpError::NotFound("none".to_string())));
    }
}
