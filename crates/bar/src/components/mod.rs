use serde::de::DeserializeOwned;

pub mod battery;
pub mod brightness;
pub mod cpu;
pub mod error_icon;
pub mod left_bar;
pub mod middle_bar;
pub mod ram;
pub mod right_bar;
pub mod separator;
pub mod space;
pub mod temperature;
pub mod time;
pub mod volume;
pub mod weather;
pub mod wifi;
pub mod windows;
pub mod workspaces;

pub trait ConfigurableComponent {
    type Config: DeserializeOwned;

    fn from_config(config: Self::Config) -> color_eyre::Result<Self>
    where
        Self: Sized;
}

type ComponentFactory = std::boxed::Box<
    dyn Fn(&serde_json::Value) -> Option<color_eyre::Result<crate::component_manager::Component>>,
>;

pub struct ConfigurableComponentRegistry {
    factories: std::collections::HashMap<String, ComponentFactory>,
}

impl ConfigurableComponentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: std::collections::HashMap::new(),
        };

        // Auto-register all built-in configurable components
        registry.register_wifi();

        registry
    }

    pub fn register_wifi(&mut self) {
        let factory: ComponentFactory = std::boxed::Box::new(|value: &serde_json::Value| {
            if let Ok(config) =
                serde_json::from_value::<<Wifi as ConfigurableComponent>::Config>(value.clone())
                && let Ok(wifi) = Wifi::from_config(config)
            {
                return Some(Ok(crate::component_manager::Component::Wifi(wifi)));
            }
            None
        });
        self.factories.insert("wifi".to_string(), factory);
    }

    // Future: Add generic register method for plugin support
    // This would allow runtime registration of new configurable components

    pub fn try_create(
        &self,
        component_name: &str,
        value: &serde_json::Value,
    ) -> Option<color_eyre::Result<crate::component_manager::Component>> {
        if let Some(factory) = self.factories.get(component_name) {
            factory(value)
        } else {
            None
        }
    }

    pub fn extract_component_name(value: &serde_json::Value) -> String {
        if let Some(obj) = value.as_object()
            && let Some(component) = obj.get("component").and_then(|c| c.as_str())
        {
            return component.to_string();
        }
        "unknown".to_string()
    }
}

pub use battery::Battery;
pub use brightness::Brightness;
pub use cpu::Cpu;
pub use error_icon::ErrorIcon;
pub use left_bar::LeftBar;
pub use middle_bar::MiddleBar;
pub use ram::Ram;
pub use right_bar::RightBar;
pub use separator::Separator;
pub use space::Space;
pub use temperature::Temperature;
pub use time::Time;
pub use volume::Volume;
pub use weather::Weather;
pub use wifi::Wifi;
pub use windows::Windows;
pub use workspaces::Workspaces;
