//! Domain-specific newtype wrappers for type safety
//!
//! These types prevent mixing up similar primitive types and make the API clearer.

use serde::{Deserialize, Serialize};

/// Dungeon depth (0 = surface, 1+ = dungeon levels)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Depth(pub i32);

impl Depth {
    /// Surface level (not in dungeon)
    pub const SURFACE: Depth = Depth(0);

    /// First dungeon level
    pub const FIRST_LEVEL: Depth = Depth(1);

    /// Descend one level deeper
    pub fn descend(self) -> Depth {
        Depth(self.0 + 1)
    }

    /// Ascend one level (returns None if already at surface)
    pub fn ascend(self) -> Option<Depth> {
        if self.0 > 0 {
            Some(Depth(self.0 - 1))
        } else {
            None
        }
    }

    /// Check if this is the surface
    pub fn is_surface(self) -> bool {
        self.0 == 0
    }

    /// Check if this is in a dungeon
    pub fn is_dungeon(self) -> bool {
        self.0 > 0
    }

    /// Get the raw depth value
    pub fn value(self) -> i32 {
        self.0
    }
}

impl Default for Depth {
    fn default() -> Self {
        Self::SURFACE
    }
}

impl std::fmt::Display for Depth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_surface() {
            write!(f, "Surface")
        } else {
            write!(f, "Depth {}", self.0)
        }
    }
}

/// Hit points (current HP)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HitPoints(pub i32);

impl HitPoints {
    /// Create new hit points
    pub fn new(hp: i32) -> Self {
        HitPoints(hp.max(0))
    }

    /// Subtract HP (saturating at 0)
    pub fn saturating_sub(self, amount: i32) -> Self {
        HitPoints((self.0 - amount).max(0))
    }

    /// Add HP
    pub fn saturating_add(self, amount: i32) -> Self {
        HitPoints(self.0 + amount)
    }

    /// Check if dead (HP <= 0)
    pub fn is_dead(self) -> bool {
        self.0 <= 0
    }

    /// Check if alive (HP > 0)
    pub fn is_alive(self) -> bool {
        self.0 > 0
    }

    /// Get raw value
    pub fn value(self) -> i32 {
        self.0
    }
}

impl Default for HitPoints {
    fn default() -> Self {
        HitPoints(0)
    }
}

impl std::fmt::Display for HitPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} HP", self.0)
    }
}

/// Maximum hit points
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxHitPoints(pub i32);

impl MaxHitPoints {
    /// Create new max HP
    pub fn new(max_hp: i32) -> Self {
        MaxHitPoints(max_hp.max(1))
    }

    /// Get raw value
    pub fn value(self) -> i32 {
        self.0
    }
}

impl Default for MaxHitPoints {
    fn default() -> Self {
        MaxHitPoints(1)
    }
}

impl std::fmt::Display for MaxHitPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Attack power
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Power(pub i32);

impl Power {
    /// Create new power value
    pub fn new(power: i32) -> Self {
        Power(power.max(0))
    }

    /// Get raw value
    pub fn value(self) -> i32 {
        self.0
    }
}

impl Default for Power {
    fn default() -> Self {
        Power(0)
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Power {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Power(self.0 + other.0)
    }
}

impl std::ops::Sub for Power {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Power(self.0 - other.0)
    }
}

impl std::ops::AddAssign for Power {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign for Power {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Defense value
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Defense(pub i32);

impl Defense {
    /// Create new defense value
    pub fn new(defense: i32) -> Self {
        Defense(defense.max(0))
    }

    /// Get raw value
    pub fn value(self) -> i32 {
        self.0
    }
}

impl Default for Defense {
    fn default() -> Self {
        Defense(0)
    }
}

impl std::fmt::Display for Defense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Defense {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Defense(self.0 + other.0)
    }
}

impl std::ops::Sub for Defense {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Defense(self.0 - other.0)
    }
}

impl std::ops::AddAssign for Defense {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign for Defense {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Money/currency
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(pub u32);

impl Money {
    /// No money
    pub const ZERO: Money = Money(0);

    /// Create new money amount
    pub fn new(amount: u32) -> Self {
        Money(amount)
    }

    /// Add money
    pub fn add(self, amount: u32) -> Self {
        Money(self.0.saturating_add(amount))
    }

    /// Subtract money (saturating at 0)
    pub fn sub(self, amount: u32) -> Self {
        Money(self.0.saturating_sub(amount))
    }

    /// Check if can afford
    pub fn can_afford(self, cost: Money) -> bool {
        self.0 >= cost.0
    }

    /// Get raw value
    pub fn value(self) -> u32 {
        self.0
    }
}

impl Default for Money {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} gold", self.0)
    }
}

impl std::ops::Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Money(self.0.saturating_add(other.0))
    }
}

impl std::ops::Sub for Money {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Money(self.0.saturating_sub(other.0))
    }
}

impl std::ops::AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
    }
}

impl std::ops::SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.saturating_sub(other.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_surface() {
        let depth = Depth::SURFACE;
        assert!(depth.is_surface());
        assert!(!depth.is_dungeon());
        assert_eq!(depth.value(), 0);
    }

    #[test]
    fn test_depth_descend() {
        let surface = Depth::SURFACE;
        let level1 = surface.descend();
        assert_eq!(level1, Depth::FIRST_LEVEL);
        assert!(level1.is_dungeon());
    }

    #[test]
    fn test_depth_ascend() {
        let level2 = Depth(2);
        let level1 = level2.ascend().unwrap();
        assert_eq!(level1, Depth::FIRST_LEVEL);

        let surface = level1.ascend().unwrap();
        assert_eq!(surface, Depth::SURFACE);

        assert!(surface.ascend().is_none());
    }

    #[test]
    fn test_hitpoints_saturating_sub() {
        let hp = HitPoints(10);
        let damaged = hp.saturating_sub(5);
        assert_eq!(damaged.value(), 5);

        let overkill = hp.saturating_sub(20);
        assert_eq!(overkill.value(), 0);
        assert!(overkill.is_dead());
    }

    #[test]
    fn test_power_add_sub() {
        let power = Power(5);
        let buffed = power + Power(3);
        assert_eq!(buffed.value(), 8);

        let debuffed = buffed - Power(2);
        assert_eq!(debuffed.value(), 6);
    }

    #[test]
    fn test_money_can_afford() {
        let wallet = Money(100);
        assert!(wallet.can_afford(Money(50)));
        assert!(wallet.can_afford(Money(100)));
        assert!(!wallet.can_afford(Money(101)));
    }

    #[test]
    fn test_money_saturating_ops() {
        let money = Money(10);
        let overflow = money.add(u32::MAX);
        assert_eq!(overflow.value(), u32::MAX);

        let underflow = money.sub(20);
        assert_eq!(underflow.value(), 0);
    }
}
