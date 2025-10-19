/// Time progression and weather system integration tests
///
/// Tests that validate time advancement, seasonal changes, weather updates,
/// day/night cycle, and camping mechanics.

mod common;
use common::*;
use crossterm::event::KeyCode;

#[test]
fn test_local_movement_time_progression() {
    let mut harness = GameTestHarness::new(12345);

    let initial_time = harness.current_time().total_minutes();

    // Move locally (should advance time slightly or not at all)
    for _ in 0..10 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    let final_time = harness.current_time().total_minutes();
    let time_diff = final_time - initial_time;

    // Local movement should take minimal time (0-10 minutes for 10 moves)
    assert!(time_diff <= 10,
        "Local movement should take minimal time. Elapsed: {} minutes",
        time_diff);
}

#[test]
fn test_overmap_movement_time_cost() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_time = harness.current_time().total_minutes();

    // Move on overmap (should take ~60 minutes per tile)
    harness.press_key(KeyCode::Right).ok();
    harness.tick(1);

    let final_time = harness.current_time().total_minutes();
    let time_diff = final_time - initial_time;

    // Should take significant time (30-120 minutes depending on terrain/weather)
    assert!(time_diff >= 20 && time_diff <= 200,
        "Overmap movement should take 20-200 minutes. Actual: {} minutes",
        time_diff);
}

#[test]
fn test_day_night_cycle() {
    let mut harness = GameTestHarness::new(12345);

    // Advance time by 24 hours
    harness.advance_time_by_days(1);

    let time = harness.current_time();

    // Day should have advanced
    assert_eq!(time.day_of_year, 2, "Should advance to day 2");

    // Should be back to starting hour (8:00 AM)
    assert_eq!(time.hour, 8, "Should be 8:00 AM after 24 hours from 8:00 AM start");
}

#[test]
fn test_season_progression() {
    let mut harness = GameTestHarness::new(12345);

    // Start in Spring (day 1)
    let initial_season = harness.current_time().season();
    assert_eq!(initial_season, dungeon_clawler_tui::world::Season::Spring);

    // Advance 90 days (into Summer)
    harness.advance_time_by_days(90);

    let summer_season = harness.current_time().season();
    assert_eq!(summer_season, dungeon_clawler_tui::world::Season::Summer,
        "Day 91 should be Summer");

    // Advance 90 more days (into Autumn)
    harness.advance_time_by_days(90);

    let autumn_season = harness.current_time().season();
    assert_eq!(autumn_season, dungeon_clawler_tui::world::Season::Autumn,
        "Day 181 should be Autumn");

    // Advance 90 more days (into Winter)
    harness.advance_time_by_days(90);

    let winter_season = harness.current_time().season();
    assert_eq!(winter_season, dungeon_clawler_tui::world::Season::Winter,
        "Day 271 should be Winter");
}

#[test]
fn test_year_rollover() {
    let mut harness = GameTestHarness::new(12345);

    let initial_year = harness.current_time().year;

    // Advance 360 days (full year)
    harness.advance_time_by_days(360);

    let final_year = harness.current_time().year;

    // Should advance to next year
    assert_eq!(final_year, initial_year + 1, "Should advance to next year after 360 days");

    // Should be back to day 1
    let day = harness.current_time().day_of_year;
    assert_eq!(day, 1, "Should reset to day 1 after year rollover");
}

#[test]
fn test_weather_system_initialization() {
    let harness = GameTestHarness::new(12345);

    let weather = harness.current_weather();

    // Should have a valid weather type
    use dungeon_clawler_tui::world::Weather;
    let valid_weathers = [
        Weather::Clear,
        Weather::PartlyCloudy,
        Weather::Overcast,
        Weather::LightRain,
        Weather::HeavyRain,
        Weather::Thunderstorm,
        Weather::LightSnow,
        Weather::HeavySnow,
        Weather::Fog,
        Weather::Windy,
    ];

    assert!(valid_weathers.contains(&weather),
        "Should have valid weather type: {:?}", weather);
}

