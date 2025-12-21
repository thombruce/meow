#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.bars.left, vec!["workspaces"]);
        assert_eq!(config.bars.middle, vec!["time", "weather"]);
        assert_eq!(config.bars.right.len(), 8);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.bars.left, parsed.bars.left);
        assert_eq!(config.bars.middle, parsed.bars.middle);
        assert_eq!(config.bars.right, parsed.bars.right);
    }

    #[test]
    fn test_component_creation() {
        let component_names = vec!["time", "weather", "temperature"];
        for name in component_names {
            let result = Component::new(name);
            assert!(result.is_ok(), "Failed to create component: {}", name);
        }
    }

    #[test]
    fn test_component_manager() {
        let manager = ComponentManager::new();
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        let left_components = manager.get_bar_components("left");
        assert!(!left_components.is_empty());
        
        let middle_components = manager.get_bar_components("middle");
        assert_eq!(middle_components.len(), 2);
        
        let right_components = manager.get_bar_components("right");
        assert_eq!(right_components.len(), 8);
    }
}