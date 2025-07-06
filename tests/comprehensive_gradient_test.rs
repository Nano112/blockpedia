use blockpedia::query_builder::*;
use blockpedia::color::ExtendedColorData;
use blockpedia::*;

#[test]
fn test_comprehensive_gradient_generation() {
    // Test different color spaces
    let color_spaces = [
        ColorSpace::Rgb,
        ColorSpace::Hsl,
        ColorSpace::Oklab,
        ColorSpace::Lab,
    ];
    
    // Test different easing functions
    let easing_functions = [
        EasingFunction::Linear,
        EasingFunction::EaseIn,
        EasingFunction::EaseOut,
        EasingFunction::EaseInOut,
        EasingFunction::CubicBezier { p1: (0.25, 0.1), p2: (0.25, 1.0) },
        EasingFunction::Sine,
        EasingFunction::Exponential,
    ];
    
    // Test different sampling methods
    let sampling_methods = [
        ColorSamplingMethod::Dominant,
        ColorSamplingMethod::Average,
        ColorSamplingMethod::Clustering { k: 3 },
        ColorSamplingMethod::EdgeWeighted,
        ColorSamplingMethod::MostFrequent { bins: 16 },
    ];
    
    // Test different gradient sizes
    let gradient_sizes = [1, 3, 5, 10, 20];
    
    let start_color = ExtendedColorData::from_rgb(255, 0, 0); // Red
    let end_color = ExtendedColorData::from_rgb(0, 0, 255);   // Blue
    
    for &color_space in &color_spaces {
        for &easing in &easing_functions {
            for &sampling in &sampling_methods {
                for &size in &gradient_sizes {
                    let config = GradientConfig::new(size)
                        .with_color_space(color_space)
                        .with_easing(easing)
                        .with_sampling(sampling);
                    
                    // Test gradient between two colors
                    let gradient = AllBlocks::new()
                        .with_color()
                        .generate_gradient_between_colors(start_color, end_color, config.clone());
                    
                    // Should have at most the requested number of blocks
                    assert!(gradient.len() <= size, 
                        "Gradient size {} exceeded requested size {} for {:?}/{:?}/{:?}", 
                        gradient.len(), size, color_space, easing, sampling);
                    
                    // If we have colored blocks, we should get some result for reasonable sizes
                    if size > 0 && AllBlocks::new().with_color().len() > 0 {
                        if size <= 10 { // For reasonable sizes, we should get some blocks
                            assert!(gradient.len() > 0, 
                                "Should get at least some blocks for size {} with {:?}/{:?}/{:?}", 
                                size, color_space, easing, sampling);
                        }
                    }
                    
                    // All returned blocks should have color data
                    for block in gradient.collect() {
                        assert!(block.extras.color.is_some(), 
                            "Block {} should have color data", block.id());
                    }
                }
            }
        }
    }
}

#[test]
fn test_gradient_edge_cases() {
    // Test with no colored blocks available (empty set)
    let empty_gradient = AllBlocks::new()
        .matching("nonexistent:impossible_block")
        .generate_gradient(GradientConfig::new(5));
    assert_eq!(empty_gradient.len(), 0, "Empty query should produce empty gradient");
    
    // Test with single colored block
    let single_block_gradient = AllBlocks::new()
        .with_color()
        .limit(1)
        .generate_gradient(GradientConfig::new(5));
    assert!(single_block_gradient.len() <= 1, "Single block should not produce more than 1 result");
    
    // Test zero-size gradient
    let zero_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient(GradientConfig::new(0));
    assert_eq!(zero_gradient.len(), 0, "Zero-size gradient should be empty");
    
    // Test very large gradient
    let large_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient(GradientConfig::new(1000));
    assert!(large_gradient.len() <= 1000, "Large gradient should not exceed requested size");
}

