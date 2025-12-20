use std::process::Command;

#[derive(Debug)]
pub struct Volume {
    pub level: String,
}

impl Volume {
    pub fn new() -> Self {
        Self {
            level: get_system_volume().unwrap_or(0).to_string(),
        }
    }

    pub fn update(&mut self) {
        self.level = get_system_volume().unwrap_or(0).to_string();
    }
}

fn get_system_volume() -> Option<i32> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .expect("failed to get volume");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let parts: Vec<&str> = stdout.trim().split_whitespace().collect();

        if let Ok(volume) = parts[1].parse::<f32>() {
            return Some((volume * 100.0) as i32);
        }

        eprintln!("Failed to parse volume from output: {}", stdout);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    Some(0)
}
