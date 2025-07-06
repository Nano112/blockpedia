use blockpedia::color::block_palettes::BlockPaletteGenerator;
use blockpedia::color::ExtendedColorData;
use blockpedia::BLOCKS;

fn main() {
    println!("🧱 Blockpedia Block Palette Showcase");
    println!("=====================================\n");

    // Example 1: Natural Biome Palettes
    showcase_natural_palettes();
    
    // Example 2: Architectural Style Palettes
    showcase_architectural_palettes();
    
    // Example 3: Block Gradient Palettes
    showcase_block_gradients();
    
    // Example 4: Monochrome Palettes
    showcase_monochrome_palettes();
    
    // Example 5: Complementary Palettes
    showcase_complementary_palettes();
    
    // Example 6: Color Range Search
    showcase_color_range_search();
    
    // Example 7: Export Formats
    showcase_export_formats();
    
    // Example 8: Theme and Style Lists
    showcase_available_options();
}

fn showcase_natural_palettes() {
    println!("🌿 NATURAL BIOME PALETTES");
    println!("=========================\n");
    
    let natural_themes = vec!["forest", "desert", "ocean", "mountain", "nether", "end"];
    
    for theme in natural_themes {
        if let Some(palette) = BlockPaletteGenerator::generate_natural_palette(theme) {
            println!("🏞️  {} Palette", palette.name);
            println!("   Description: {}", palette.description);
            println!("   Theme: {:?}", palette.theme);
            println!("   Block Count: {}", palette.blocks.len());
            println!("   Recommended Blocks:");
            
            for (i, rec) in palette.blocks.iter().take(4).enumerate() {
                let block_name = rec.block.id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id())
                    .replace('_', " ");
                    
                println!("     {}. {} {} - {} ({})", 
                    i + 1,
                    rec.color.hex_string(),
                    block_name,
                    format!("{:?}", rec.role),
                    rec.usage_notes.split('.').next().unwrap_or("General use")
                );
            }
            println!();
        }
    }
    
    println!("💡 Use Case Examples:");
    println!("   • Forest palette: Perfect for woodland builds, treehouses, ranger stations");
    println!("   • Desert palette: Ideal for Middle Eastern architecture, pyramids, oases");
    println!("   • Ocean palette: Great for underwater cities, aquariums, coastal builds");
    println!("   • Mountain palette: Excellent for fortresses, mining towns, rocky terrain");
    println!("   • Nether palette: Perfect for hellish builds, demon towers, fire themes");
    println!("   • End palette: Ideal for alien structures, space stations, ethereal builds\n");
}

fn showcase_architectural_palettes() {
    println!("🏗️  ARCHITECTURAL STYLE PALETTES");
    println!("================================\n");
    
    let architectural_styles = vec!["medieval", "modern", "rustic", "industrial"];
    
    for style in architectural_styles {
        if let Some(palette) = BlockPaletteGenerator::generate_architectural_palette(style) {
            println!("🏛️  {} Style", palette.name);
            println!("   Description: {}", palette.description);
            println!("   Best For: {}", match style {
                "medieval" => "Castles, villages, fantasy towns, historical builds",
                "modern" => "Cities, skyscrapers, contemporary homes, office buildings",
                "rustic" => "Farmhouses, barns, countryside, cozy cabins",
                "industrial" => "Factories, steampunk, machinery, urban decay",
                _ => "Various architectural projects"
            });
            println!("   Color Scheme: {}", match style {
                "medieval" => "Warm browns, stone grays, natural wood tones",
                "modern" => "Clean whites, cool grays, sleek metallics",
                "rustic" => "Weathered woods, earthy browns, natural textures",
                "industrial" => "Dark grays, metallic silvers, utilitarian tones",
                _ => "Varied"
            });
            println!("   Block Recommendations:");
            
            for (i, rec) in palette.blocks.iter().enumerate() {
                let block_name = rec.block.id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(rec.block.id())
                    .replace('_', " ");
                    
                let role_emoji = match rec.role {
                    blockpedia::color::block_palettes::BlockRole::Primary => "🏗️",
                    blockpedia::color::block_palettes::BlockRole::Secondary => "🔧",
                    blockpedia::color::block_palettes::BlockRole::Accent => "✨",
                    _ => "📦"
                };
                
                println!("     {}. {} {} {} - {}", 
                    i + 1,
                    role_emoji,
                    rec.color.hex_string(),
                    block_name,
                    rec.usage_notes.split('.').next().unwrap_or("General use")
                );
            }
            println!();
        }
    }
    
    println!("🎯 Pro Tips:");
    println!("   • Medieval: Layer different stone types for realistic weathering");
    println!("   • Modern: Use glass and concrete in 2:1 ratios for clean aesthetics");
    println!("   • Rustic: Mix stripped and regular logs for authentic aging");
    println!("   • Industrial: Add redstone components for functional machinery\n");
}