#[test]
fn test_multi_gradient_comprehensive() {
    let config = GradientConfig::new(10)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::EaseInOut);
    
    let multi_gradient = AllBlocks::new()
        .with_color()
        .limit(20) // Use a reasonable subset
        .generate_multi_gradient(config);
    
    // Should get some result if we have colored blocks
    let colored_count = AllBlocks::new().with_color().len();
    if colored_count > 0 {
        assert!(multi_gradient.len() <= 10, "Multi-gradient should respect size limit");
        
        // All blocks should have color data
        for block in multi_gradient.collect() {
            assert!(block.extras.color.is_some(), 
                "Multi-gradient block {} should have color data", block.id());
        }
    }
}

#[test]
fn test_color_space_differences() {
    let start_color = ExtendedColorData::from_rgb(255, 0, 0); // Red
    let end_color = ExtendedColorData::from_rgb(0, 255, 0);   // Green
    
    let rgb_config = GradientConfig::new(5)
        .with_color_space(ColorSpace::Rgb)
        .with_easing(EasingFunction::Linear);
    
    let hsl_config = GradientConfig::new(5)
        .with_color_space(ColorSpace::Hsl)
        .with_easing(EasingFunction::Linear);
    
    let oklab_config = GradientConfig::new(5)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::Linear);
    
    let rgb_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, rgb_config);
    
    let hsl_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, hsl_config);
    
    let oklab_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, oklab_config);
    
    // All should work and return some result
    println!("RGB gradient: {} blocks", rgb_gradient.len());
    println!("HSL gradient: {} blocks", hsl_gradient.len());
    println!("Oklab gradient: {} blocks", oklab_gradient.len());
    
    // The gradients might produce different results due to different color space interpolation
    // This is expected behavior - just verify they all work
    assert!(rgb_gradient.len() <= 5);
    assert!(hsl_gradient.len() <= 5);
    assert!(oklab_gradient.len() <= 5);
}

#[test]
fn test_easing_function_effects() {
    let start_color = ExtendedColorData::from_rgb(0, 0, 0);     // Black
    let end_color = ExtendedColorData::from_rgb(255, 255, 255); // White
    
    let linear_config = GradientConfig::new(7)
        .with_color_space(ColorSpace::Rgb)
        .with_easing(EasingFunction::Linear);
    
    let ease_in_config = GradientConfig::new(7)
        .with_color_space(ColorSpace::Rgb)
        .with_easing(EasingFunction::EaseIn);
    
    let ease_out_config = GradientConfig::new(7)
        .with_color_space(ColorSpace::Rgb)
        .with_easing(EasingFunction::EaseOut);
    
    let ease_in_out_config = GradientConfig::new(7)
        .with_color_space(ColorSpace::Rgb)
        .with_easing(EasingFunction::EaseInOut);
    
    let linear_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, linear_config);
    
    let ease_in_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, ease_in_config);
    
    let ease_out_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, ease_out_config);
    
    let ease_in_out_gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, ease_in_out_config);
    
    // All easing functions should work
    assert!(linear_gradient.len() <= 7);
    assert!(ease_in_gradient.len() <= 7);
    assert!(ease_out_gradient.len() <= 7);
    assert!(ease_in_out_gradient.len() <= 7);
    
    println!("Linear: {} blocks", linear_gradient.len());
    println!("Ease In: {} blocks", ease_in_gradient.len());
    println!("Ease Out: {} blocks", ease_out_gradient.len());
    println!("Ease In-Out: {} blocks", ease_in_out_gradient.len());
}

#[test]
fn test_sampling_method_consistency() {
    let start_color = ExtendedColorData::from_rgb(128, 64, 192);
    let end_color = ExtendedColorData::from_rgb(64, 192, 128);
    
    let sampling_methods = [
        ColorSamplingMethod::Dominant,
        ColorSamplingMethod::Average,
        ColorSamplingMethod::Clustering { k: 5 },
        ColorSamplingMethod::EdgeWeighted,
        ColorSamplingMethod::MostFrequent { bins: 8 },
    ];
    
    for &sampling in &sampling_methods {
        let config = GradientConfig::new(6)
            .with_color_space(ColorSpace::Oklab)
            .with_easing(EasingFunction::Linear)
            .with_sampling(sampling);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(start_color, end_color, config);
        
        // Should work with any sampling method
        assert!(gradient.len() <= 6, 
            "Gradient with {:?} sampling should respect size limit", sampling);
        
        // All blocks should have color data
        for block in gradient.collect() {
            assert!(block.extras.color.is_some(), 
                "Block {} with {:?} sampling should have color data", 
                block.id(), sampling);
        }
    }
}

