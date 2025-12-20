use std::time::{Duration, Instant};
use sysinfo::{MemoryRefreshKind, RefreshKind};

#[derive(Debug)]
pub struct Ram {
    pub usage: String,
    system: sysinfo::System,
    last_update: Instant,
    update_interval: Duration,
}

impl Ram {
    pub fn new() -> Self {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        Self {
            usage: "0".to_string(),
            system,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.system.refresh_memory();

            let mem_percent: u32 =
                (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as u32;
            self.usage = mem_percent.to_string();
            
            self.last_update = now;
        }
    }
}