fn showcase_block_gradients() {
    println!("🌈 BLOCK GRADIENT PALETTES");
    println!("==========================\n");
    
    // Find some interesting blocks for gradients
    let colored_blocks: Vec<_> = BLOCKS.values()
        .filter(|b| b.extras.color.is_some())
        .collect();
    
    if colored_blocks.len() >= 6 {
        let gradient_examples = vec![
            (
                colored_blocks.iter().find(|b| b.id().contains("stone")).unwrap_or(&colored_blocks[0]),
                colored_blocks.iter().find(|b| b.id().contains("grass")).unwrap_or(&colored_blocks[1]),
                "Natural Stone to Grass Transition",
                "Perfect for blending stone structures into natural landscapes"
            ),
            (
                colored_blocks.iter().find(|b| b.id().contains("sand")).unwrap_or(&colored_blocks[2]),
                colored_blocks.iter().find(|b| b.id().contains("water")).unwrap_or(&colored_blocks[3]),
                "Desert to Ocean Gradient",
                "Ideal for creating coastal transitions or oasis effects"
            ),
            (
                colored_blocks.iter().find(|b| b.id().contains("oak")).unwrap_or(&colored_blocks[4]),
                colored_blocks.iter().find(|b| b.id().contains("dark_oak")).unwrap_or(&colored_blocks[5]),
                "Light to Dark Wood Transition",
                "Great for creating depth and shadow effects in wooden builds"
            ),
        ];
        
        for (i, (start_block, end_block, name, description)) in gradient_examples.iter().enumerate() {
            if let Some(gradient) = BlockPaletteGenerator::generate_block_gradient(start_block, end_block, 7) {
                println!("🎨 Example {}: {}", i + 1, name);
                println!("   Start: {} {}", 
                    start_block.extras.color.unwrap().to_extended().hex_string(),
                    start_block.id().strip_prefix("minecraft:").unwrap_or(start_block.id()).replace('_', " ")
                );
                println!("   End: {} {}", 
                    end_block.extras.color.unwrap().to_extended().hex_string(),
                    end_block.id().strip_prefix("minecraft:").unwrap_or(end_block.id()).replace('_', " ")
                );
                println!("   Use Case: {}", description);
                println!("   Gradient Steps:");
                
                for (j, rec) in gradient.blocks.iter().enumerate() {
                    let block_name = rec.block.id()
                        .strip_prefix("minecraft:")
                        .unwrap_or(rec.block.id())
                        .replace('_', " ");
                    
                    let progress = j as f32 / (gradient.blocks.len() - 1) as f32;
                    println!("     Step {}: {} {} ({}% transition)", 
                        j + 1,
                        rec.color.hex_string(),
                        block_name,
                        (progress * 100.0) as u8
                    );
                }
                println!();
            }
        }
    }
    
    println!("🛠️  Gradient Applications:");
    println!("   • Landscape Blending: Smooth transitions between different terrain types");
    println!("   • Architectural Shading: Create depth and shadow effects on large structures");
    println!("   • Artistic Builds: Rainbow bridges, sunset skies, color-based art");
    println!("   • Organic Shapes: Natural-looking curves and flowing designs");
    println!("   • Time-of-Day Effects: Simulate lighting changes with color gradients\n");
}