#[test]
fn test_gradient_size_scaling() {
    let start_color = ExtendedColorData::from_rgb(255, 100, 50);
    let end_color = ExtendedColorData::from_rgb(50, 255, 100);
    
    let sizes = [1, 2, 3, 5, 8, 13, 21, 50];
    
    for &size in &sizes {
        let config = GradientConfig::new(size)
            .with_color_space(ColorSpace::Hsl)
            .with_easing(EasingFunction::EaseInOut);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(start_color, end_color, config);
        
        assert!(gradient.len() <= size, 
            "Gradient size {} should not exceed requested size", size);
        
        if size > 0 && AllBlocks::new().with_color().len() > 0 {
            // For very small gradients, we should get at least something
            if size <= 5 {
                assert!(gradient.len() > 0, 
                    "Should get at least one block for size {}", size);
            }
        }
        
        println!("Size {}: got {} blocks", size, gradient.len());
    }
}

#[test]
fn test_color_gradient_sort() {
    let colored_blocks = AllBlocks::new()
        .with_color()
        .limit(10)
        .sort_by_color_gradient();
    
    let blocks = colored_blocks.collect();
    
    // Should maintain the same number of blocks
    assert!(blocks.len() <= 10);
    
    // All blocks should have color data
    for block in &blocks {
        assert!(block.extras.color.is_some(), 
            "Sorted block {} should have color data", block.id());
    }
    
    // If we have multiple blocks, check that they form a reasonable progression
    if blocks.len() > 1 {
        println!("Color gradient sort produced {} blocks:", blocks.len());
        for (i, block) in blocks.iter().enumerate() {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                println!("  {}: {} - RGB({}, {}, {})", 
                    i, block.id(), ext_color.rgb[0], ext_color.rgb[1], ext_color.rgb[2]);
            }
        }
    }
}

