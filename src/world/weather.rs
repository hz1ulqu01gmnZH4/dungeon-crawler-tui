use crate::world::{Season, TerrainType};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Weather conditions
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,          // Perfect conditions
    PartlyCloudy,   // Some clouds
    Overcast,       // Heavy clouds, no precipitation
    LightRain,      // Light rain
    HeavyRain,      // Heavy rain, visibility reduced
    Thunderstorm,   // Dangerous storms
    LightSnow,      // Light snowfall
    HeavySnow,      // Blizzard conditions
    Fog,            // Dense fog, visibility heavily reduced
    Windy,          // High winds
    Heatwave,       // Extreme heat
}

impl Weather {
    pub fn name(&self) -> &'static str {
        match self {
            Weather::Clear => "Clear Skies",
            Weather::PartlyCloudy => "Partly Cloudy",
            Weather::Overcast => "Overcast",
            Weather::LightRain => "Light Rain",
            Weather::HeavyRain => "Heavy Rain",
            Weather::Thunderstorm => "Thunderstorm",
            Weather::LightSnow => "Light Snow",
            Weather::HeavySnow => "Heavy Snow",
            Weather::Fog => "Dense Fog",
            Weather::Windy => "Windy",
            Weather::Heatwave => "Heatwave",
        }
    }

    /// Visibility modifier (-5 to 0, where negative reduces sight range)
    pub fn visibility_modifier(&self) -> i32 {
        match self {
            Weather::Clear => 0,
            Weather::PartlyCloudy => 0,
            Weather::Overcast => -1,
            Weather::LightRain => -1,
            Weather::HeavyRain => -2,
            Weather::Thunderstorm => -3,
            Weather::LightSnow => -1,
            Weather::HeavySnow => -3,
            Weather::Fog => -5,
            Weather::Windy => -1,
            Weather::Heatwave => 0,
        }
    }

    /// Travel speed modifier (0.5 = half speed, 1.0 = normal, 1.2 = faster)
    pub fn travel_speed_modifier(&self) -> f32 {
        match self {
            Weather::Clear => 1.0,
            Weather::PartlyCloudy => 1.0,
            Weather::Overcast => 0.95,
            Weather::LightRain => 0.9,
            Weather::HeavyRain => 0.7,
            Weather::Thunderstorm => 0.6,
            Weather::LightSnow => 0.85,
            Weather::HeavySnow => 0.5,
            Weather::Fog => 0.75,
            Weather::Windy => 0.9,
            Weather::Heatwave => 0.85,
        }
    }

    /// HP cost per hour of travel (negative = damage, 0 = no effect)
    pub fn hp_per_hour(&self) -> i32 {
        match self {
            Weather::Clear | Weather::PartlyCloudy => 0,
            Weather::Overcast | Weather::Windy => 0,
            Weather::LightRain | Weather::LightSnow => -1,
            Weather::HeavyRain => -2,
            Weather::HeavySnow => -3,
            Weather::Thunderstorm => -2,
            Weather::Fog => 0,
            Weather::Heatwave => -2,
        }
    }

    /// Whether this weather is dangerous
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            Weather::Thunderstorm | Weather::HeavySnow | Weather::Heatwave
        )
    }

    /// Weather description for atmospheric flavor
    pub fn description(&self) -> &'static str {
        match self {
            Weather::Clear => "The sky is clear and blue.",
            Weather::PartlyCloudy => "Scattered clouds drift across the sky.",
            Weather::Overcast => "Gray clouds blanket the sky.",
            Weather::LightRain => "A gentle rain falls steadily.",
            Weather::HeavyRain => "Rain pours down heavily, soaking everything.",
            Weather::Thunderstorm => "Thunder crashes as lightning splits the sky!",
            Weather::LightSnow => "Soft snowflakes drift down from above.",
            Weather::HeavySnow => "A fierce blizzard rages, obscuring everything!",
            Weather::Fog => "Dense fog surrounds you, limiting visibility.",
            Weather::Windy => "Strong winds buffet you from all sides.",
            Weather::Heatwave => "The oppressive heat beats down mercilessly.",
        }
    }
}

/// Weather system that generates and tracks current weather
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherSystem {
    pub current_weather: Weather,
    weather_duration: i32,  // Minutes until weather changes
    last_update_time: i64,  // Total minutes when last updated
}

impl WeatherSystem {
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let weather = Self::generate_initial_weather(&mut rng);

