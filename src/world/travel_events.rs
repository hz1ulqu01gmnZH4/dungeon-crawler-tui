use crate::world::{TerrainType, TimeOfDay};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of travel events that can occur on the overmap
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TravelEventType {
    // Positive events
    FindHerbs,          // Find medicinal herbs (heal HP)
    FindTreasure,       // Find small amount of gold/items
    MeetTrader,         // Encounter traveling merchant
    FindCache,          // Discover hidden supply cache
    PleasantWeather,    // Good weather, morale boost

    // Neutral/informative events
    SpotLandmark,       // Notice a distant landmark
    AnimalSighting,     // See wildlife passing by
    FindTracks,         // Find tracks of creatures/people
    HearSounds,         // Hear distant sounds
    Nothing,            // Uneventful travel

    // Negative events
    HostileEncounter,   // Combat encounter
    BadWeather,         // Harsh weather (damage/delays)
    GetLost,            // Lose time wandering
    InjuredAnimal,      // Encounter wounded aggressive beast
    Ambush,             // Bandit/monster ambush
}

impl TravelEventType {
    pub fn name(&self) -> &'static str {
        match self {
            TravelEventType::FindHerbs => "Find Herbs",
            TravelEventType::FindTreasure => "Find Treasure",
            TravelEventType::MeetTrader => "Traveling Merchant",
            TravelEventType::FindCache => "Supply Cache",
            TravelEventType::PleasantWeather => "Pleasant Weather",
            TravelEventType::SpotLandmark => "Spot Landmark",
            TravelEventType::AnimalSighting => "Animal Sighting",
            TravelEventType::FindTracks => "Find Tracks",
            TravelEventType::HearSounds => "Hear Sounds",
            TravelEventType::Nothing => "Uneventful Travel",
            TravelEventType::HostileEncounter => "Hostile Encounter",
            TravelEventType::BadWeather => "Bad Weather",
            TravelEventType::GetLost => "Got Lost",
            TravelEventType::InjuredAnimal => "Wounded Beast",
            TravelEventType::Ambush => "Ambush",
        }
    }

    pub fn is_hostile(&self) -> bool {
        matches!(
            self,
            TravelEventType::HostileEncounter
                | TravelEventType::InjuredAnimal
                | TravelEventType::Ambush
        )
    }

    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            TravelEventType::FindHerbs
                | TravelEventType::FindTreasure
                | TravelEventType::MeetTrader
                | TravelEventType::FindCache
                | TravelEventType::PleasantWeather
        )
    }
}

/// A travel event that occurred
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TravelEvent {
    pub event_type: TravelEventType,
    pub message: String,
    pub hp_change: i32,      // Positive = heal, negative = damage
    pub gold_change: i32,    // Gold gained or lost
    pub time_cost: i32,      // Additional time cost in minutes
}

impl TravelEvent {
    pub fn new(event_type: TravelEventType, message: String) -> Self {
        Self {
            event_type,
            message,
            hp_change: 0,
            gold_change: 0,
            time_cost: 0,
        }
    }

    pub fn with_hp(mut self, hp: i32) -> Self {
        self.hp_change = hp;
        self
    }

    pub fn with_gold(mut self, gold: i32) -> Self {
        self.gold_change = gold;
        self
    }

    pub fn with_time(mut self, time: i32) -> Self {
        self.time_cost = time;
        self
    }
}

/// Event probability calculator based on context
pub struct TravelEventChance {
    base_chance: f32,               // Base chance of any event (0.0-1.0)
    pub danger_multiplier: f32,     // Multiplier for dangerous events
}

impl TravelEventChance {
    pub fn new() -> Self {
        Self {
            base_chance: 0.15,      // 15% base chance per move
            danger_multiplier: 1.0,
        }
    }

    /// Calculate danger multiplier based on context
    pub fn calculate_danger(
        &mut self,
        terrain: TerrainType,
        time_of_day: TimeOfDay,
        distance_to_nearest_settlement: i32,
    ) {
        // Base danger from terrain
        let terrain_danger = match terrain {
            TerrainType::Plains => 0.5,
            TerrainType::Forest => 1.0,
            TerrainType::DenseForest => 1.3,
            TerrainType::Hills => 1.2,
            TerrainType::Mountains => 1.8,
            TerrainType::Swamp => 1.5,
            TerrainType::Lake | TerrainType::River => 0.3,
            TerrainType::Road | TerrainType::KingRoad | TerrainType::TradePath => 0.4,
            _ => 0.5,
        };

        // Time of day modifier
        let time_danger = match time_of_day {
            TimeOfDay::DeepNight | TimeOfDay::Night => 1.8,  // Much more dangerous at night
            TimeOfDay::Dawn | TimeOfDay::Dusk => 1.3,
            TimeOfDay::Day => 1.0,
        };

        // Distance from safety
        let distance_danger = 1.0 + (distance_to_nearest_settlement as f32 * 0.05).min(1.0);

        self.danger_multiplier = terrain_danger * time_danger * distance_danger;
    }