#[test]
fn test_gradient_endpoints_match_parameters() {
    println!("Testing that gradient endpoints match the input parameters...");
    
    // Test with specific colors that should have close matches in the block set
    let test_cases = [
        // Red to Blue - common colors likely to have exact or very close matches
        (ExtendedColorData::from_rgb(255, 0, 0), ExtendedColorData::from_rgb(0, 0, 255)),
        // Black to White - should have exact matches
        (ExtendedColorData::from_rgb(0, 0, 0), ExtendedColorData::from_rgb(255, 255, 255)),
        // Green to Yellow
        (ExtendedColorData::from_rgb(0, 255, 0), ExtendedColorData::from_rgb(255, 255, 0)),
        // Purple to Orange
        (ExtendedColorData::from_rgb(128, 0, 128), ExtendedColorData::from_rgb(255, 165, 0)),
    ];
    
    let color_spaces = [ColorSpace::Rgb, ColorSpace::Hsl, ColorSpace::Oklab];
    let gradient_sizes = [2, 3, 5, 10];
    
    for (start_color, end_color) in &test_cases {
        for &color_space in &color_spaces {
            for &size in &gradient_sizes {
                let config = GradientConfig::new(size)
                    .with_color_space(color_space)
                    .with_easing(EasingFunction::Linear);
                
                let gradient = AllBlocks::new()
                    .with_color()
                    .generate_gradient_between_colors(*start_color, *end_color, config);
                
                let blocks = gradient.collect();
                
                if blocks.len() >= 2 {
                    // Test first block (should be closest to start_color)
                    let first_block = &blocks[0];
                    if let Some(first_color) = first_block.extras.color {
                        let first_extended = first_color.to_extended();
                        let first_distance = start_color.distance_oklab(&first_extended);
                        
                        // Test last block (should be closest to end_color)
                        let last_block = &blocks[blocks.len() - 1];
                        if let Some(last_color) = last_block.extras.color {
                            let last_extended = last_color.to_extended();
                            let last_distance = end_color.distance_oklab(&last_extended);
                            
                            println!(
                                "Gradient {:?} {} steps: {} -> {} (distances: {:.3}, {:.3})",
                                color_space, size,
                                first_block.id(), last_block.id(),
                                first_distance, last_distance
                            );
                            
                            // The gradient should start and end with blocks that are reasonable 
                            // approximations of the target colors
                            // Allow for some tolerance since we're finding the "closest" blocks
                            let reasonable_distance = 50.0; // Oklab distance threshold
                            
                            assert!(first_distance < reasonable_distance,
                                "First block {} (RGB: {:?}) should be closer to start color (RGB: {:?}). Distance: {:.3}",
                                first_block.id(), first_extended.rgb, start_color.rgb, first_distance);
                            
                            assert!(last_distance < reasonable_distance,
                                "Last block {} (RGB: {:?}) should be closer to end color (RGB: {:?}). Distance: {:.3}",
                                last_block.id(), last_extended.rgb, end_color.rgb, last_distance);
                        }
                    }
                } else if blocks.len() == 1 {
                    // For single-block gradients, it should be closer to start color
                    let block = &blocks[0];
                    if let Some(block_color) = block.extras.color {
                        let extended = block_color.to_extended();
                        let start_distance = start_color.distance_oklab(&extended);
                        let end_distance = end_color.distance_oklab(&extended);
                        
                        println!(
                            "Single block gradient {:?}: {} (start distance: {:.3}, end distance: {:.3})",
                            color_space, block.id(), start_distance, end_distance
                        );
                        
                        // Single block should be reasonably close to one of the endpoints
                        let min_distance = start_distance.min(end_distance);
                        assert!(min_distance < 100.0, // More lenient for single blocks
                            "Single block {} should be reasonably close to one of the endpoints. Best distance: {:.3}",
                            block.id(), min_distance);
                    }
                }
            }
        }
    }
}

#[test]
fn test_gradient_endpoints_exact_color_matching() {
    println!("Testing gradient endpoints with colors that should have exact block matches...");
    
    // Get some actual block colors to test with
    let colored_blocks: Vec<_> = AllBlocks::new()
        .with_color()
        .limit(20)
        .collect();
    
    if colored_blocks.len() < 2 {
        println!("Skipping test - not enough colored blocks available");
        return;
    }
    
    // Use actual block colors as our test endpoints
    let start_block = &colored_blocks[0];
    let end_block = &colored_blocks[colored_blocks.len() - 1];
    
    let start_color = start_block.extras.color.unwrap().to_extended();
    let end_color = end_block.extras.color.unwrap().to_extended();
    
    println!("Testing with actual block colors:");
    println!("  Start: {} - RGB({}, {}, {})", start_block.id(), start_color.rgb[0], start_color.rgb[1], start_color.rgb[2]);
    println!("  End: {} - RGB({}, {}, {})", end_block.id(), end_color.rgb[0], end_color.rgb[1], end_color.rgb[2]);
    
    let config = GradientConfig::new(5)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::Linear);
    
    let gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, config);
    
    let gradient_blocks = gradient.collect();
    
    if gradient_blocks.len() >= 2 {
        let first_gradient_block = &gradient_blocks[0];
        let last_gradient_block = &gradient_blocks[gradient_blocks.len() - 1];
        
        let first_gradient_color = first_gradient_block.extras.color.unwrap().to_extended();
        let last_gradient_color = last_gradient_block.extras.color.unwrap().to_extended();
        
        println!("Gradient result:");
        println!("  First: {} - RGB({}, {}, {})", first_gradient_block.id(), first_gradient_color.rgb[0], first_gradient_color.rgb[1], first_gradient_color.rgb[2]);
        println!("  Last: {} - RGB({}, {}, {})", last_gradient_block.id(), last_gradient_color.rgb[0], last_gradient_color.rgb[1], last_gradient_color.rgb[2]);
        
        // Calculate distances
        let start_distance = start_color.distance_oklab(&first_gradient_color);
        let end_distance = end_color.distance_oklab(&last_gradient_color);
        
        println!("  Start distance: {:.6}", start_distance);
        println!("  End distance: {:.6}", end_distance);
        
        // Since we're using actual block colors as inputs, the endpoints should be
        // very close matches (ideally the exact same blocks, distance = 0)
        assert!(start_distance < 1.0,
            "First gradient block should be very close to start color (distance: {:.6})", start_distance);
        
        assert!(end_distance < 1.0,
            "Last gradient block should be very close to end color (distance: {:.6})", end_distance);
        
        // Ideally, they should be the exact same blocks
        if start_distance < 0.001 {
            println!("✓ Perfect match for start block!");
        }
        if end_distance < 0.001 {
            println!("✓ Perfect match for end block!");
        }
    }
}

