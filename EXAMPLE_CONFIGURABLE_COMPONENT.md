# Example: Adding Battery Component Configuration

This document demonstrates how easy it is to add configuration support to any component using the new macro-based registration system.

## Steps to Make Battery Component Configurable

### 1. Add Configuration Structure
```rust
// In crates/bar/src/components/battery.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryConfig {
    pub component: String,
    #[serde(default)]
    pub show_percentage: bool,
    #[serde(default)]
    pub show_charging_status: bool,
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

fn default_update_interval() -> u64 {
    30  // Check battery every 30 seconds
}
```

### 2. Implement ConfigurableComponent Trait
```rust
// In crates/bar/src/components/battery.rs
use crate::components::ConfigurableComponent;

impl ConfigurableComponent for Battery {
    type Config = BatteryConfig;

    fn from_config(config: Self::Config) -> color_eyre::Result<Self> {
        let mut battery = Self::new()?;
        
        // Apply configuration options
        if !config.show_percentage {
            // Customize display logic
        }
        
        battery.update_interval = Duration::from_secs(config.update_interval);
        
        Ok(battery)
    }
}
```

### 3. Add to Component Registry
```rust
// In crates/bar/src/components/mod.rs
// Simply uncomment the battery line in the macro:
define_configurable_components!(
    Wifi => "wifi",
    Battery => "battery",  // <- Uncomment this line
    // Cpu => "cpu",
    // Ram => "ram",
);
```

### 4. That's It! 🎉

No need to:
- ✅ No manual From trait implementations (macro handles this)
- ✅ No manual registration code (macro handles this)  
- ✅ No factory functions to write (macro handles this)
- ✅ No registry updates needed (macro handles this)

## Result

Users can now configure the battery component with:
```json
{
  "component": "battery",
  "show_percentage": true,
  "show_charging_status": true,
  "update_interval": 30
}
```

Or keep using the simple string:
```json
{
  "right": ["battery"]
}
```

## Benefits

1. **Zero Boilerplate**: The macro eliminates all repetitive code
2. **Type Safety**: Compile-time verification of component registration
3. **Extensibility**: Easy to add plugin support later
4. **Maintainability**: Single place to manage all configurable components
5. **Performance**: Zero-cost abstractions, no runtime overhead

This demonstrates the power and elegance of the new macro-based registration system!