fn showcase_monochrome_palettes() {
    println!("⚫ MONOCHROME PALETTES");
    println!("=====================\n");
    
    // Create monochrome examples with different base blocks
    let base_examples = vec![
        ("stone", "Classic Gray Monochrome", "Perfect for minimalist and modern builds"),
        ("oak_planks", "Warm Wood Monochrome", "Ideal for cozy, natural-feeling structures"),
        ("red_wool", "Bold Red Monochrome", "Great for dramatic, high-impact designs"),
        ("prismarine", "Cool Blue Monochrome", "Excellent for underwater or ice-themed builds"),
    ];
    
    for (i, (block_search, name, description)) in base_examples.iter().enumerate() {
        if let Some(base_block) = BLOCKS.values().find(|b| 
            b.id().contains(block_search) && b.extras.color.is_some()
        ) {
            if let Some(mono_palette) = BlockPaletteGenerator::generate_monochrome_palette(base_block, 6) {
                println!("🎭 Example {}: {}", i + 1, name);
                println!("   Base Block: {} {}", 
                    base_block.extras.color.unwrap().to_extended().hex_string(),
                    base_block.id().strip_prefix("minecraft:").unwrap_or(base_block.id()).replace('_', " ")
                );
                println!("   Description: {}", description);
                println!("   Tonal Variations:");
                
                for (j, rec) in mono_palette.blocks.iter().enumerate() {
                    let block_name = rec.block.id()
                        .strip_prefix("minecraft:")
                        .unwrap_or(rec.block.id())
                        .replace('_', " ");
                    
                    let tone_description = match j {
                        0 => "Darkest",
                        j if j == mono_palette.blocks.len() - 1 => "Lightest",
                        j if j == mono_palette.blocks.len() / 2 => "Base Tone",
                        _ => "Mid Tone"
                    };
                    
                    println!("     {}: {} {} - {} ({})", 
                        tone_description,
                        rec.color.hex_string(),
                        block_name,
                        format!("{:?}", rec.role),
                        rec.usage_notes.split('.').next().unwrap_or("General use")
                    );
                }
                println!();
            }
        }
    }
    
    println!("🎨 Monochrome Design Principles:");
    println!("   • Contrast: Use lightest tones for highlights, darkest for shadows");
    println!("   • Hierarchy: Primary role blocks for main structure, accents for details");
    println!("   • Texture: Vary block types within the same color family for interest");
    println!("   • Scale: Use darker tones for large surfaces, lighter for smaller elements");
    println!("   • Mood: Cool tones for calm/modern, warm tones for cozy/traditional\n");
}

fn showcase_complementary_palettes() {
    println!("🔄 COMPLEMENTARY PALETTES");
    println!("=========================\n");
    
    // Find blocks with strong colors for complementary examples
    let strong_color_blocks: Vec<_> = BLOCKS.values()
        .filter(|b| {
            if let Some(color) = b.extras.color {
                // Look for blocks with saturated colors (not too gray)
                let (r, g, b) = (color.rgb[0] as f32, color.rgb[1] as f32, color.rgb[2] as f32);
                let max_component = r.max(g).max(b);
                let min_component = r.min(g).min(b);
                let saturation = if max_component > 0.0 {
                    (max_component - min_component) / max_component
                } else {
                    0.0
                };
                saturation > 0.3 // Moderately saturated colors
            } else {
                false
            }
        })
        .collect();
    
    println!("🎯 Complementary Color Theory:");
    println!("   Complementary colors are opposite on the color wheel and create high contrast");
    println!("   when used together. They're perfect for:");
    println!("   • Creating focal points and emphasis");
    println!("   • Adding visual energy and excitement");
    println!("   • Making builds stand out from their surroundings");
    println!("   • Balancing warm and cool color temperatures\n");
    
    if !strong_color_blocks.is_empty() {
        for (i, base_block) in strong_color_blocks.iter().take(3).enumerate() {
            if let Some(comp_palette) = BlockPaletteGenerator::generate_complementary_palette(base_block) {
                let base_color = base_block.extras.color.unwrap();
                let color_family = if base_color.rgb[0] > base_color.rgb[1] && base_color.rgb[0] > base_color.rgb[2] {
                    "Red Family"
                } else if base_color.rgb[1] > base_color.rgb[0] && base_color.rgb[1] > base_color.rgb[2] {
                    "Green Family"
                } else if base_color.rgb[2] > base_color.rgb[0] && base_color.rgb[2] > base_color.rgb[1] {
                    "Blue Family"
                } else {
                    "Neutral"
                };
                
                println!("⚡ Example {}: {} Complementary Scheme", i + 1, color_family);
                println!("   Base Block: {} {}", 
                    base_color.to_extended().hex_string(),
                    base_block.id().strip_prefix("minecraft:").unwrap_or(base_block.id()).replace('_', " ")
                );
                println!("   Color Relationship: {}", comp_palette.description);
                println!("   High-Contrast Blocks:");
                
                for (j, rec) in comp_palette.blocks.iter().enumerate() {
                    let block_name = rec.block.id()
                        .strip_prefix("minecraft:")
                        .unwrap_or(rec.block.id())
                        .replace('_', " ");
                    
                    let relationship = match j {
                        0 => "Base Color",
                        1 => "Complement",
                        _ => "Supporting"
                    };
                    
                    println!("     {}: {} {} - {} ({})", 
                        relationship,
                        rec.color.hex_string(),
                        block_name,
                        format!("{:?}", rec.role),
                        rec.usage_notes.split('.').next().unwrap_or("High contrast use")
                    );
                }
                println!();
            }
        }
    }
    
    println!("💡 Complementary Design Tips:");
    println!("   • Use the 60-30-10 rule: 60% base color, 30% complement, 10% accent");
    println!("   • Place complementary colors next to each other for maximum impact");
    println!("   • Use one color for large areas, complement for smaller details");
    println!("   • Consider tinting one color lighter/darker to reduce intensity");
    println!("   • Great for team builds: assign each team a complementary color\n");
}

