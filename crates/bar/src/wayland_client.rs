use crate::logging;
use std::sync::{Arc, Mutex};
use wayland_client::Connection;

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: String,
    pub title: String,
    pub class: String,
}

pub trait WaylandClient: Send + Sync {
    fn get_workspaces(&self) -> color_eyre::Result<Vec<WorkspaceInfo>>;
    fn get_windows(&self) -> color_eyre::Result<Vec<WindowInfo>>;
    fn get_active_workspace(&self) -> color_eyre::Result<Option<String>>;
    fn get_active_window(&self) -> color_eyre::Result<Option<String>>;
}

#[derive(Clone)]
pub struct WaylandManager {
    client: Arc<Mutex<Box<dyn WaylandClient>>>,
}

impl WaylandManager {
    pub fn new() -> color_eyre::Result<Self> {
        // Try to detect the compositor and use appropriate client
        let client = if Self::is_hyprland() {
            Box::new(HyprlandClient::new()?) as Box<dyn WaylandClient>
        } else if Self::is_sway() {
            Box::new(SwayClient::new()?) as Box<dyn WaylandClient>
        } else {
            // Fallback to generic Wayland protocols for basic window info
            Box::new(GenericWaylandClient::new()?) as Box<dyn WaylandClient>
        };

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    fn is_hyprland() -> bool {
        std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
    }

    fn is_sway() -> bool {
        std::env::var("SWAYSOCK").is_ok()
    }

    pub fn get_workspaces(&self) -> color_eyre::Result<Vec<WorkspaceInfo>> {
        self.client.lock().unwrap().get_workspaces()
    }

    pub fn get_windows(&self) -> color_eyre::Result<Vec<WindowInfo>> {
        self.client.lock().unwrap().get_windows()
    }

    pub fn get_active_workspace(&self) -> color_eyre::Result<Option<String>> {
        self.client.lock().unwrap().get_active_workspace()
    }

    pub fn get_active_window(&self) -> color_eyre::Result<Option<String>> {
        self.client.lock().unwrap().get_active_window()
    }
}

// Hyprland-specific implementation (using existing hyprctl commands as fallback)
struct HyprlandClient;

impl HyprlandClient {
    fn new() -> color_eyre::Result<Self> {
        Ok(Self)
    }
}

impl WaylandClient for HyprlandClient {
    fn get_workspaces(&self) -> color_eyre::Result<Vec<WorkspaceInfo>> {
        let output = std::process::Command::new("hyprctl")
            .args(["workspaces", "-j"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let workspaces: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

            Ok(workspaces
                .into_iter()
                .map(|w| WorkspaceInfo {
                    id: w["id"].as_i64().unwrap_or(0).to_string(),
                    name: w["name"].as_str().unwrap_or("").to_string(),
                    is_active: false, // We'll update this with active workspace call
                })
                .collect())
        } else {
            Err(color_eyre::eyre::eyre!(
                "Failed to get workspaces from hyprctl"
            ))
        }
    }

    fn get_windows(&self) -> color_eyre::Result<Vec<WindowInfo>> {
        let output = std::process::Command::new("hyprctl")
            .args(["clients", "-j"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let windows: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

            Ok(windows
                .into_iter()
                .filter_map(|w| {
                    let workspace_id = w["workspace"]["id"].as_i64()?;
                    if workspace_id <= 0 {
                        return None;
                    } // Filter out special workspaces

                    Some(WindowInfo {
                        id: w["address"].as_str()?.to_string(),
                        title: w["title"].as_str()?.to_string(),
                        class: w["class"].as_str()?.to_string(),
                    })
                })
                .collect())
        } else {
            Err(color_eyre::eyre::eyre!(
                "Failed to get windows from hyprctl"
            ))
        }
    }

    fn get_active_workspace(&self) -> color_eyre::Result<Option<String>> {
        let output = std::process::Command::new("hyprctl")
            .args(["activeworkspace", "-j"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let workspace: serde_json::Value = serde_json::from_str(&stdout)?;
            Ok(Some(workspace["id"].as_i64().unwrap_or(0).to_string()))
        } else {
            Ok(None)
        }
    }

    fn get_active_window(&self) -> color_eyre::Result<Option<String>> {
        let output = std::process::Command::new("hyprctl")
            .args(["activewindow", "-j"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let window: serde_json::Value = serde_json::from_str(&stdout)?;
            Ok(window["address"].as_str().map(|s| s.to_string()))
        } else {
            Ok(None)
        }
    }
}

// Sway/i3-specific implementation (using i3-msg)
struct SwayClient;

impl SwayClient {
    fn new() -> color_eyre::Result<Self> {
        Ok(Self)
    }
}

impl WaylandClient for SwayClient {
    fn get_workspaces(&self) -> color_eyre::Result<Vec<WorkspaceInfo>> {
        let output = std::process::Command::new("i3-msg")
            .args(["-t", "get_workspaces"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let workspaces: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

            Ok(workspaces
                .into_iter()
                .map(|w| WorkspaceInfo {
                    id: w["num"].as_i64().unwrap_or(0).to_string(),
                    name: w["name"].as_str().unwrap_or("").to_string(),
                    is_active: w["focused"].as_bool().unwrap_or(false),
                })
                .collect())
        } else {
            Err(color_eyre::eyre::eyre!(
                "Failed to get workspaces from i3-msg"
            ))
        }
    }

    fn get_windows(&self) -> color_eyre::Result<Vec<WindowInfo>> {
        let output = std::process::Command::new("i3-msg")
            .args(["-t", "get_tree"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let tree: serde_json::Value = serde_json::from_str(&stdout)?;

            let mut windows = Vec::new();
            self.extract_windows_from_node(&tree, &mut windows);
            Ok(windows)
        } else {
            Err(color_eyre::eyre::eyre!("Failed to get windows from i3-msg"))
        }
    }

    fn get_active_workspace(&self) -> color_eyre::Result<Option<String>> {
        let workspaces = self.get_workspaces()?;
        Ok(workspaces
            .iter()
            .find(|w| w.is_active)
            .map(|w| w.id.clone()))
    }

    fn get_active_window(&self) -> color_eyre::Result<Option<String>> {
        let output = std::process::Command::new("i3-msg")
            .args(["-t", "get_tree"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let tree: serde_json::Value = serde_json::from_str(&stdout)?;
            Ok(self.find_focused_window(&tree))
        } else {
            Ok(None)
        }
    }
}

impl SwayClient {
    fn extract_windows_from_node(&self, node: &serde_json::Value, windows: &mut Vec<WindowInfo>) {
        if let Some(window_props) = node.get("window_properties")
            && let (Some(id), Some(title), Some(class)) = (
                node["id"].as_i64(),
                window_props["title"].as_str(),
                window_props["class"].as_str(),
            )
        {
            windows.push(WindowInfo {
                id: id.to_string(),
                title: title.to_string(),
                class: class.to_string(),
            });
        }

        if let Some(nodes) = node.get("nodes").and_then(|n| n.as_array()) {
            for child in nodes {
                self.extract_windows_from_node(child, windows);
            }
        }

        if let Some(floating_nodes) = node.get("floating_nodes").and_then(|n| n.as_array()) {
            for child in floating_nodes {
                self.extract_windows_from_node(child, windows);
            }
        }
    }

    fn find_focused_window(&self, node: &serde_json::Value) -> Option<String> {
        if node["focused"].as_bool() == Some(true) && node.get("window_properties").is_some() {
            node["id"].as_i64().map(|id| id.to_string())
        } else {
            // Check children
            if let Some(nodes) = node.get("nodes").and_then(|n| n.as_array()) {
                for child in nodes {
                    if let Some(id) = self.find_focused_window(child) {
                        return Some(id);
                    }
                }
            }

            // Check floating nodes
            if let Some(floating_nodes) = node.get("floating_nodes").and_then(|n| n.as_array()) {
                for child in floating_nodes {
                    if let Some(id) = self.find_focused_window(child) {
                        return Some(id);
                    }
                }
            }

            None
        }
    }
}

// Generic Wayland client for basic window information
struct GenericWaylandClient {
    _connection: Connection,
}

impl GenericWaylandClient {
    fn new() -> color_eyre::Result<Self> {
        let connection = Connection::connect_to_env()?;
        Ok(Self {
            _connection: connection,
        })
    }
}

impl WaylandClient for GenericWaylandClient {
    fn get_workspaces(&self) -> color_eyre::Result<Vec<WorkspaceInfo>> {
        // Generic Wayland doesn't have standardized workspace management
        // Return empty for now - this could be extended with compositor-specific protocols
        Ok(Vec::new())
    }

    fn get_windows(&self) -> color_eyre::Result<Vec<WindowInfo>> {
        // Generic Wayland client can't get detailed window info without compositor-specific protocols
        // This would need to be extended with protocols like wlr-foreign-toplevel
        logging::log_component_error(
            "WAYLAND",
            "Generic Wayland client doesn't support detailed window information",
        );
        Ok(Vec::new())
    }

    fn get_active_workspace(&self) -> color_eyre::Result<Option<String>> {
        Ok(None)
    }

    fn get_active_window(&self) -> color_eyre::Result<Option<String>> {
        Ok(None)
    }
}
