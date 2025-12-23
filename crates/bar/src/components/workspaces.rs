use crate::wayland_client::WaylandManager;
use ratatui::{prelude::Stylize, style::Color, text::Span};

#[derive(Clone)]
pub struct Workspaces {
    wayland_manager: WaylandManager,
    pub workspaces: Vec<String>,
    pub active_workspace: String,
}

impl std::fmt::Debug for Workspaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Workspaces")
            .field("workspaces", &self.workspaces)
            .field("active_workspace", &self.active_workspace)
            .finish()
    }
}

impl Workspaces {
    pub fn new() -> color_eyre::Result<Self> {
        let wayland_manager = WaylandManager::new()?;
        let mut instance = Self {
            wayland_manager,
            workspaces: Vec::new(),
            active_workspace: String::new(),
        };
        instance.update()?;
        Ok(instance)
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        // Get workspaces from Wayland manager
        let workspace_infos = self.wayland_manager.get_workspaces().unwrap_or_default();
        self.workspaces = workspace_infos.iter().map(|w| w.name.clone()).collect();

        // Get active workspace
        if let Some(active_id) = self
            .wayland_manager
            .get_active_workspace()
            .unwrap_or_default()
        {
            self.active_workspace = workspace_infos
                .iter()
                .find(|w| w.id == active_id)
                .map(|w| w.name.clone())
                .unwrap_or_default();
        }

        Ok(())
    }

    pub fn render(&self) -> Vec<Span<'_>> {
        let rainbow_colors = [
            Color::Red,      // 1
            Color::Yellow,   // 2
            Color::Green,    // 3
            Color::Cyan,     // 4
            Color::Blue,     // 5
            Color::Magenta,  // 6
            Color::LightRed, // 7
        ];

        self.workspaces
            .iter()
            .map(|w| {
                if w == &self.active_workspace {
                    if let Ok(workspace_num) = w.parse::<usize>() {
                        let color_index = (workspace_num - 1) % rainbow_colors.len();
                        let bg_color = rainbow_colors[color_index];
                        // Use black text for better readability on all colored backgrounds
                        Span::raw(format!(" {} ", w)).bg(bg_color).fg(Color::Black)
                    } else {
                        // Fallback for non-numeric workspace names
                        Span::raw(format!(" {} ", w))
                            .bg(Color::White)
                            .fg(Color::Black)
                    }
                } else if let Ok(workspace_num) = w.parse::<usize>() {
                    let color_index = (workspace_num - 1) % rainbow_colors.len();
                    let color = rainbow_colors[color_index];
                    Span::raw(format!(" {} ", w)).fg(color)
                } else {
                    Span::raw(format!(" {} ", w))
                }
            })
            .collect::<Vec<Span>>()
    }
}
