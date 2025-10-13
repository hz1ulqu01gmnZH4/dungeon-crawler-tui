use serde::{Deserialize, Serialize};

/// Seasons in the game world
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring, // Days 1-90
    Summer, // Days 91-180
    Autumn, // Days 181-270
    Winter, // Days 271-360
}

impl Season {
    /// Get season from day of year (1-360)
    pub fn from_day(day: i32) -> Self {
        match day {
            1..=90 => Season::Spring,
            91..=180 => Season::Summer,
            181..=270 => Season::Autumn,
            _ => Season::Winter,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
}

/// Time of day categories
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOfDay {
    DeepNight,  // 0-5
    Dawn,       // 6-7
    Day,        // 8-17
    Dusk,       // 18-19
    Night,      // 20-23
}

impl TimeOfDay {
    pub fn from_hour(hour: i32) -> Self {
        match hour {
            0..=5 => TimeOfDay::DeepNight,
            6..=7 => TimeOfDay::Dawn,
            8..=17 => TimeOfDay::Day,
            18..=19 => TimeOfDay::Dusk,
            _ => TimeOfDay::Night,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TimeOfDay::DeepNight => "Deep Night",
            TimeOfDay::Dawn => "Dawn",
            TimeOfDay::Day => "Day",
            TimeOfDay::Dusk => "Dusk",
            TimeOfDay::Night => "Night",
        }
    }

    /// Visibility modifier for FOV range
    pub fn visibility_modifier(&self) -> i32 {
        match self {
            TimeOfDay::DeepNight => -3,
            TimeOfDay::Night => -2,
            TimeOfDay::Dawn => -1,
            TimeOfDay::Day => 0,
            TimeOfDay::Dusk => -1,
        }
    }
}

/// World time tracking year, season, day, hour, minute
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldTime {
    pub year: i32,
    pub day_of_year: i32, // 1-360
    pub hour: i32,        // 0-23
    pub minute: i32,      // 0-59
}

impl WorldTime {
    /// Create new world time starting at Year 762, Spring Day 1, 8:00 AM
    pub fn new() -> Self {
        Self {
            year: 762,
            day_of_year: 1,
            hour: 8,
            minute: 0,
        }
    }

    /// Create world time with specific values
    pub fn with_values(year: i32, day: i32, hour: i32, minute: i32) -> Self {
        Self {
            year,
            day_of_year: day.clamp(1, 360),
            hour: hour.clamp(0, 23),
            minute: minute.clamp(0, 59),
        }
    }

    /// Advance time by minutes
    pub fn advance_minutes(&mut self, minutes: i32) {
        self.minute += minutes;

        // Handle minute overflow
        while self.minute >= 60 {
            self.minute -= 60;
            self.hour += 1;
        }

        // Handle hour overflow
        while self.hour >= 24 {
            self.hour -= 24;
            self.day_of_year += 1;
        }

        // Handle day overflow
        while self.day_of_year > 360 {
            self.day_of_year -= 360;
            self.year += 1;
        }
    }

    /// Advance time by hours
    pub fn advance_hours(&mut self, hours: i32) {
        self.advance_minutes(hours * 60);
    }

    /// Advance time by days
    pub fn advance_days(&mut self, days: i32) {
        self.advance_hours(days * 24);
    }

    /// Get current season
    pub fn season(&self) -> Season {
        Season::from_day(self.day_of_year)
    }

    /// Get current time of day
    pub fn time_of_day(&self) -> TimeOfDay {
        TimeOfDay::from_hour(self.hour)
    }

    /// Format time as string (HH:MM)
    pub fn time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Format date as string
    pub fn date_string(&self) -> String {
        let season = self.season();
        let day_in_season = ((self.day_of_year - 1) % 90) + 1;
        format!(
            "{} {}, Year {}",
            season.name(),
            day_in_season,
            self.year
        )
    }

    /// Get total minutes since epoch (Year 0, Day 1, 00:00)
    pub fn total_minutes(&self) -> i64 {
        let years = self.year as i64;
        let days = self.day_of_year as i64 - 1; // 0-indexed for calculation
        let hours = self.hour as i64;
        let minutes = self.minute as i64;

        (years * 360 * 24 * 60) + (days * 24 * 60) + (hours * 60) + minutes
    }

    /// Check if it's currently night (for gameplay effects)
    pub fn is_night(&self) -> bool {
        matches!(
            self.time_of_day(),
            TimeOfDay::DeepNight | TimeOfDay::Night
        )
    }

    /// Check if it's currently day
    pub fn is_day(&self) -> bool {
        matches!(self.time_of_day(), TimeOfDay::Day)
    }
}

impl Default for WorldTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Time costs for various actions (in minutes)
pub struct TimeCosts;

impl TimeCosts {
    pub const MOVE_LOCAL: i32 = 0; // 6 seconds, but we round to 0 minutes
    pub const MOVE_OVERMAP: i32 = 60; // 1 hour per tile
    pub const MELEE_ATTACK: i32 = 0; // 6 seconds
    pub const CAST_SPELL: i32 = 1; // 1 minute
    pub const REST: i32 = 480; // 8 hours
    pub const EAT: i32 = 5; // 5 minutes
    pub const READ: i32 = 60; // 1 hour
    pub const CRAFT_SIMPLE: i32 = 10; // 10 minutes
    pub const CRAFT_COMPLEX: i32 = 480; // 8 hours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_creation() {
        let time = WorldTime::new();
        assert_eq!(time.year, 762);
        assert_eq!(time.day_of_year, 1);
        assert_eq!(time.hour, 8);
        assert_eq!(time.minute, 0);
    }

    #[test]
    fn test_advance_minutes() {
        let mut time = WorldTime::new();

        // Advance 30 minutes
        time.advance_minutes(30);
        assert_eq!(time.minute, 30);
        assert_eq!(time.hour, 8);

        // Advance 40 minutes (overflow to next hour)
        time.advance_minutes(40);
        assert_eq!(time.minute, 10);
        assert_eq!(time.hour, 9);
    }

    #[test]
    fn test_advance_hours() {
        let mut time = WorldTime::new();

        // Advance 5 hours
        time.advance_hours(5);
        assert_eq!(time.hour, 13);
        assert_eq!(time.day_of_year, 1);

        // Advance 20 hours (overflow to next day)
        time.advance_hours(20);
        assert_eq!(time.hour, 9);
        assert_eq!(time.day_of_year, 2);
    }

    #[test]
    fn test_advance_days() {
        let mut time = WorldTime::new();

        // Advance 10 days
        time.advance_days(10);
        assert_eq!(time.day_of_year, 11);
        assert_eq!(time.year, 762);

        // Advance 360 days (overflow to next year)
        time.advance_days(360);
        assert_eq!(time.day_of_year, 11);
        assert_eq!(time.year, 763);
    }

    #[test]
    fn test_seasons() {
        assert_eq!(Season::from_day(1), Season::Spring);
        assert_eq!(Season::from_day(90), Season::Spring);
        assert_eq!(Season::from_day(91), Season::Summer);
        assert_eq!(Season::from_day(180), Season::Summer);
        assert_eq!(Season::from_day(181), Season::Autumn);
        assert_eq!(Season::from_day(270), Season::Autumn);
        assert_eq!(Season::from_day(271), Season::Winter);
        assert_eq!(Season::from_day(360), Season::Winter);
    }

    #[test]
    fn test_time_of_day() {
        assert_eq!(TimeOfDay::from_hour(0), TimeOfDay::DeepNight);
        assert_eq!(TimeOfDay::from_hour(5), TimeOfDay::DeepNight);
        assert_eq!(TimeOfDay::from_hour(6), TimeOfDay::Dawn);
        assert_eq!(TimeOfDay::from_hour(7), TimeOfDay::Dawn);
        assert_eq!(TimeOfDay::from_hour(8), TimeOfDay::Day);
        assert_eq!(TimeOfDay::from_hour(17), TimeOfDay::Day);
        assert_eq!(TimeOfDay::from_hour(18), TimeOfDay::Dusk);
        assert_eq!(TimeOfDay::from_hour(19), TimeOfDay::Dusk);
        assert_eq!(TimeOfDay::from_hour(20), TimeOfDay::Night);
        assert_eq!(TimeOfDay::from_hour(23), TimeOfDay::Night);
    }

    #[test]
    fn test_time_formatting() {
        let time = WorldTime::with_values(762, 45, 14, 30);

        assert_eq!(time.time_string(), "14:30");
        assert_eq!(time.date_string(), "Spring 45, Year 762");
    }

    #[test]
    fn test_night_detection() {
        let mut time = WorldTime::new();

        time.hour = 12;
        assert!(!time.is_night());
        assert!(time.is_day());

        time.hour = 22;
        assert!(time.is_night());
        assert!(!time.is_day());
    }

    #[test]
    fn test_total_minutes() {
        let time = WorldTime::with_values(762, 1, 0, 0);
        let total = time.total_minutes();

        // Year 762 * 360 days * 24 hours * 60 minutes
        assert_eq!(total, 762 * 360 * 24 * 60);
    }

    #[test]
    fn test_year_rollover() {
        let mut time = WorldTime::with_values(762, 360, 23, 59);

        // Advance 1 minute to next year
        time.advance_minutes(1);

        assert_eq!(time.year, 763);
        assert_eq!(time.day_of_year, 1);
        assert_eq!(time.hour, 0);
        assert_eq!(time.minute, 0);
    }
}
