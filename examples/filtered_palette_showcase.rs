use blockpedia::color::block_palettes::{BlockFilter, BlockPaletteGenerator};
use blockpedia::BLOCKS;

fn main() {
    println!("🧱 Filtered Block Palette Showcase");
    println!("===================================\n");

    // Example 1: Solid Blocks Only
    showcase_solid_blocks_filter();

    // Example 2: Structural Blocks Only
    showcase_structural_blocks_filter();

    // Example 3: Decorative Blocks
    showcase_decorative_blocks_filter();

    // Example 4: Custom Filter Examples
    showcase_custom_filters();

    // Example 5: Filter Comparison
    showcase_filter_comparison();
}

fn showcase_solid_blocks_filter() {
    println!("🏗️  SOLID BLOCKS ONLY FILTER");
    println!("============================\n");

    let solid_filter = BlockFilter::solid_blocks_only();

    println!("Filter Configuration:");
    println!("• Exclude falling blocks: {}", solid_filter.exclude_falling);
    println!(
        "• Exclude tile entities: {}",
        solid_filter.exclude_tile_entities
    );
    println!("• Full blocks only: {}", solid_filter.full_blocks_only);
    println!(
        "• Exclude needs support: {}",
        solid_filter.exclude_needs_support
    );
    println!(
        "• Exclude transparent: {}",
        solid_filter.exclude_transparent
    );
    println!(
        "• Survival obtainable only: {}",
        solid_filter.survival_obtainable_only
    );
    println!();

    if let Some(medieval_solid) =
        BlockPaletteGenerator::generate_architectural_palette_filtered("medieval", &solid_filter)
    {
        println!("🏰 Medieval Palette (Solid Blocks Only):");
        println!("   Description: {}", medieval_solid.description);
        println!("   Block Count: {}", medieval_solid.blocks.len());
        println!("   Recommended Blocks:");

        for (i, rec) in medieval_solid.blocks.iter().enumerate() {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");

            println!(
                "     {}. {} {} - {} ({})",
                i + 1,
                rec.color.hex_string(),
                block_name,
                format!("{:?}", rec.role),
                rec.usage_notes.split('.').next().unwrap_or("General use")
            );
        }
        println!();
    }

    println!("✨ Perfect for:");
    println!("   • Large-scale structural builds");
    println!("   • Foundations and main walls");
    println!("   • Builds that need to be stable and permanent");
    println!("   • Avoiding maintenance issues with falling/breaking blocks\n");
}

fn showcase_structural_blocks_filter() {
    println!("🏢 STRUCTURAL BLOCKS ONLY FILTER");
    println!("=================================\n");

    let structural_filter = BlockFilter::structural_blocks_only();

    println!("Filter Configuration (Most Conservative):");
    println!(
        "• Exclude falling blocks: {}",
        structural_filter.exclude_falling
    );
    println!(
        "• Exclude tile entities: {}",
        structural_filter.exclude_tile_entities
    );
    println!("• Full blocks only: {}", structural_filter.full_blocks_only);
    println!(
        "• Exclude needs support: {}",
        structural_filter.exclude_needs_support
    );
    println!(
        "• Exclude transparent: {}",
        structural_filter.exclude_transparent
    );
    println!(
        "• Exclude light sources: {}",
        structural_filter.exclude_light_sources
    );
    println!("• Exclude glass and water: Yes");
    println!();

    if let Some(modern_structural) =
        BlockPaletteGenerator::generate_architectural_palette_filtered("modern", &structural_filter)
    {
        println!("🏢 Modern Palette (Structural Blocks Only):");
        println!("   Description: {}", modern_structural.description);
        println!("   Block Count: {}", modern_structural.blocks.len());
        println!("   Recommended Blocks:");

        for (i, rec) in modern_structural.blocks.iter().enumerate() {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");

            println!(
                "     {}. {} {} - {}",
                i + 1,
                rec.color.hex_string(),
                block_name,
                format!("{:?}", rec.role)
            );
        }
        println!();
    }

    println!("🎯 Best for:");
    println!("   • Load-bearing structures");
    println!("   • Infrastructure projects");
    println!("   • Builds requiring maximum stability");
    println!("   • Large-scale construction where reliability is key\n");
}

fn showcase_decorative_blocks_filter() {
    println!("🎨 DECORATIVE BLOCKS FILTER");
    println!("===========================\n");

    let decorative_filter = BlockFilter::decorative_blocks();

    println!("Filter Configuration (Allows More Variety):");
    println!(
        "• Exclude falling blocks: {}",
        decorative_filter.exclude_falling
    );
    println!(
        "• Exclude tile entities: {}",
        decorative_filter.exclude_tile_entities
    );
    println!("• Full blocks only: {}", decorative_filter.full_blocks_only);
    println!(
        "• Exclude needs support: {}",
        decorative_filter.exclude_needs_support
    );
    println!(
        "• Exclude transparent: {}",
        decorative_filter.exclude_transparent
    );
    println!("• Allows stairs, slabs, fences: Yes");
    println!();

    if let Some(rustic_decorative) =
        BlockPaletteGenerator::generate_architectural_palette_filtered("rustic", &decorative_filter)
    {
        println!("🏡 Rustic Palette (Decorative Blocks):");
        println!("   Description: {}", rustic_decorative.description);
        println!("   Block Count: {}", rustic_decorative.blocks.len());
        println!("   Recommended Blocks:");

        for (i, rec) in rustic_decorative.blocks.iter().enumerate() {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");

            println!(
                "     {}. {} {} - {}",
                i + 1,
                rec.color.hex_string(),
                block_name,
                format!("{:?}", rec.role)
            );
        }
        println!();
    }

    println!("✨ Great for:");
    println!("   • Detailed architectural features");
    println!("   • Interior decoration");
    println!("   • Creative builds with varied textures");
    println!("   • Builds where aesthetics matter more than structural concerns\n");
}

