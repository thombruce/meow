use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone)]
pub struct Weather {
    pub temperature: String,
    pub condition: String,
    pub icon: String,
    last_update: u64,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<WeatherCondition>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherCondition {
    main: String,
}

impl Weather {
    pub fn new() -> Self {
        Self {
            temperature: "--".to_string(),
            condition: "Unknown".to_string(),
            icon: "󰖐".to_string(),
            last_update: 0,
        }
    }

    pub fn update(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update every 10 minutes to avoid API rate limits
        if now - self.last_update < 600 {
            return;
        }

        if let Ok(weather_data) = self.fetch_weather() {
            self.temperature = format!("{:.0}", weather_data.main.temp);
            self.condition = weather_data.weather[0].main.clone();
            self.icon = self.get_weather_icon(&self.condition);
            self.last_update = now;
        }
    }

    fn fetch_weather(&self) -> color_eyre::Result<WeatherResponse> {
        // Using a free weather API that doesn't require API key
        // Note: This uses wttr.in for current weather
        let url = "http://wttr.in/?format=j1";

        let response = reqwest::blocking::get(url)?;
        let json: serde_json::Value = response.json()?;

        // Parse wttr.in response format
        if let Some(current) = json["current_condition"].get(0) {
            let temp = current["temp_C"]
                .as_str()
                .unwrap_or("--")
                .parse::<f64>()
                .unwrap_or(0.0);
            let condition = current["weatherDesc"][0]["value"]
                .as_str()
                .unwrap_or("Unknown");

            return Ok(WeatherResponse {
                main: Main { temp },
                weather: vec![WeatherCondition {
                    main: condition.to_string(),
                }],
            });
        }

        Err(color_eyre::eyre::eyre!("Failed to parse weather data"))
    }

    fn get_weather_icon(&self, condition: &str) -> String {
        let condition_lower = condition.to_lowercase();
        match condition_lower.as_str() {
            cond if cond.contains("clear") || cond.contains("sunny") => "󰖙".to_string(),
            cond if cond.contains("cloud") || cond.contains("overcast") => "󰖐".to_string(),
            cond if cond.contains("rain") || cond.contains("drizzle") => "󰖗".to_string(),
            cond if cond.contains("snow") || cond.contains("sleet") => "󰖘".to_string(),
            cond if cond.contains("thunder") || cond.contains("storm") => "󰖓".to_string(),
            cond if cond.contains("fog") || cond.contains("mist") => "󰖑".to_string(),
            cond if cond.contains("wind") => "󰖝".to_string(),
            _ => "󰖐".to_string(),
        }
    }
}

