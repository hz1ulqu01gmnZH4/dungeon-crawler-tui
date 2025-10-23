/// Export overmap with biomes as text/visual representation
use dungeon_clawler_tui::{Resources, world::{BiomeId, TerrainType, BiomeGenerator, TerrainGenerator, HydrologyGenerator}};
use noise::Perlin;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let seed = 12345u64;
    println!("Generating overmap with seed {}...", seed);

    let mut resources = Resources::new(80, 50, seed);

    // Generate biomes
    println!("  - Generating biomes...");
    let biome_generator = BiomeGenerator::new(seed);
    let biome_map = biome_generator.generate(50, 50);
    biome_generator.apply_to_overmap(&biome_map, &mut resources.world.overmap);

    // Generate terrain (without old rivers)
    println!("  - Generating terrain...");
    let terrain_generator = TerrainGenerator::new(seed);
    terrain_generator.generate(&mut resources.world.overmap);

    // Generate rivers using Priority-Flood hydrology
    println!("  - Generating rivers (Priority-Flood)...");
    let elevation_noise = Perlin::new(seed as u32);
    let hydrology = HydrologyGenerator::new(50, 50, seed, elevation_noise);
    hydrology.generate_rivers(&mut resources.world.overmap);

    let overmap = &resources.world.overmap;

    println!("Overmap size: {}x{}", overmap.width, overmap.height);
    println!("Exporting maps...\n");

    // Export biome map
    export_biome_map(overmap, "overmap_biomes.txt")?;
    println!("✓ Saved biome map to overmap_biomes.txt");

    // Export terrain map
    export_terrain_map(overmap, "overmap_terrain.txt")?;
    println!("✓ Saved terrain map to overmap_terrain.txt");

    // Export combined map with legend
    export_combined_map(overmap, "overmap_combined.txt")?;
    println!("✓ Saved combined map to overmap_combined.txt");

    // Export statistics
    export_statistics(&resources, "overmap_stats.txt")?;
    println!("✓ Saved statistics to overmap_stats.txt");

    println!("\nExport complete!");
    Ok(())
}

fn export_biome_map(overmap: &dungeon_clawler_tui::world::Overmap, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "BIOME MAP ({}x{})", overmap.width, overmap.height)?;
    writeln!(file, "Seed: (use same seed as Resources::new)")?;
    writeln!(file)?;
    writeln!(file, "Legend:")?;
    writeln!(file, "  P = Plains")?;
    writeln!(file, "  F = Forest")?;
    writeln!(file, "  W = DeepWilderness")?;
    writeln!(file, "  H = Hills")?;
    writeln!(file, "  M = Mountains")?;
    writeln!(file, "  ~ = Wetlands")?;
    writeln!(file)?;

    for y in 0..overmap.height {
        for x in 0..overmap.width {
            if let Some(tile) = overmap.get_tile(x, y) {
                let symbol = biome_to_char(tile.biome);
                write!(file, "{}", symbol)?;
            } else {
                write!(file, "?")?;
            }
        }
        writeln!(file)?;
    }

    Ok(())
}

fn export_terrain_map(overmap: &dungeon_clawler_tui::world::Overmap, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "TERRAIN MAP ({}x{})", overmap.width, overmap.height)?;
    writeln!(file)?;
    writeln!(file, "Legend:")?;
    writeln!(file, "  . = Plains")?;
    writeln!(file, "  t = Forest")?;
    writeln!(file, "  T = DenseForest")?;
    writeln!(file, "  ^ = Mountains")?;
    writeln!(file, "  n = Hills")?;
    writeln!(file, "  ~ = Lake/River")?;
    writeln!(file, "  = = Road")?;
    writeln!(file, "  @ = Settlement")?;
    writeln!(file)?;

    for y in 0..overmap.height {
        for x in 0..overmap.width {
            if let Some(tile) = overmap.get_tile(x, y) {
                let symbol = terrain_to_char(tile.terrain);
                write!(file, "{}", symbol)?;
            } else {
                write!(file, "?")?;
            }
        }
        writeln!(file)?;
    }

    Ok(())
}

