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

    fn component_name() -> &'static str;
}

#[macro_export]
macro_rules! try_all_configurable_components {
    ($value:expr, $($component_type:ty => $enum_variant:ident),*) => {
        $(
            if <$component_type>::component_name() == extract_component_name($value) {
                if let Ok(config) = serde_json::from_value::<<$component_type as ConfigurableComponent>::Config>($value.clone()) {
                    if let Ok(component) = <$component_type>::from_config(config) {
                        return Some(Ok(Component::$enum_variant(component)));
                    }
                }
            }
        )*
    };
}

pub fn extract_component_name(value: &serde_json::Value) -> String {
    if let Some(obj) = value.as_object()
        && let Some(component) = obj.get("component").and_then(|c| c.as_str())
    {
        return component.to_string();
    }
    "unknown".to_string()
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