fn showcase_custom_filters() {
    println!("⚙️  CUSTOM FILTER EXAMPLES");
    println!("===========================\n");

    // Example 1: No Water/Lava Filter
    let mut no_liquids_filter = BlockFilter::default();
    no_liquids_filter.exclude_patterns = vec!["water".to_string(), "lava".to_string()];

    println!("🚫 No Liquids Filter:");
    if let Some(ocean_no_liquids) =
        BlockPaletteGenerator::generate_natural_palette_filtered("ocean", &no_liquids_filter)
    {
        println!("   Ocean Palette without water/lava:");
        for rec in ocean_no_liquids.blocks.iter().take(3) {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");
            println!("   • {} {}", rec.color.hex_string(), block_name);
        }
    }
    println!();

    // Example 2: Only Concrete Blocks
    let mut concrete_only_filter = BlockFilter::default();
    concrete_only_filter.include_patterns = vec!["concrete".to_string()];

    println!("🧱 Concrete Only Filter:");
    if let Some(modern_concrete) = BlockPaletteGenerator::generate_architectural_palette_filtered(
        "modern",
        &concrete_only_filter,
    ) {
        println!("   Modern palette with only concrete blocks:");
        for rec in modern_concrete.blocks.iter() {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");
            println!("   • {} {}", rec.color.hex_string(), block_name);
        }
    }
    println!();

    // Example 3: No Light Sources for Dark Builds
    let mut no_light_filter = BlockFilter::default();
    no_light_filter.exclude_light_sources = true;

    println!("🌑 No Light Sources Filter:");
    if let Some(nether_dark) =
        BlockPaletteGenerator::generate_natural_palette_filtered("nether", &no_light_filter)
    {
        println!("   Nether palette without light-emitting blocks:");
        for rec in nether_dark.blocks.iter().take(4) {
            let block_name = rec
                .block
                .id()
                .strip_prefix("minecraft:")
                .unwrap_or(rec.block.id())
                .replace('_', " ");
            println!("   • {} {}", rec.color.hex_string(), block_name);
        }
    }
    println!();

    println!("💡 Custom Filter Use Cases:");
    println!("   • No Liquids: Dry builds, desert themes, avoiding water mechanics");
    println!("   • Concrete Only: Ultra-modern builds, industrial themes, clean aesthetics");
    println!("   • No Light Sources: Dark/spooky builds, realistic lighting control");
    println!("   • Wood Only: Natural builds, cabin themes, organic architecture\n");
}

fn showcase_filter_comparison() {
    println!("📊 FILTER COMPARISON");
    println!("====================\n");

    let filters = vec![
        ("Default (No Filter)", BlockFilter::default()),
        ("Solid Blocks Only", BlockFilter::solid_blocks_only()),
        ("Structural Only", BlockFilter::structural_blocks_only()),
        ("Decorative", BlockFilter::decorative_blocks()),
    ];

    for (filter_name, filter) in filters {
        if let Some(palette) =
            BlockPaletteGenerator::generate_architectural_palette_filtered("medieval", &filter)
        {
            println!("🏰 Medieval Palette - {}:", filter_name);
            println!("   Block Count: {}", palette.blocks.len());
            println!("   Sample Blocks:");

            for rec in palette.blocks.iter().take(3) {
                let block_name = rec
                    .block
                    .id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id())
                    .replace('_', " ");
                println!("     • {} {}", rec.color.hex_string(), block_name);
            }
            println!();
        }
    }

    println!("📈 Filter Impact Analysis:");
    println!("   • Default Filter: Maximum variety, includes all block types");
    println!("   • Solid Blocks: Reduces complexity, ensures stability");
    println!("   • Structural Only: Minimal set, maximum reliability");
    println!("   • Decorative: Balanced approach, good for detailed builds\n");

    // Show which blocks get filtered out
    let total_blocks = BLOCKS.len();
    let solid_filter = BlockFilter::solid_blocks_only();
    let structural_filter = BlockFilter::structural_blocks_only();

    let solid_count = BLOCKS
        .values()
        .filter(|b| solid_filter.allows_block(b))
        .count();
    let structural_count = BLOCKS
        .values()
        .filter(|b| structural_filter.allows_block(b))
        .count();

    println!("📊 Block Filtering Statistics:");
    println!("   • Total blocks in game: {}", total_blocks);
    println!(
        "   • Solid filter allows: {} blocks ({:.1}%)",
        solid_count,
        (solid_count as f64 / total_blocks as f64) * 100.0
    );
    println!(
        "   • Structural filter allows: {} blocks ({:.1}%)",
        structural_count,
        (structural_count as f64 / total_blocks as f64) * 100.0
    );
    println!();

    println!("🎯 Choosing the Right Filter:");
    println!("   • New builders: Use Solid Blocks filter for easier, stable builds");
    println!("   • Large projects: Use Structural filter for reliability");
    println!("   • Creative builds: Use Decorative filter for maximum variety");
    println!("   • Specific themes: Create custom filters for unique requirements");
}