fn export_combined_map(overmap: &dungeon_clawler_tui::world::Overmap, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "COMBINED BIOME + TERRAIN MAP ({}x{})", overmap.width, overmap.height)?;
    writeln!(file)?;
    writeln!(file, "Format: [Biome:Terrain]")?;
    writeln!(file)?;

    // Create a more compact view - sample every 2x2 block
    for y in (0..overmap.height).step_by(2) {
        for x in (0..overmap.width).step_by(2) {
            if let Some(tile) = overmap.get_tile(x, y) {
                let biome = biome_to_char(tile.biome);
                let terrain = terrain_to_char(tile.terrain);
                write!(file, "{}{}", biome, terrain)?;
            } else {
                write!(file, "??")?;
            }
        }
        writeln!(file)?;
    }

    Ok(())
}

fn export_statistics(resources: &Resources, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let overmap = &resources.world.overmap;

    writeln!(file, "OVERMAP STATISTICS")?;
    writeln!(file, "===================")?;
    writeln!(file)?;
    writeln!(file, "Dimensions: {}x{}", overmap.width, overmap.height)?;
    writeln!(file, "Total tiles: {}", overmap.width * overmap.height)?;
    writeln!(file)?;

    // Count biomes
    let mut biome_counts = std::collections::HashMap::new();
    let mut terrain_counts = std::collections::HashMap::new();

    for y in 0..overmap.height {
        for x in 0..overmap.width {
            if let Some(tile) = overmap.get_tile(x, y) {
                *biome_counts.entry(tile.biome).or_insert(0) += 1;
                *terrain_counts.entry(tile.terrain).or_insert(0) += 1;
            }
        }
    }

    writeln!(file, "BIOME DISTRIBUTION:")?;
    let total = (overmap.width * overmap.height) as f32;
    let mut biome_vec: Vec<_> = biome_counts.iter().collect();
    biome_vec.sort_by_key(|(_, count)| -(**count as i32));

    for (biome, count) in biome_vec {
        let percent = (*count as f32 / total) * 100.0;
        writeln!(file, "  {:?}: {} tiles ({:.1}%)", biome, count, percent)?;
    }

    writeln!(file)?;
    writeln!(file, "TERRAIN DISTRIBUTION:")?;
    let mut terrain_vec: Vec<_> = terrain_counts.iter().collect();
    terrain_vec.sort_by_key(|(_, count)| -(**count as i32));

    for (terrain, count) in terrain_vec {
        let percent = (*count as f32 / total) * 100.0;
        writeln!(file, "  {:?}: {} tiles ({:.1}%)", terrain, count, percent)?;
    }

    writeln!(file)?;
    writeln!(file, "SETTLEMENTS:")?;
    for settlement in &resources.world.settlements {
        writeln!(file, "  {} ({:?}) at ({}, {})",
                settlement.name, settlement.settlement_type,
                settlement.position.0, settlement.position.1)?;
    }

    writeln!(file)?;
    writeln!(file, "ROADS: {} segments", resources.world.roads.len())?;

    Ok(())
}

fn biome_to_char(biome: BiomeId) -> char {
    match biome {
        BiomeId::Plains => 'P',
        BiomeId::Forest => 'F',
        BiomeId::DeepWilderness => 'W',
        BiomeId::Hills => 'H',
        BiomeId::Mountains => 'M',
        BiomeId::Wetlands => '~',
    }
}

fn terrain_to_char(terrain: TerrainType) -> char {
    match terrain {
        TerrainType::Plains => '.',
        TerrainType::Forest => 't',
        TerrainType::DenseForest => 'T',
        TerrainType::Mountains => '^',
        TerrainType::Hills => 'n',
        TerrainType::Lake => '~',
        TerrainType::River => '~',
        TerrainType::Swamp => '%',
        TerrainType::Road => '=',
        TerrainType::KingRoad => '#',
        TerrainType::TradePath => '-',
        TerrainType::Settlement => '@',
        TerrainType::Dungeon => 'D',
        TerrainType::SpecialLocation => 'X',
    }
}
