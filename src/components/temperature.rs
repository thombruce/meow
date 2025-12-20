use sysinfo::Components;

#[derive(Debug)]
pub struct Temperature {
    pub value: String,
    components: Components,
}

impl Temperature {
    pub fn new() -> Self {
        let components = Components::new();

        Self {
            value: "0".to_string(),
            components,
        }
    }

    pub fn update(&mut self) {
        self.components.refresh(true);

        if let Some(component) = self.components.iter().find(|c| {
            c.label().to_lowercase().contains("cpu")
                || c.label().to_lowercase().contains("core")
                || c.label().to_lowercase().contains("package")
        }) && let Some(temp) = component.temperature()
        {
            self.value = format!("{:.0}", temp);
        }
    }
}
