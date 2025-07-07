use blockpedia::BLOCKS;
use std::collections::HashMap;

fn main() {
    println!("🔍 Missing Color Data Analysis");
    println!("==============================");

    let total_blocks = BLOCKS.len();
    let mut blocks_with_color = 0;
    let mut blocks_without_color = Vec::new();

    // Analyze all blocks
    for (block_id, block_facts) in BLOCKS.entries() {
        if block_facts.extras.color.is_some() {
            blocks_with_color += 1;
        } else {
            blocks_without_color.push(block_id);
        }
    }

    let missing_count = blocks_without_color.len();
    let coverage_percentage = (blocks_with_color as f32 / total_blocks as f32) * 100.0;

    println!("📊 Overall Statistics:");
    println!("  Total blocks: {}", total_blocks);
    println!("  Blocks WITH color: {}", blocks_with_color);
    println!("  Blocks MISSING color: {}", missing_count);
    println!("  Coverage: {:.1}%", coverage_percentage);
    println!();

    // Categorize missing blocks by type
    let mut categories = HashMap::new();

    for block_id in &blocks_without_color {
        let category = categorize_block(block_id);
        categories
            .entry(category)
            .or_insert(Vec::new())
            .push(block_id);
    }

    println!("📋 Missing Blocks by Category:");
    println!("==============================");

    let mut sorted_categories: Vec<_> = categories.iter().collect();
    sorted_categories.sort_by_key(|(_, blocks)| -(blocks.len() as i32));

    for (category, blocks) in sorted_categories {
        println!("\n🏷️  {} ({} blocks):", category, blocks.len());

        // Show first 20 blocks per category
        for (i, block_id) in blocks.iter().enumerate() {
            match i.cmp(&20) {
                std::cmp::Ordering::Less => {
                    println!("   • {}", block_id);
                }
                std::cmp::Ordering::Equal => {
                    println!("   ... and {} more", blocks.len() - 20);
                    break;
                }
                std::cmp::Ordering::Greater => break,
            }
        }
    }

    println!("\n🎯 Analysis Summary:");
    println!("===================");

    for (category, blocks) in &categories {
        let percentage = (blocks.len() as f32 / missing_count as f32) * 100.0;
        println!(
            "  {:20} | {:3} blocks ({:4.1}%)",
            category,
            blocks.len(),
            percentage
        );
    }

    println!("\n💡 Improvement Suggestions:");
    println!("===========================");

    // Analyze improvement opportunities
    if let Some(air_blocks) = categories.get("Air/Invisible") {
        println!("  🌫️  Air/Invisible blocks ({} blocks): These intentionally have no visual representation", air_blocks.len());
    }

    if let Some(redstone_blocks) = categories.get("Redstone") {
        println!(
            "  ⚡ Redstone blocks ({} blocks): Need redstone-specific textures",
            redstone_blocks.len()
        );
    }

    if let Some(plant_blocks) = categories.get("Plants/Natural") {
        println!("  🌱 Plants/Natural blocks ({} blocks): Need plant texture pack or biome-specific colors", plant_blocks.len());
    }

    if let Some(technical_blocks) = categories.get("Technical/Special") {
        println!(
            "  🔧 Technical blocks ({} blocks): May need custom textures or computed colors",
            technical_blocks.len()
        );
    }

    if let Some(stairs_blocks) = categories.get("Stairs") {
        println!(
            "  🪜 Stairs ({} blocks): Could inherit colors from base block materials",
            stairs_blocks.len()
        );
    }

    if let Some(slab_blocks) = categories.get("Slabs") {
        println!(
            "  📦 Slabs ({} blocks): Could inherit colors from base block materials",
            slab_blocks.len()
        );
    }

    if let Some(wall_blocks) = categories.get("Walls") {
        println!(
            "  🧱 Walls ({} blocks): Could inherit colors from base block materials",
            wall_blocks.len()
        );
    }

    println!("\n🚀 Next Steps:");
    println!("==============");
    println!("  1. Add texture mappings for the largest missing categories");
    println!("  2. Implement color inheritance for stairs/slabs/walls from base materials");
    println!("  3. Add special handling for redstone and plant blocks");
    println!("  4. Consider procedural color generation for technical blocks");
}

fn categorize_block(block_id: &str) -> &'static str {
    let block_name = block_id.strip_prefix("minecraft:").unwrap_or(block_id);

    match block_name {
        // Air and invisible blocks
        "air" | "void_air" | "cave_air" | "light" | "barrier" | "structure_void" => "Air/Invisible",

        // Water and fluids
        name if name.contains("water") || name.contains("lava") => "Fluids",

        // Redstone components
        name if name.contains("redstone")
            || name.contains("repeater")
            || name.contains("comparator")
            || name.contains("lever")
            || name.contains("button")
            || name.contains("pressure_plate")
            || name.contains("tripwire")
            || name.contains("piston")
            || name.contains("dispenser")
            || name.contains("dropper")
            || name.contains("hopper") =>
        {
            "Redstone"
        }

        // Doors and gates
        name if name.ends_with("_door")
            || name.ends_with("_gate")
            || name.ends_with("_trapdoor") =>
        {
            "Doors/Gates"
        }

        // Stairs
        name if name.ends_with("_stairs") => "Stairs",

        // Slabs
        name if name.ends_with("_slab") => "Slabs",

        // Walls
        name if name.ends_with("_wall") => "Walls",

        // Fences
        name if name.contains("fence") => "Fences",

        // Signs
        name if name.contains("sign") => "Signs",

        // Banners
        name if name.contains("banner") => "Banners",

        // Plants and natural
        name if name.contains("flower")
            || name.contains("grass")
            || name.contains("fern")
            || name.contains("vine")
            || name.contains("leaves")
            || name.contains("sapling")
            || name.contains("kelp")
            || name.contains("seagrass")
            || name.contains("coral")
            || name.contains("bamboo")
            || name.contains("cactus")
            || name.contains("sugar_cane")
            || name.contains("wheat")
            || name.contains("carrot")
            || name.contains("potato")
            || name.contains("beetroot")
            || name.contains("melon_stem")
            || name.contains("pumpkin_stem")
            || name.contains("sweet_berry")
            || name.contains("cocoa")
            || name.contains("mushroom")
            || name.contains("fungus")
            || name.contains("roots")
            || name.contains("lily_pad") =>
        {
            "Plants/Natural"
        }

        // Rails and transportation
        name if name.contains("rail") || name.contains("minecart") => "Transportation",

        // Beds
        name if name.contains("bed") => "Beds",

        // Candles and lighting
        name if name.contains("candle")
            || name.contains("torch")
            || name.contains("lantern")
            || name.contains("soul_fire")
            || name.contains("fire") =>
        {
            "Lighting"
        }

        // Heads and decorative
        name if name.contains("head") || name.contains("skull") => "Heads/Skulls",

        // Spawn eggs and items that aren't blocks
        name if name.contains("spawn_egg") || name.contains("item") => "Items/Non-blocks",

        // Technical/special blocks
        name if name.contains("command")
            || name.contains("structure")
            || name.contains("jigsaw")
            || name.contains("debug")
            || name.contains("end_portal")
            || name.contains("nether_portal")
            || name.contains("moving_piston")
            || name.contains("piston_head") =>
        {
            "Technical/Special"
        }

        // Crops and farmland
        name if name.contains("crop") || name.contains("farmland") || name.contains("soil") => {
            "Farming"
        }

        // Fluids and bubble columns
        name if name.contains("bubble") || name.contains("flowing") => "Fluid Effects",

        // Default category
        _ => "Other",
    }
}
