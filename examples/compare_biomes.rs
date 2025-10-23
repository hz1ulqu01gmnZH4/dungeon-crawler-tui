/// Compare current vs MRF biome generation
use dungeon_clawler_tui::{Resources, world::{BiomeId, BiomeGenerator, MRFBiomeGenerator, TerrainGenerator}};
use noise::Perlin;

fn main() -> std::io::Result<()> {
    let seed = 12345u64;
    println!("{}", "=".repeat(70));
    println!("BIOME GENERATION COMPARISON: Current vs MRF");
    println!("{}", "=".repeat(70));
    println!();

    // Test 1: Current multi-pass generator
    println!(">>> CURRENT GENERATOR (Multi-pass neighbor influence)");
    let mut resources1 = Resources::new(80, 50, seed);
    
    let biome_gen_current = BiomeGenerator::new(seed);
    let biome_map_current = biome_gen_current.generate(50, 50);
    biome_gen_current.apply_to_overmap(&biome_map_current, &mut resources1.world.overmap);
    
    let terrain_gen1 = TerrainGenerator::new(seed);
    terrain_gen1.generate(&mut resources1.world.overmap);
    
    let stats_current = compute_biome_stats(&resources1.world.overmap);
    print_stats("CURRENT", &stats_current);
    
    println!();
    println!(">>> MRF GENERATOR (Graph-cut optimization)");
    
    // Test 2: MRF generator
    let mut resources2 = Resources::new(80, 50, seed);
    
    let elevation_noise = Perlin::new(seed as u32);
    let moisture_noise = Perlin::new((seed + 1) as u32);
    let biome_gen_mrf = MRFBiomeGenerator::new(50, 50, seed, elevation_noise, moisture_noise);
    let biome_map_mrf = biome_gen_mrf.generate();
    
    // Apply MRF biomes to overmap
    for y in 0..50 {
        for x in 0..50 {
            let idx = (y * 50 + x) as usize;
            if let Some(tile) = resources2.world.overmap.get_tile_mut(x, y) {
                tile.biome = biome_map_mrf.biomes[idx];
                tile.biome_strength = 1.0;
            }
        }
    }
    
    let terrain_gen2 = TerrainGenerator::new(seed);
    terrain_gen2.generate(&mut resources2.world.overmap);
    
    let stats_mrf = compute_biome_stats(&resources2.world.overmap);
    print_stats("MRF", &stats_mrf);
    
    println!();
    println!("{}", "=".repeat(70));
    println!("COMPARISON SUMMARY");
    println!("{}", "=".repeat(70));
    compare_stats(&stats_current, &stats_mrf);
    
    Ok(())
}

fn compute_biome_stats(overmap: &dungeon_clawler_tui::world::Overmap) -> std::collections::HashMap<BiomeId, usize> {
    let mut counts = std::collections::HashMap::new();
    
    for y in 0..overmap.height {
        for x in 0..overmap.width {
            if let Some(tile) = overmap.get_tile(x, y) {
                *counts.entry(tile.biome).or_insert(0) += 1;
            }
        }
    }
    
    counts
}

fn print_stats(_name: &str, stats: &std::collections::HashMap<BiomeId, usize>) {
    println!("  Biome distribution:");
    let total: usize = stats.values().sum();
    
    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by_key(|(_, count)| -(**count as i32));
    
    for (biome, count) in sorted {
        let percent = (*count as f32 / total as f32) * 100.0;
        println!("    {:?}: {} tiles ({:.1}%)", biome, count, percent);
    }
}

fn compare_stats(current: &std::collections::HashMap<BiomeId, usize>, mrf: &std::collections::HashMap<BiomeId, usize>) {
    println!("Biome          | Current | MRF     | Difference");
    println!("---------------|---------|---------|------------");
    
    let all_biomes = [
        BiomeId::Plains,
        BiomeId::Forest,
        BiomeId::DeepWilderness,
        BiomeId::Hills,
        BiomeId::Mountains,
        BiomeId::Wetlands,
    ];
    
    for biome in &all_biomes {
        let c = current.get(biome).unwrap_or(&0);
        let m = mrf.get(biome).unwrap_or(&0);
        let diff = *m as i32 - *c as i32;
        let sign = if diff > 0 { "+" } else { "" };
        println!("{:14} | {:7} | {:7} | {}{}", 
            format!("{:?}", biome), c, m, sign, diff);
    }
}
