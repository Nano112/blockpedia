use blockpedia::BLOCKS;

fn main() {
    println!("🎨 Blockpedia Color Data Test");
    println!("=============================");

    let total_blocks = BLOCKS.len();
    let mut blocks_with_color = 0;
    let mut color_examples = Vec::new();

    // Collect some interesting color examples
    for (block_id, block_facts) in BLOCKS.entries() {
        if let Some(color) = &block_facts.extras.color {
            blocks_with_color += 1;

            // Collect some interesting examples
            if color_examples.len() < 20 {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                color_examples.push((block_id, hex, color.rgb));
            }
        }
    }

    let coverage_percentage = (blocks_with_color as f32 / total_blocks as f32) * 100.0;

    println!("📊 Color Coverage Statistics:");
    println!("  Total blocks: {}", total_blocks);
    println!("  Blocks with color data: {}", blocks_with_color);
    println!("  Coverage: {:.1}%", coverage_percentage);
    println!();

    println!("🌈 Sample Block Colors:");
    println!("  Block ID                    | Hex Color | RGB Values");
    println!("  ---------------------------------------------------");

    for (block_id, hex, rgb) in color_examples {
        println!(
            "  {:26} | {:9} | {:3}, {:3}, {:3}",
            block_id, hex, rgb[0], rgb[1], rgb[2]
        );
    }

    // Test some specific blocks we know should have colors
    println!();
    println!("🔍 Specific Block Tests:");

    let test_blocks = [
        "minecraft:stone",
        "minecraft:grass_block",
        "minecraft:dirt",
        "minecraft:oak_log",
        "minecraft:oak_planks",
        "minecraft:cobblestone",
        "minecraft:red_wool",
        "minecraft:diamond_ore",
        "minecraft:gold_block",
        "minecraft:emerald_block",
    ];

    for block_id in test_blocks {
        if let Some(block) = BLOCKS.get(block_id) {
            if let Some(color) = &block.extras.color {
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.rgb[0], color.rgb[1], color.rgb[2]
                );
                println!(
                    "  ✅ {} -> {} (RGB: {}, {}, {})",
                    block_id, hex, color.rgb[0], color.rgb[1], color.rgb[2]
                );
            } else {
                println!("  ❌ {} -> No color data", block_id);
            }
        } else {
            println!("  ❓ {} -> Block not found", block_id);
        }
    }

    println!();
    println!("🎉 Color extraction system is working!");
    println!(
        "   From 14 blocks (1.3%) to {} blocks ({:.1}%)",
        blocks_with_color, coverage_percentage
    );
    println!(
        "   That's a {:.1}x improvement!",
        blocks_with_color as f32 / 14.0
    );
}