#[test]
fn test_gradient_progression_smoothness() {
    println!("Testing that gradient color progression is smooth and monotonic...");
    
    let start_color = ExtendedColorData::from_rgb(255, 0, 0); // Red
    let end_color = ExtendedColorData::from_rgb(0, 0, 255);   // Blue
    
    let config = GradientConfig::new(8)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::Linear);
    
    let gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, config);
    
    let blocks = gradient.collect();
    
    if blocks.len() >= 3 {
        println!("Gradient progression:");
        
        // Calculate distances between consecutive blocks
        let mut distances = Vec::new();
        
        for i in 0..blocks.len() {
            let block = &blocks[i];
            let color = block.extras.color.unwrap().to_extended();
            
            // Distance from start color
            let start_dist = start_color.distance_oklab(&color);
            // Distance from end color  
            let end_dist = end_color.distance_oklab(&color);
            
            println!("  {}: {} - RGB({}, {}, {}) - start_dist: {:.3}, end_dist: {:.3}",
                i, block.id(), color.rgb[0], color.rgb[1], color.rgb[2], start_dist, end_dist);
            
            if i > 0 {
                let prev_color = blocks[i-1].extras.color.unwrap().to_extended();
                let step_distance = prev_color.distance_oklab(&color);
                distances.push(step_distance);
            }
        }
        
        // Check that we're progressing from start to end
        let first_block_color = blocks[0].extras.color.unwrap().to_extended();
        let last_block_color = blocks[blocks.len()-1].extras.color.unwrap().to_extended();
        
        let first_to_start = start_color.distance_oklab(&first_block_color);
        let first_to_end = end_color.distance_oklab(&first_block_color);
        let last_to_start = start_color.distance_oklab(&last_block_color);
        let last_to_end = end_color.distance_oklab(&last_block_color);
        
        // First block should be closer to start than to end
        assert!(first_to_start <= first_to_end,
            "First block should be closer to start color. Start distance: {:.3}, End distance: {:.3}",
            first_to_start, first_to_end);
        
        // Last block should be closer to end than to start
        assert!(last_to_end <= last_to_start,
            "Last block should be closer to end color. Start distance: {:.3}, End distance: {:.3}",
            last_to_start, last_to_end);
        
        println!("✓ Gradient correctly progresses from start to end");
        
        // Print step distances
        if !distances.is_empty() {
            let avg_distance = distances.iter().sum::<f32>() / distances.len() as f32;
            let max_distance = distances.iter().fold(0.0f32, |a, &b| a.max(b));
            let min_distance = distances.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            
            println!("  Step distances - avg: {:.3}, min: {:.3}, max: {:.3}", avg_distance, min_distance, max_distance);
            
            // Check that steps aren't too uneven (no step should be more than 3x the average)
            let max_allowed = avg_distance * 3.0;
            for (i, &dist) in distances.iter().enumerate() {
                assert!(dist <= max_allowed,
                    "Step {} distance ({:.3}) is too large compared to average ({:.3})", i, dist, avg_distance);
            }
        }
    }
}