fn showcase_color_range_search() {
    println!("🔍 COLOR RANGE SEARCH");
    println!("====================\n");
    
    // Example searches with different target colors and tolerances
    let search_examples = vec![
        (
            ExtendedColorData::from_rgb(128, 128, 128),
            30.0,
            "Neutral Gray Search",
            "Find blocks similar to stone gray - perfect for modern builds"
        ),
        (
            ExtendedColorData::from_rgb(139, 69, 19),
            40.0,
            "Warm Brown Search", 
            "Find earth-tone blocks for natural, rustic builds"
        ),
        (
            ExtendedColorData::from_rgb(34, 139, 34),
            35.0,
            "Forest Green Search",
            "Find green blocks for nature-themed builds and landscaping"
        ),
        (
            ExtendedColorData::from_rgb(70, 130, 180),
            45.0,
            "Ocean Blue Search",
            "Find blue blocks for water-themed and sky builds"
        ),
    ];
    
    for (i, (target_color, tolerance, name, description)) in search_examples.iter().enumerate() {
        let similar_blocks = BlockPaletteGenerator::find_blocks_by_color_range(
            *target_color, *tolerance, 8
        );
        
        println!("🎯 Example {}: {}", i + 1, name);
        println!("   Target Color: {} RGB({}, {}, {})", 
            target_color.hex_string(),
            target_color.rgb[0], target_color.rgb[1], target_color.rgb[2]
        );
        println!("   Search Tolerance: ±{:.0} units", tolerance);
        println!("   Use Case: {}", description);
        println!("   Found {} similar blocks:", similar_blocks.len());
        
        for (j, block) in similar_blocks.iter().enumerate() {
            if let Some(color) = block.extras.color {
                let block_name = block.id()
                    .strip_prefix("minecraft:")
                    .unwrap_or(block.id())
                    .replace('_', " ");
                
                let distance = color.to_extended().distance_oklab(target_color);
                
                println!("     {}. {} {} (Δ {:.1})", 
                    j + 1,
                    color.to_extended().hex_string(),
                    block_name,
                    distance
                );
            }
        }
        println!();
    }
    
    println!("🔧 Search Parameters Guide:");
    println!("   • Tolerance 0-20: Very strict matching, only nearly identical colors");
    println!("   • Tolerance 20-40: Moderate matching, similar shades and tints");
    println!("   • Tolerance 40-60: Loose matching, same color family");
    println!("   • Tolerance 60+: Very loose, includes related colors");
    println!("   • Max blocks: Limit results to most relevant matches\n");
}

