use crate::components::{
    Battery, Brightness, Cpu, Ram, Separator, Space, Temperature, Time, Volume, Vpn, Weather, Wifi,
    Workspaces,
};
use crate::config::Config;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Component {
    Workspaces(Workspaces),
    Time(Time),
    Weather(Weather),
    Temperature(Temperature),
    Cpu(Cpu),
    Ram(Ram),
    Wifi(Wifi),
    Vpn(Vpn),
    Brightness(Brightness),
    Volume(Volume),
    Battery(Battery),
    Separator(Separator),
    Space(Space),
}

impl Component {
    pub fn new(component_type: &str) -> color_eyre::Result<Self> {
        match component_type {
            "workspaces" => Ok(Component::Workspaces(Workspaces::new())),
            "time" => Ok(Component::Time(Time::new())),
            "weather" => Ok(Component::Weather(Weather::new())),
            "temperature" => Ok(Component::Temperature(Temperature::new())),
            "cpu" => Ok(Component::Cpu(Cpu::new())),
            "ram" => Ok(Component::Ram(Ram::new())),
            "wifi" => Ok(Component::Wifi(Wifi::new())),
            "vpn" => Ok(Component::Vpn(Vpn::new())),
            "brightness" => Ok(Component::Brightness(Brightness::new())),
            "volume" => Ok(Component::Volume(Volume::new())),
            "battery" => Ok(Component::Battery(Battery::new()?)),
            "separator" => Ok(Component::Separator(Separator::new())),
            "space" => Ok(Component::Space(Space::new())),
            _ => Err(color_eyre::eyre::eyre!(
                "Unknown component type: {}",
                component_type
            )),
        }
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        match self {
            Component::Workspaces(component) => {
                component.update();
                Ok(())
            }
            Component::Time(component) => {
                component.update();
                Ok(())
            }
            Component::Weather(component) => {
                component.update();
                Ok(())
            }
            Component::Temperature(component) => {
                component.update();
                Ok(())
            }
            Component::Cpu(component) => {
                component.update();
                Ok(())
            }
            Component::Ram(component) => {
                component.update();
                Ok(())
            }
            Component::Wifi(component) => {
                component.update();
                Ok(())
            }
            Component::Vpn(component) => {
                component.update();
                Ok(())
            }
            Component::Brightness(component) => {
                component.update();
                Ok(())
            }
            Component::Volume(component) => {
                component.update();
                Ok(())
            }
            Component::Battery(component) => {
                component.update()?;
                Ok(())
            }
            Component::Separator(_component) => {
                // Separator doesn't need updates
                Ok(())
            }
            Component::Space(_component) => {
                // Space doesn't need updates
                Ok(())
            }
        }
    }

    pub fn render(&self) -> String {
        match self {
            Component::Workspaces(component) => {
                let spans = component.render();
                spans
                    .iter()
                    .map(|span| span.content.clone())
                    .collect::<String>()
            }
            Component::Time(component) => component.time_string.clone(),
            Component::Weather(component) => component.render(),
            Component::Temperature(component) => component.render(),
            Component::Cpu(component) => component.render(),
            Component::Ram(component) => component.render(),
            Component::Wifi(component) => component.render(),
            Component::Vpn(component) => component.render(),
            Component::Brightness(component) => component.render(),
            Component::Volume(component) => component.render(),
            Component::Battery(component) => component.render(),
            Component::Separator(component) => component.render(),
            Component::Space(component) => component.render(),
        }
    }

    pub fn is_muted(&self) -> bool {
        match self {
            Component::Volume(component) => component.is_muted,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component_names = vec![
            "time",
            "temperature",
            "cpu",
            "ram",
            "brightness",
            "volume",
            "separator",
            "space",
        ];
        for name in component_names {
            let result = Component::new(name);
            assert!(result.is_ok(), "Failed to create component: {}", name);
        }
    }

    #[tokio::test]
    async fn test_component_manager() {
        let manager = ComponentManager::new();
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        let left_components = manager.get_bar_components("left");
        assert!(!left_components.is_empty());

        let middle_components = manager.get_bar_components("middle");
        assert_eq!(middle_components.len(), 3); // time, separator, weather

        let right_components = manager.get_bar_components("right");
        assert_eq!(right_components.len(), 11); // 8 components + 3 separators
    }
}

#[derive(Debug)]
pub struct ComponentManager {
    components: HashMap<String, Component>,
    config: Config,
}

impl ComponentManager {
    pub fn new() -> color_eyre::Result<Self> {
        let config = Config::load()?;
        let mut components = HashMap::new();

        for component_name in &config.bars.left {
            if let Ok(component) = Component::new(component_name) {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.middle {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.right {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
                components.insert(component_name.clone(), component);
            }
        }

        Ok(Self { components, config })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        for component in self.components.values_mut() {
            component.update()?;
        }
        Ok(())
    }

    pub fn get_bar_components(&self, bar: &str) -> Vec<&Component> {
        if let Some(component_names) = self.config.get_components_for_bar(bar) {
            component_names
                .iter()
                .filter_map(|name| self.components.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }
}
