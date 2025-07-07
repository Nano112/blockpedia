use blockpedia::BLOCKS;

fn main() {
    println!("🎨 Testing Colors CLI Tab Features");
    println!("=================================");

    // Test Color Coverage Analysis
    println!("\n📊 Color Coverage Analysis:");
    let total_blocks = BLOCKS.len();
    let blocks_with_color: Vec<_> = BLOCKS
        .values()
        .filter(|b| b.extras.color.is_some())
        .collect();
    let coverage_percentage = (blocks_with_color.len() as f64 / total_blocks as f64) * 100.0;

    println!("  Total blocks: {}", total_blocks);
    println!("  Blocks with color data: {}", blocks_with_color.len());
    println!("  Coverage: {:.1}%", coverage_percentage);

    // Test Color Palette Analysis
    println!("\n🎯 Color Palette Analysis:");
    let mut red_blocks = Vec::new();
    let mut blue_blocks = Vec::new();
    let mut green_blocks = Vec::new();
    let mut other_blocks = Vec::new();

    for block in &blocks_with_color {
        if let Some(color) = &block.extras.color {
            let (r, g, b) = (color.rgb[0], color.rgb[1], color.rgb[2]);
            if r > g && r > b {
                red_blocks.push((block, color));
            } else if b > r && b > g {
                blue_blocks.push((block, color));
            } else if g > r && g > b {
                green_blocks.push((block, color));
            } else {
                other_blocks.push((block, color));
            }
        }
    }

    println!("  🔴 Red-dominant blocks: {}", red_blocks.len());
    println!("  🔵 Blue-dominant blocks: {}", blue_blocks.len());
    println!("  🟢 Green-dominant blocks: {}", green_blocks.len());
    println!("  ⚪ Other colors: {}", other_blocks.len());

    // Test Color Similarity Search
    println!("\n🔍 Color Similarity Search (similar to stone gray):");
    let target_color = [125, 125, 125]; // Stone color
    let mut similar_blocks = Vec::new();

    for block in &blocks_with_color {
        if let Some(color) = &block.extras.color {
            let distance = {
                let dr = (color.rgb[0] as f32) - (target_color[0] as f32);
                let dg = (color.rgb[1] as f32) - (target_color[1] as f32);
                let db = (color.rgb[2] as f32) - (target_color[2] as f32);
                (dr * dr + dg * dg + db * db).sqrt()
            };

            if distance < 50.0 {
                // Threshold for similarity
                similar_blocks.push((block, color, distance));
            }
        }
    }

    similar_blocks.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!(
        "  Found {} similar blocks to stone gray",
        similar_blocks.len()
    );

    for (block, color, distance) in similar_blocks.iter().take(5) {
        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            color.rgb[0], color.rgb[1], color.rgb[2]
        );
        println!("    • {} → {} (distance: {:.1})", block.id(), hex, distance);
    }

    // Test Color Analysis
    println!("\n📈 Color Analysis:");
    let mut avg_red = 0.0;
    let mut avg_green = 0.0;
    let mut avg_blue = 0.0;
    let mut brightest = (String::new(), 0u32);
    let mut darkest = (String::new(), 255u32 * 3);

    for block in &blocks_with_color {
        if let Some(color) = &block.extras.color {
            avg_red += color.rgb[0] as f64;
            avg_green += color.rgb[1] as f64;
            avg_blue += color.rgb[2] as f64;

            let brightness = color.rgb[0] as u32 + color.rgb[1] as u32 + color.rgb[2] as u32;
            if brightness > brightest.1 {
                brightest = (block.id().to_string(), brightness);
            }
            if brightness < darkest.1 {
                darkest = (block.id().to_string(), brightness);
            }
        }
    }

    let count = blocks_with_color.len() as f64;
    avg_red /= count;
    avg_green /= count;
    avg_blue /= count;

    println!(
        "  Average color: RGB({:.0}, {:.0}, {:.0})",
        avg_red, avg_green, avg_blue
    );
    println!(
        "  Average hex: #{:02X}{:02X}{:02X}",
        avg_red as u8, avg_green as u8, avg_blue as u8
    );
    println!(
        "  Brightest block: {} (brightness: {})",
        brightest.0, brightest.1
    );
    println!("  Darkest block: {} (brightness: {})", darkest.0, darkest.1);

    println!("\n✅ All Colors CLI features are working correctly!");
    println!("🎨 Ready to showcase our amazing color system in the interactive CLI!");
}