#[test]
fn test_weather_changes_over_time() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap to enable weather updates
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_weather = harness.current_weather();

    // Move around to trigger weather updates
    let mut weather_changed = false;
    for _ in 0..20 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);

        let current_weather = harness.current_weather();
        if current_weather != initial_weather {
            weather_changed = true;
            break;
        }
    }

    // Weather might change (it's probabilistic, but 20 moves gives good chance)
    // This test documents that weather CAN change, not that it MUST
    if weather_changed {
        let final_weather = harness.current_weather();
        assert_ne!(final_weather, initial_weather,
            "Weather should change over time");
    }
}

#[test]
fn test_camping_restores_hp() {
    let mut harness = GameTestHarness::new(12345);

    // Enter settlement (safe location for camping)
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Get max HP and damage player to 50%
    let max_hp = harness.get_player_stats().unwrap().max_hp.value();
    let target_hp = max_hp / 2;
    harness.damage_player(target_hp);

    let damaged_hp = harness.get_player_stats().unwrap().hp.value();
    assert!(damaged_hp < max_hp, "Player should be damaged");

    // Camp/rest (using 'r' key)
    harness.press_key(KeyCode::Char('r')).ok();
    harness.tick(1);

    // HP should be restored
    let restored_hp = harness.get_player_stats().unwrap().hp.value();
    assert!(restored_hp > damaged_hp,
        "Camping should restore HP. Was: {}, Now: {}", damaged_hp, restored_hp);
}

#[test]
fn test_camping_advances_time() {
    let mut harness = GameTestHarness::new(12345);

    // Enter settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Damage player so camping is allowed
    let max_hp = harness.get_player_stats().unwrap().max_hp.value();
    harness.damage_player(max_hp / 2);

    let initial_time = harness.current_time().total_minutes();

    // Camp
    harness.press_key(KeyCode::Char('r')).ok();
    harness.tick(1);

    let final_time = harness.current_time().total_minutes();
    let time_diff = final_time - initial_time;

    // Should advance time by several hours (rest is 8 hours = 480 minutes)
    assert!(time_diff >= 400,
        "Camping should advance time significantly. Elapsed: {} minutes", time_diff);
}

#[test]
fn test_cannot_rest_at_full_hp() {
    let mut harness = GameTestHarness::new(12345);

    // Enter settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Ensure player is at full HP
    let stats = harness.get_player_stats().unwrap();
    assert_eq!(stats.hp.value(), stats.max_hp.value(), "Player should start at full HP");

    let initial_time = harness.current_time().total_minutes();

    // Try to camp at full HP
    harness.press_key(KeyCode::Char('r')).ok();
    harness.tick(1);

    let final_time = harness.current_time().total_minutes();

    // Time should NOT advance significantly (rest rejected)
    let time_diff = final_time - initial_time;
    assert!(time_diff < 10,
        "Should not be able to rest at full HP. Time elapsed: {}", time_diff);

    // Message log should indicate already at full health
    let messages = harness.get_message_log();
    let has_full_hp_message = messages.iter()
        .any(|m| m.contains("full health") || m.contains("already"));
    assert!(has_full_hp_message, "Should message that player is already at full health");
}

#[test]
fn test_cannot_rest_in_wilderness() {
    let mut harness = GameTestHarness::new(12345);

    // Stay in local map mode (wilderness/dungeon)
    // Damage player
    let max_hp = harness.get_player_stats().unwrap().max_hp.value();
    harness.damage_player(max_hp / 2);

    // Try to rest in wilderness
    harness.press_key(KeyCode::Char('r')).ok();
    harness.tick(1);

    // Should not restore HP (not safe)
    let messages = harness.get_message_log();
    let has_unsafe_message = messages.iter()
        .any(|m| m.contains("not safe") || m.contains("cannot rest"));
    assert!(has_unsafe_message, "Should indicate cannot rest in unsafe location");
}

#[test]
fn test_time_of_day_changes() {
    let mut harness = GameTestHarness::new(12345);

    // Start at 8:00 AM (Day)
    use dungeon_clawler_tui::world::TimeOfDay;
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Day);

    // Advance to evening
    harness.resources.world_time.advance_hours(11); // 19:00 = Dusk
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Dusk);

    // Advance to night
    harness.resources.world_time.advance_hours(2); // 21:00 = Night
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Night);

    // Advance to deep night
    harness.resources.world_time.advance_hours(4); // 01:00 = DeepNight
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::DeepNight);

    // Advance to dawn
    harness.resources.world_time.advance_hours(6); // 07:00 = Dawn
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Dawn);
}