    pub fn should_trigger_event(&self, rng: &mut StdRng) -> bool {
        rng.gen::<f32>() < self.base_chance
    }
}

/// Travel event generator
pub struct TravelEventGenerator {
    seed: u64,
}

impl TravelEventGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate a random travel event based on context
    pub fn generate_event(
        &self,
        rng: &mut StdRng,
        terrain: TerrainType,
        time_of_day: TimeOfDay,
        danger_multiplier: f32,
    ) -> TravelEvent {
        let danger_chance = (danger_multiplier * 0.3).min(0.7); // Max 70% danger

        // Roll for event category
        let roll = rng.gen::<f32>();

        if roll < danger_chance {
            self.generate_hostile_event(rng, terrain, time_of_day)
        } else if roll < danger_chance + 0.25 {
            self.generate_positive_event(rng, terrain)
        } else {
            self.generate_neutral_event(rng, terrain)
        }
    }

    fn generate_hostile_event(
        &self,
        rng: &mut StdRng,
        terrain: TerrainType,
        time_of_day: TimeOfDay,
    ) -> TravelEvent {
        let events = vec![
            TravelEventType::HostileEncounter,
            TravelEventType::InjuredAnimal,
            TravelEventType::Ambush,
        ];
        let event_type = *events.choose(rng).unwrap();

        let (message, hp_damage) = match event_type {
            TravelEventType::HostileEncounter => {
                let creature = Self::get_creature_for_terrain(terrain);
                let damage = rng.gen_range(2..=8);
                (
                    format!("You encounter {}! You fight them off but sustain injuries.", creature),
                    damage,
                )
            }
            TravelEventType::InjuredAnimal => {
                let animal = Self::get_animal_for_terrain(terrain);
                let damage = rng.gen_range(1..=5);
                (
                    format!("A wounded {} attacks you in desperation!", animal),
                    damage,
                )
            }
            TravelEventType::Ambush => {
                let damage = rng.gen_range(3..=10);
                let time = rng.gen_range(10..=30);
                return TravelEvent::new(
                    event_type,
                    "Bandits ambush you from hiding! You fight them off but it takes time.".to_string(),
                )
                .with_hp(-damage)
                .with_time(time);
            }
            _ => unreachable!(),
        };

        TravelEvent::new(event_type, message).with_hp(-hp_damage)
    }

    fn generate_positive_event(&self, rng: &mut StdRng, terrain: TerrainType) -> TravelEvent {
        let events = vec![
            TravelEventType::FindHerbs,
            TravelEventType::FindTreasure,
            TravelEventType::FindCache,
            TravelEventType::PleasantWeather,
        ];
        let event_type = *events.choose(rng).unwrap();

        match event_type {
            TravelEventType::FindHerbs => {
                let hp = rng.gen_range(3..=8);
                TravelEvent::new(
                    event_type,
                    format!("You find medicinal herbs and use them to treat wounds. (+{} HP)", hp),
                )
                .with_hp(hp)
            }
            TravelEventType::FindTreasure => {
                let gold = rng.gen_range(10..=50);
                TravelEvent::new(
                    event_type,
                    format!("You discover a small pouch of coins! (+{} gold)", gold),
                )
                .with_gold(gold)
            }
            TravelEventType::FindCache => {
                let gold = rng.gen_range(20..=80);
                let hp = rng.gen_range(2..=5);
                TravelEvent::new(
                    event_type,
                    format!(
                        "You find a hidden supply cache! (+{} gold, +{} HP from supplies)",
                        gold, hp
                    ),
                )
                .with_gold(gold)
                .with_hp(hp)
            }
            TravelEventType::PleasantWeather => TravelEvent::new(
                event_type,
                "The weather is perfect for travel. You feel energized!".to_string(),
            )
            .with_hp(1),
            _ => unreachable!(),
        }
    }

    fn generate_neutral_event(&self, rng: &mut StdRng, terrain: TerrainType) -> TravelEvent {
        let events = vec![
            TravelEventType::SpotLandmark,
            TravelEventType::AnimalSighting,
            TravelEventType::FindTracks,
            TravelEventType::HearSounds,
            TravelEventType::Nothing,
        ];
        let event_type = *events.choose(rng).unwrap();

        let message = match event_type {
            TravelEventType::SpotLandmark => {
                let landmarks = Self::get_landmarks_for_terrain(terrain);
                let landmark = landmarks.choose(rng).unwrap();
                format!("You spot {} in the distance.", landmark)
            }
            TravelEventType::AnimalSighting => {
                let animal = Self::get_animal_for_terrain(terrain);
                format!("You see {} passing by peacefully.", animal)
            }
            TravelEventType::FindTracks => {
                let creature = Self::get_creature_for_terrain(terrain);
                format!("You notice {} tracks. They appear to be several hours old.", creature)
            }
            TravelEventType::HearSounds => {
                let sounds = Self::get_sounds_for_terrain(terrain);
                let sound = sounds.choose(rng).unwrap();
                format!("You hear {} in the distance.", sound)
            }
            TravelEventType::Nothing => "Your journey continues uneventfully.".to_string(),
            _ => unreachable!(),
        };

        TravelEvent::new(event_type, message)
    }

    fn get_creature_for_terrain(terrain: TerrainType) -> &'static str {
        match terrain {
            TerrainType::Forest | TerrainType::DenseForest => "a pack of wolves",
            TerrainType::Mountains => "mountain trolls",
            TerrainType::Plains => "highway bandits",
            TerrainType::Swamp => "swamp creatures",
            TerrainType::Hills => "hill goblins",
            TerrainType::River | TerrainType::Lake => "river pirates",
            _ => "hostile creatures",
        }
    }

    fn get_animal_for_terrain(terrain: TerrainType) -> &'static str {
        match terrain {
            TerrainType::Forest | TerrainType::DenseForest => "a bear",
            TerrainType::Mountains => "a mountain goat",
            TerrainType::Plains => "a wild horse",
            TerrainType::Swamp => "a crocodile",
            TerrainType::Hills => "a wild boar",
            TerrainType::River | TerrainType::Lake => "a fish",
            _ => "an animal",
        }
    }

    fn get_landmarks_for_terrain(terrain: TerrainType) -> Vec<&'static str> {
        match terrain {
            TerrainType::Forest => vec![
                "a massive ancient tree",
                "a clearing with standing stones",
                "a forest shrine",
            ],
            TerrainType::Mountains => vec![
                "a towering peak",
                "a mountain pass",
                "a waterfall cascading down cliffs",
            ],
            TerrainType::Plains => vec![
                "a lone windmill",
                "an abandoned watchtower",
                "a circle of standing stones",
            ],
            TerrainType::Hills => vec!["a hill fort ruins", "a shepherd's hut", "a stone marker"],
            _ => vec!["an unusual rock formation", "a weathered monument"],
        }
    }

    fn get_sounds_for_terrain(terrain: TerrainType) -> Vec<&'static str> {
        match terrain {
            TerrainType::Forest => vec!["birds singing", "wolves howling", "rustling leaves"],
            TerrainType::Mountains => vec![
                "wind whistling through peaks",
                "distant rockslides",
                "eagles crying",
            ],
            TerrainType::Plains => vec!["distant thunder", "wind through grass", "cattle lowing"],
            TerrainType::Swamp => vec![
                "croaking frogs",
                "bubbling water",
                "strange splashing",
            ],
            _ => vec!["mysterious sounds", "the wind", "distant echoes"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_types() {
        let event = TravelEventType::FindTreasure;
        assert_eq!(event.name(), "Find Treasure");
        assert!(event.is_positive());
        assert!(!event.is_hostile());

        let hostile = TravelEventType::Ambush;
        assert!(hostile.is_hostile());
        assert!(!hostile.is_positive());
    }

    #[test]
    fn test_travel_event_creation() {
        let event = TravelEvent::new(
            TravelEventType::FindHerbs,
            "Found herbs".to_string(),
        )
        .with_hp(5)
        .with_gold(10)
        .with_time(15);

        assert_eq!(event.hp_change, 5);
        assert_eq!(event.gold_change, 10);
        assert_eq!(event.time_cost, 15);
    }

    #[test]
    fn test_event_chance_calculation() {
        let mut chance = TravelEventChance::new();

        // Test safe conditions
        chance.calculate_danger(TerrainType::Plains, TimeOfDay::Day, 2);
        assert!(chance.danger_multiplier < 1.5);

        // Test dangerous conditions
        chance.calculate_danger(TerrainType::Mountains, TimeOfDay::Night, 20);
        assert!(chance.danger_multiplier > 2.0);
    }

    #[test]
    fn test_event_generation() {
        let generator = TravelEventGenerator::new(12345);
        let mut rng = StdRng::seed_from_u64(12345);

        let event = generator.generate_event(
            &mut rng,
            TerrainType::Forest,
            TimeOfDay::Night,
            2.0,
        );

        assert!(!event.message.is_empty());
    }

    #[test]
    fn test_deterministic_generation() {
        let generator = TravelEventGenerator::new(99999);
        let mut rng1 = StdRng::seed_from_u64(99999);
        let mut rng2 = StdRng::seed_from_u64(99999);

        let event1 = generator.generate_event(
            &mut rng1,
            TerrainType::Plains,
            TimeOfDay::Day,
            1.0,
        );
        let event2 = generator.generate_event(
            &mut rng2,
            TerrainType::Plains,
            TimeOfDay::Day,
            1.0,
        );

        assert_eq!(event1.event_type, event2.event_type);
        assert_eq!(event1.message, event2.message);
    }
}