fn showcase_export_formats() {
    println!("📄 EXPORT FORMATS");
    println!("=================\n");
    
    // Generate a sample palette for export examples
    if let Some(sample_palette) = BlockPaletteGenerator::generate_natural_palette("forest") {
        println!("📝 Text List Format:");
        println!("   Perfect for: Discord sharing, forum posts, planning documents");
        println!("   Content Preview:");
        let text_export = sample_palette.to_text_list();
        for line in text_export.lines().take(8) {
            println!("   {}", line);
        }
        if text_export.lines().count() > 8 {
            println!("   ... (truncated for display)");
        }
        println!();
        
        println!("📊 JSON Format:");
        println!("   Perfect for: Modding, automation, data analysis, web apps");
        println!("   Structure Preview:");
        let json_export = sample_palette.to_json();
        
        // Parse and pretty-print a portion of the JSON
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_export) {
            if let Some(obj) = parsed.as_object() {
                println!("   {{");
                if let Some(name) = obj.get("name") {
                    println!("     \"name\": {},", name);
                }
                if let Some(desc) = obj.get("description") {
                    println!("     \"description\": \"{}\",", 
                        desc.as_str().unwrap_or("").chars().take(50).collect::<String>()
                    );
                }
                if let Some(theme) = obj.get("theme") {
                    println!("     \"theme\": {},", theme);
                }
                if let Some(blocks) = obj.get("blocks").and_then(|b| b.as_array()) {
                    println!("     \"blocks\": [");
                    if let Some(first_block) = blocks.first() {
                        println!("       {{{}", 
                            serde_json::to_string_pretty(first_block)
                                .unwrap_or_default()
                                .lines()
                                .skip(1)
                                .take(5)
                                .map(|line| format!("       {}", line.trim_start()))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                        println!("       }},");
                        if blocks.len() > 1 {
                            println!("       ... ({} more blocks)", blocks.len() - 1);
                        }
                    }
                    println!("     ]");
                }
                println!("   }}");
            }
        }
        println!();
    }
    
    println!("💾 Export Use Cases:");
    println!("   Text Format:");
    println!("   • Share on social media and forums");
    println!("   • Create build planning documents");
    println!("   • Generate shopping lists for creative mode");
    println!("   • Copy-paste into build tutorials");
    println!();
    println!("   JSON Format:");
    println!("   • Import into external tools and mods");
    println!("   • Process with scripts and automation");
    println!("   • Integrate with web applications");
    println!("   • Store in databases for large projects");
    println!("   • Generate custom resource packs\n");
}

fn showcase_available_options() {
    println!("📋 AVAILABLE THEMES AND STYLES");
    println!("==============================\n");
    
    println!("🌿 Natural Biome Themes:");
    let natural_themes = BlockPaletteGenerator::get_natural_themes();
    for (i, theme) in natural_themes.iter().enumerate() {
        let description = match *theme {
            "forest" => "Rich greens and browns for woodland builds",
            "desert" => "Warm sandy tones for arid landscapes", 
            "ocean" => "Cool blues and aquatic colors",
            "mountain" => "Rocky grays and mineral tones",
            "nether" => "Dark reds and hellish colors",
            "end" => "Pale yellows and ethereal purples",
            _ => "Natural color palette"
        };
        
        let emoji = match *theme {
            "forest" => "🌲",
            "desert" => "🏜️",
            "ocean" => "🌊", 
            "mountain" => "⛰️",
            "nether" => "🔥",
            "end" => "🌌",
            _ => "🌿"
        };
        
        println!("   {}. {} {} - {}", i + 1, emoji, theme, description);
    }
    println!();
    
    println!("🏗️  Architectural Styles:");
    let arch_styles = BlockPaletteGenerator::get_architectural_styles();
    for (i, style) in arch_styles.iter().enumerate() {
        let description = match *style {
            "medieval" => "Traditional materials for castles and villages",
            "modern" => "Clean lines and contemporary materials",
            "rustic" => "Natural materials for countryside builds",
            "industrial" => "Metallic and mechanical components",
            _ => "Architectural style palette"
        };
        
        let emoji = match *style {
            "medieval" => "🏰",
            "modern" => "🏢",
            "rustic" => "🏡",
            "industrial" => "🏭",
            _ => "🏗️"
        };
        
        println!("   {}. {} {} - {}", i + 1, emoji, style, description);
    }
    println!();
    
    println!("🎨 Custom Palette Types:");
    println!("   1. 🌈 Gradient - Smooth transitions between any two colored blocks");
    println!("   2. ⚫ Monochrome - Tonal variations of any base block (3-10 shades)");
    println!("   3. 🔄 Complementary - High-contrast color combinations");
    println!("   4. 🔍 Color Search - Find blocks within custom color ranges");
    println!();
    
    println!("⚙️  API Quick Reference:");
    println!("   // Natural palettes");
    println!("   BlockPaletteGenerator::generate_natural_palette(\"forest\")");
    println!("   ");
    println!("   // Architectural palettes");
    println!("   BlockPaletteGenerator::generate_architectural_palette(\"medieval\")");
    println!("   ");
    println!("   // Block gradients");
    println!("   BlockPaletteGenerator::generate_block_gradient(block1, block2, 7)");
    println!("   ");
    println!("   // Monochrome variations");
    println!("   BlockPaletteGenerator::generate_monochrome_palette(base_block, 5)");
    println!("   ");
    println!("   // Color similarity search");
    println!("   BlockPaletteGenerator::find_blocks_by_color_range(target_color, 30.0, 10)");
    println!();
}
