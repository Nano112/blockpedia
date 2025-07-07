use blockpedia::{AllBlocks, ColorSpace, EasingFunction, ExtendedColorData, GradientConfig};

fn main() {
    println!("🧱 Blockpedia Query Builder Demo");
    println!("=================================\n");

    // Example 1: Basic filtering - solid blocks only
    println!("1. Finding solid building blocks:");
    let solid_blocks = AllBlocks::new()
        .only_solid()
        .exclude_tile_entities()
        .survival_only()
        .with_color()
        .limit(10)
        .collect();

    for block in &solid_blocks {
        if let Some(color) = block.extras.color {
            println!(
                "  {} (#{:02X}{:02X}{:02X})",
                block.id(),
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }
    println!(
        "  Found {} solid blocks with color data\n",
        solid_blocks.len()
    );

    // Example 2: Color-based filtering and gradient generation
    println!("2. Finding blocks similar to stone gray and generating gradient:");
    let stone_gray = ExtendedColorData::from_rgb(125, 125, 125);

    let gray_blocks = AllBlocks::new()
        .with_color()
        .similar_to_color(stone_gray, 30.0)
        .sort_by_color_similarity(stone_gray)
        .limit(20)
        .collect();

    println!(
        "  Found {} blocks similar to stone gray:",
        gray_blocks.len()
    );
    for block in &gray_blocks[..5.min(gray_blocks.len())] {
        if let Some(color) = block.extras.color {
            println!(
                "    {} (#{:02X}{:02X}{:02X})",
                block.id(),
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }

    // Now generate a gradient from these blocks
    let config = GradientConfig::new(8)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::EaseInOut);

    let gradient_blocks = AllBlocks::new()
        .with_color()
        .similar_to_color(stone_gray, 30.0)
        .generate_gradient(config)
        .collect();

    println!(
        "\n  Generated gradient with {} blocks:",
        gradient_blocks.len()
    );
    for block in &gradient_blocks {
        if let Some(color) = block.extras.color {
            println!(
                "    {} (#{:02X}{:02X}{:02X})",
                block.id(),
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }
    println!();

    // Example 3: Chaining gradient then filtering (shows bidirectional chaining)
    println!("3. Generate gradient first, then filter to only solid blocks:");
    let red = ExtendedColorData::from_rgb(200, 50, 50);
    let blue = ExtendedColorData::from_rgb(50, 50, 200);

    let gradient_config = GradientConfig::new(12)
        .with_color_space(ColorSpace::Hsl)
        .with_easing(EasingFunction::Sine);

    let filtered_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(red, blue, gradient_config)
        .only_solid()
        .exclude_transparent()
        .collect();

    println!("  Red to Blue gradient (solid blocks only):");
    for block in &filtered_gradient {
        if let Some(color) = block.extras.color {
            println!(
                "    {} (#{:02X}{:02X}{:02X})",
                block.id(),
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }
    println!(
        "  {} blocks in filtered gradient\n",
        filtered_gradient.len()
    );

    // Example 4: Family-based filtering with color sorting
    println!("4. Wool blocks sorted by hue:");
    let wool_blocks = AllBlocks::new()
        .from_families(&["wool"])
        .with_color()
        .sort_by_color_gradient()
        .collect();

    for block in &wool_blocks {
        if let Some(color) = block.extras.color {
            let extended = color.to_extended();
            println!(
                "    {} (H:{:.0}°, #{:02X}{:02X}{:02X})",
                block.id(),
                extended.hsl[0],
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }
    println!("  {} wool blocks found\n", wool_blocks.len());

    // Example 5: Property-based filtering
    println!("5. Redstone components that can be powered:");
    let redstone_blocks = AllBlocks::new()
        .with_property("powered")
        .survival_only()
        .sort_by_name()
        .limit(10)
        .collect();

    for block in &redstone_blocks {
        println!("    {}", block.id());
    }
    println!("  {} redstone blocks found\n", redstone_blocks.len());

    // Example 6: Complex multi-color gradient
    println!("6. Multi-color gradient through different block families:");
    let stone_blocks = AllBlocks::new()
        .from_families(&["stone"])
        .with_color()
        .limit(3)
        .collect();

    if stone_blocks.len() >= 3 {
        let multi_gradient_config = GradientConfig::new(15)
            .with_color_space(ColorSpace::Oklab)
            .with_easing(EasingFunction::CubicBezier {
                p1: (0.25, 0.1),
                p2: (0.75, 0.9),
            });

        let multi_gradient = AllBlocks::new()
            .from_families(&["stone"])
            .with_color()
            .limit(3)
            .generate_multi_gradient(multi_gradient_config)
            .only_solid()
            .collect();

        println!("  Multi-color stone gradient:");
        for block in &multi_gradient {
            if let Some(color) = block.extras.color {
                println!(
                    "    {} (#{:02X}{:02X}{:02X})",
                    block.id(),
                    color.rgb[0],
                    color.rgb[1],
                    color.rgb[2]
                );
            }
        }
        println!("  {} blocks in multi-gradient\n", multi_gradient.len());
    }

    // Example 7: Pattern matching with wildcards
    println!("7. All minecraft blocks containing 'stone' but not 'redstone':");
    let stone_variants = AllBlocks::new()
        .matching("*stone*")
        .exclude_families(&["redstone"])
        .with_color()
        .sort_by_name()
        .limit(8)
        .collect();

    for block in &stone_variants {
        if let Some(color) = block.extras.color {
            println!(
                "    {} (#{:02X}{:02X}{:02X})",
                block.id(),
                color.rgb[0],
                color.rgb[1],
                color.rgb[2]
            );
        }
    }
    println!("  {} stone variants found\n", stone_variants.len());

    println!("✨ Query Builder Demo Complete!");
    println!("\nKey Features Demonstrated:");
    println!("  • Chainable filtering (solid, survival, color, etc.)");
    println!("  • Gradient generation with custom config");
    println!("  • Bidirectional chaining (filter→gradient→filter)");
    println!("  • Multiple color spaces and easing functions");
    println!("  • Family-based and property-based filtering");
    println!("  • Pattern matching with wildcards");
    println!("  • Color similarity search and sorting");
}
