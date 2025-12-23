use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bars: BarsConfig,
    pub colorize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarsConfig {
    pub left: Vec<ComponentConfig>,
    pub middle: Vec<ComponentConfig>,
    pub right: Vec<ComponentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentConfig {
    Simple(String),
    Detailed(serde_json::Value),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bars: BarsConfig {
                left: vec![
                    ComponentConfig::Simple("workspaces".to_string()),
                    ComponentConfig::Simple("windows".to_string()),
                ],
                middle: vec![
                    ComponentConfig::Simple("time".to_string()),
                    ComponentConfig::Simple("separator".to_string()),
                    ComponentConfig::Simple("weather".to_string()),
                ],
                right: vec![
                    ComponentConfig::Simple("temperature".to_string()),
                    ComponentConfig::Simple("space".to_string()),
                    ComponentConfig::Simple("cpu".to_string()),
                    ComponentConfig::Simple("space".to_string()),
                    ComponentConfig::Simple("ram".to_string()),
                    ComponentConfig::Simple("separator".to_string()),
                    ComponentConfig::Simple("wifi".to_string()),
                    ComponentConfig::Simple("separator".to_string()),
                    ComponentConfig::Simple("brightness".to_string()),
                    ComponentConfig::Simple("space".to_string()),
                    ComponentConfig::Simple("volume".to_string()),
                    ComponentConfig::Simple("separator".to_string()),
                    ComponentConfig::Simple("battery".to_string()),
                ],
            },
            colorize: true,
        }
    }
}

impl Config {
    pub fn load() -> color_eyre::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> color_eyre::Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> std::path::PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home_dir)
            .join(".config")
            .join("catfood")
            .join("bar.json")
    }

    pub fn get_components_for_bar(&self, bar: &str) -> Option<&Vec<ComponentConfig>> {
        match bar {
            "left" => Some(&self.bars.left),
            "middle" => Some(&self.bars.middle),
            "right" => Some(&self.bars.right),
            _ => None,
        }
    }

    pub fn reload(&self) -> color_eyre::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}
