use serde::{Deserialize, Serialize};

/// Types of settlements
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementType {
    Village,  // Small, 20-50 pop
    Town,     // Medium, 100-500 pop
    City,     // Large, 1000-5000 pop
}

impl SettlementType {
    pub fn name(&self) -> &'static str {
        match self {
            SettlementType::Village => "Village",
            SettlementType::Town => "Town",
            SettlementType::City => "City",
        }
    }

    pub fn population_range(&self) -> (i32, i32) {
        match self {
            SettlementType::Village => (20, 50),
            SettlementType::Town => (100, 500),
            SettlementType::City => (1000, 5000),
        }
    }

    pub fn min_distance(&self) -> i32 {
        match self {
            SettlementType::Village => 3,  // At least 3 tiles from another settlement
            SettlementType::Town => 5,     // At least 5 tiles
            SettlementType::City => 8,     // At least 8 tiles
        }
    }
}

/// A settlement in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settlement {
    pub id: usize,
    pub name: String,
    pub settlement_type: SettlementType,
    pub position: (i32, i32),
    pub population: i32,
    pub founded_year: i32,
}

impl Settlement {
    pub fn new(
        id: usize,
        name: String,
        settlement_type: SettlementType,
        position: (i32, i32),
        population: i32,
        founded_year: i32,
    ) -> Self {
        Self {
            id,
            name,
            settlement_type,
            position,
            population,
            founded_year,
        }
    }
}

/// Simple name generator for settlements
pub struct NameGenerator {
    seed: u64,
    counter: usize,
}

impl NameGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed, counter: 0 }
    }

    pub fn generate_settlement_name(&mut self) -> String {
        // Simple deterministic name generation
        let prefixes = [
            "Kand", "Sael", "Dael", "Vorn", "Thel", "Eld", "Brae", "Mor", "Ash", "Wyn",
            "Raven", "Silver", "Iron", "Stone", "River", "Oak", "Pine", "Mist", "Shadow",
            "Dawn",
        ];

        let suffixes = [
            "vale", "haven", "ford", "wich", "ton", "bury", "field", "wood", "dale", "moor",
            "port", "bridge", "cairn", "spire", "hold", "rest", "watch", "keep", "crest",
            "mere",
        ];

        let prefix_idx = ((self.seed.wrapping_add(self.counter as u64)) % prefixes.len() as u64) as usize;
        let suffix_idx =
            ((self.seed.wrapping_add(self.counter as u64 * 17)) % suffixes.len() as u64) as usize;

        self.counter += 1;

        format!("{}{}", prefixes[prefix_idx], suffixes[suffix_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_type_properties() {
        let village = SettlementType::Village;
        assert_eq!(village.name(), "Village");
        assert_eq!(village.population_range(), (20, 50));
        assert_eq!(village.min_distance(), 3);

        let city = SettlementType::City;
        assert_eq!(city.name(), "City");
        assert_eq!(city.population_range(), (1000, 5000));
        assert_eq!(city.min_distance(), 8);
    }

    #[test]
    fn test_settlement_creation() {
        let settlement = Settlement::new(
            0,
            "Testville".to_string(),
            SettlementType::Village,
            (10, 10),
            35,
            762,
        );

        assert_eq!(settlement.id, 0);
        assert_eq!(settlement.name, "Testville");
        assert_eq!(settlement.settlement_type, SettlementType::Village);
        assert_eq!(settlement.position, (10, 10));
        assert_eq!(settlement.population, 35);
    }

    #[test]
    fn test_name_generator() {
        let mut gen = NameGenerator::new(12345);

        let name1 = gen.generate_settlement_name();
        let name2 = gen.generate_settlement_name();

        assert!(!name1.is_empty());
        assert!(!name2.is_empty());
        // Names should be different
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_name_generator_deterministic() {
        let mut gen1 = NameGenerator::new(99999);
        let mut gen2 = NameGenerator::new(99999);

        let name1 = gen1.generate_settlement_name();
        let name2 = gen2.generate_settlement_name();

        // Same seed should produce same name
        assert_eq!(name1, name2);
    }
}
