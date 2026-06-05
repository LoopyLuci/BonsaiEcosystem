//! Rule database for driver conversion

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A conversion rule that maps from one instruction pattern to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub replacement: String,
    pub platforms: Vec<String>, // empty = all platforms
    pub priority: i32,
}

impl ConversionRule {
    pub fn new(
        name: impl Into<String>,
        pattern: impl Into<String>,
        replacement: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            pattern: pattern.into(),
            replacement: replacement.into(),
            platforms: vec![],
            priority: 0,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_platforms(mut self, platforms: Vec<String>) -> Self {
        self.platforms = platforms;
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// Rule database
pub struct RuleDatabase {
    rules: Vec<ConversionRule>,
}

impl RuleDatabase {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn with_default_usb_rules() -> Self {
        let mut db = Self::new();
        db.add_rule(
            ConversionRule::new(
                "readl_to_ioread32",
                "readl(addr)",
                "ioread32(addr)",
            )
            .with_description("Linux: Convert readl to ioread32")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(10),
        );

        db.add_rule(
            ConversionRule::new(
                "writel_to_iowrite32",
                "writel(value, addr)",
                "iowrite32(value, addr)",
            )
            .with_description("Linux: Convert writel to iowrite32")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(10),
        );

        db.add_rule(
            ConversionRule::new(
                "usb_bulk_msg_pattern",
                "usb_bulk_msg(dev, pipe, buffer, length, actual, timeout)",
                "int ret = usb_bulk_msg(dev, pipe, buffer, length, &actual_length, timeout); if (ret) return ret;",
            )
            .with_description("Linux: Standardize USB bulk message pattern")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(5),
        );

        db.add_rule(
            ConversionRule::new(
                "interrupt_handler_setup",
                "setup_interrupt(irq, handler)",
                "request_irq(irq, handler, IRQF_SHARED, \"driver\", NULL)",
            )
            .with_description("Linux: Convert generic interrupt setup")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(8),
        );

        db.add_rule(
            ConversionRule::new(
                "mmio_read_pattern",
                "read_mem(addr, 32)",
                "ioread32(addr)",
            )
            .with_description("Linux: Convert generic memory read")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(5),
        );

        db.add_rule(
            ConversionRule::new(
                "mmio_write_pattern",
                "write_mem(addr, value, 32)",
                "iowrite32(value, addr)",
            )
            .with_description("Linux: Convert generic memory write")
            .with_platforms(vec!["linux_kernel".to_string()])
            .with_priority(5),
        );

        db.add_rule(
            ConversionRule::new(
                "macos_usb_bulk_write",
                "usb_bulk_write(ep, buffer, size, timeout)",
                "IOReturn ret = bulkPipe->Send(descriptor, timeout, nullptr, nullptr);",
            )
            .with_description("macOS: Convert USB bulk write")
            .with_platforms(vec!["macos_driverkit".to_string()])
            .with_priority(10),
        );

        db.add_rule(
            ConversionRule::new(
                "macos_usb_bulk_read",
                "usb_bulk_read(ep, buffer, size, timeout)",
                "IOReturn ret = bulkPipe->Recv(descriptor, timeout, nullptr, nullptr);",
            )
            .with_description("macOS: Convert USB bulk read")
            .with_platforms(vec!["macos_driverkit".to_string()])
            .with_priority(10),
        );

        db.add_rule(
            ConversionRule::new(
                "UOSC_async_operation",
                "async_operation(op)",
                "let result = operation.await;",
            )
            .with_description("UOSC: Convert to async/await pattern")
            .with_platforms(vec!["UOSC".to_string()])
            .with_priority(10),
        );

        db
    }

    pub fn add_rule(&mut self, rule: ConversionRule) {
        self.rules.push(rule);
        // Sort by priority (highest first)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn get_rules(&self) -> &[ConversionRule] {
        &self.rules
    }

    pub fn get_rules_for_platform(&self, platform: &str) -> Vec<&ConversionRule> {
        self.rules
            .iter()
            .filter(|r| r.platforms.is_empty() || r.platforms.contains(&platform.to_string()))
            .collect()
    }

    pub fn find_rule(&self, name: &str) -> Option<&ConversionRule> {
        self.rules.iter().find(|r| r.name == name)
    }

    pub fn apply_rule(&self, rule: &ConversionRule, input: &str) -> String {
        // Simple pattern replacement
        input.replace(&rule.pattern, &rule.replacement)
    }
}

impl Default for RuleDatabase {
    fn default() -> Self {
        Self::with_default_usb_rules()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rules_creation() {
        let db = RuleDatabase::with_default_usb_rules();
        assert!(!db.get_rules().is_empty());
        assert!(db.find_rule("readl_to_ioread32").is_some());
    }

    #[test]
    fn test_platform_filtering() {
        let db = RuleDatabase::with_default_usb_rules();
        let linux_rules = db.get_rules_for_platform("linux_kernel");
        assert!(!linux_rules.is_empty());
    }

    #[test]
    fn test_rule_application() {
        let db = RuleDatabase::with_default_usb_rules();
        let rule = db.find_rule("readl_to_ioread32").unwrap();
        let input = "readl(addr)";
        let output = db.apply_rule(rule, input);
        assert_eq!(output, "ioread32(addr)");
    }
}