        Self {
            current_weather: weather,
            weather_duration: Self::generate_duration(&mut rng, weather),
            last_update_time: 0,
        }
    }

    /// Update weather based on current time
    pub fn update(
        &mut self,
        current_time_minutes: i64,
        season: Season,
        terrain: TerrainType,
        rng: &mut StdRng,
    ) {
        let elapsed = current_time_minutes - self.last_update_time;

        // Check if weather should change
        if elapsed >= self.weather_duration as i64 {
            self.current_weather = Self::generate_weather(rng, season, terrain, self.current_weather);
            self.weather_duration = Self::generate_duration(rng, self.current_weather);
            self.last_update_time = current_time_minutes;
        }
    }

    /// Generate initial weather
    fn generate_initial_weather(rng: &mut StdRng) -> Weather {
        // Bias toward clear weather at start
        let roll = rng.gen::<f32>();
        if roll < 0.6 {
            Weather::Clear
        } else if roll < 0.8 {
            Weather::PartlyCloudy
        } else {
            Weather::Overcast
        }
    }

    /// Generate weather based on season, terrain, and previous weather
    fn generate_weather(
        rng: &mut StdRng,
        season: Season,
        terrain: TerrainType,
        previous: Weather,
    ) -> Weather {
        // Weather tends to persist or transition gradually
        let transition_roll = rng.gen::<f32>();

        // 40% chance to keep same weather
        if transition_roll < 0.4 {
            return previous;
        }

        // Season influences weather probability
        let season_weights = match season {
            Season::Spring => vec![
                (Weather::Clear, 0.25),
                (Weather::PartlyCloudy, 0.25),
                (Weather::Overcast, 0.15),
                (Weather::LightRain, 0.20),
                (Weather::HeavyRain, 0.10),
                (Weather::Fog, 0.05),
            ],
            Season::Summer => vec![
                (Weather::Clear, 0.40),
                (Weather::PartlyCloudy, 0.20),
                (Weather::Heatwave, 0.15),
                (Weather::Thunderstorm, 0.10),
                (Weather::Windy, 0.10),
                (Weather::Overcast, 0.05),
            ],
            Season::Autumn => vec![
                (Weather::PartlyCloudy, 0.25),
                (Weather::Overcast, 0.20),
                (Weather::LightRain, 0.20),
                (Weather::Windy, 0.15),
                (Weather::Fog, 0.10),
                (Weather::Clear, 0.10),
            ],
            Season::Winter => vec![
                (Weather::Overcast, 0.25),
                (Weather::LightSnow, 0.25),
                (Weather::HeavySnow, 0.15),
                (Weather::Clear, 0.15),
                (Weather::Windy, 0.10),
                (Weather::Fog, 0.10),
            ],
        };

        // Terrain modifiers
        let terrain_modifier = match terrain {
            TerrainType::Mountains => 1.5,  // More extreme weather
            TerrainType::Swamp => 1.3,      // More fog and rain
            TerrainType::Forest => 1.1,     // Slightly more rain
            TerrainType::DenseForest => 1.2,
            _ => 1.0,
        };

        // Select weather based on weights
        let total_weight: f32 = season_weights.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen::<f32>() * total_weight * terrain_modifier;

        for (weather, weight) in season_weights {
            roll -= weight;
            if roll <= 0.0 {
                return weather;
            }
        }

        // Fallback
        Weather::Clear
    }

    /// Generate duration for weather in minutes (1-6 hours)
    fn generate_duration(rng: &mut StdRng, weather: Weather) -> i32 {
        let base_duration = match weather {
            Weather::Clear => rng.gen_range(180..=360),          // 3-6 hours
            Weather::PartlyCloudy => rng.gen_range(120..=300),   // 2-5 hours
            Weather::Overcast => rng.gen_range(120..=240),       // 2-4 hours
            Weather::LightRain => rng.gen_range(60..=180),       // 1-3 hours
            Weather::HeavyRain => rng.gen_range(30..=120),       // 30min-2 hours
            Weather::Thunderstorm => rng.gen_range(20..=60),     // 20-60 minutes
            Weather::LightSnow => rng.gen_range(60..=180),       // 1-3 hours
            Weather::HeavySnow => rng.gen_range(30..=90),        // 30min-1.5 hours
            Weather::Fog => rng.gen_range(60..=240),             // 1-4 hours
            Weather::Windy => rng.gen_range(120..=300),          // 2-5 hours
            Weather::Heatwave => rng.gen_range(240..=480),       // 4-8 hours
        };

        base_duration
    }

    /// Get a message about weather change
    pub fn weather_change_message(&self) -> String {
        format!("The weather changes. {}", self.current_weather.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_properties() {
        let clear = Weather::Clear;
        assert_eq!(clear.visibility_modifier(), 0);
        assert_eq!(clear.travel_speed_modifier(), 1.0);
        assert!(!clear.is_dangerous());

        let storm = Weather::Thunderstorm;
        assert_eq!(storm.visibility_modifier(), -3);
        assert!(storm.travel_speed_modifier() < 1.0);
        assert!(storm.is_dangerous());
    }

    #[test]
    fn test_fog_visibility() {
        let fog = Weather::Fog;
        assert_eq!(fog.visibility_modifier(), -5);
    }

    #[test]
    fn test_weather_system_creation() {
        let system = WeatherSystem::new(12345);
        assert!(!system.current_weather.name().is_empty());
        assert!(system.weather_duration > 0);
    }

    #[test]
    fn test_weather_persistence() {
        let mut system = WeatherSystem::new(99999);
        let mut rng = StdRng::seed_from_u64(99999);

        let initial_weather = system.current_weather;

        // Small time increment shouldn't change weather
        system.update(30, Season::Spring, TerrainType::Plains, &mut rng);
        assert_eq!(system.current_weather, initial_weather);
    }

    #[test]
    fn test_seasonal_weather() {
        let mut rng = StdRng::seed_from_u64(54321);

        // Summer should not generate snow
        let summer_weather = WeatherSystem::generate_weather(
            &mut rng,
            Season::Summer,
            TerrainType::Plains,
            Weather::Clear,
        );
        assert_ne!(summer_weather, Weather::HeavySnow);
        assert_ne!(summer_weather, Weather::LightSnow);

        // Winter should not generate heatwave
        let winter_weather = WeatherSystem::generate_weather(
            &mut rng,
            Season::Winter,
            TerrainType::Plains,
            Weather::Clear,
        );
        assert_ne!(winter_weather, Weather::Heatwave);
    }

    #[test]
    fn test_weather_hp_effects() {
        assert_eq!(Weather::Clear.hp_per_hour(), 0);
        assert!(Weather::Thunderstorm.hp_per_hour() < 0);
        assert!(Weather::HeavySnow.hp_per_hour() < 0);
    }
}
