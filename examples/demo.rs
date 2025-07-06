use blockpedia::{BLOCKS, BlockState, queries::*};

fn main() {
    println!("Blockpedia Advanced Demo (Milestones 1-5)");
    println!("=========================================");
    
    // Show PHF table loading
    println!("Loaded {} blocks into PHF table", BLOCKS.len());
    
    // Demonstrate block access
    let stone = BLOCKS.get("minecraft:stone").unwrap();
    println!("\nStone block:");
    println!("  ID: {}", stone.id());
    println!("  Properties: {:?}", stone.properties());
    
    let repeater = BLOCKS.get("minecraft:repeater").unwrap();
    println!("\nRepeater block:");
    println!("  ID: {}", repeater.id());
    println!("  Has delay property: {}", repeater.has_property("delay"));
    println!("  Delay values: {:?}", repeater.get_property_values("delay"));
    println!("  Default delay: {:?}", repeater.get_property("delay"));
    
    // Demonstrate BlockState building
    println!("\nBlockState demonstration:");
    let simple_state = BlockState::new("minecraft:stone").unwrap();
    println!("  Simple state: {}", simple_state);
    
    let complex_state = BlockState::new("minecraft:repeater")
        .unwrap()
        .with("delay", "3")
        .unwrap()
        .with("facing", "north")
        .unwrap();
    println!("  Complex state: {}", complex_state);
    
    // Demonstrate validation features (Milestone 3)
    println!("\nValidation demonstration:");
    
    // Show error for unknown block
    match BlockState::new("minecraft:nonexistent") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Unknown block rejected: {}", e),
    }
    
    // Show error for invalid property
    match BlockState::new("minecraft:repeater").unwrap().with("invalid_prop", "value") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Invalid property rejected: {}", e),
    }
    
    // Show error for invalid value
    match BlockState::new("minecraft:repeater").unwrap().with("delay", "5") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Invalid value rejected: {}", e),
    }
    
    // Show parsing functionality
    let parsed = BlockState::parse("minecraft:repeater[delay=4,facing=east]").unwrap();
    println!("  ✓ Parsed state: {}", parsed);
    
    // Show default state
    let default = BlockState::from_default(repeater).unwrap();
    println!("  ✓ Default repeater state: {}", default);
    
    // Demonstrate advanced queries and families (Milestone 5)
    println!("\nAdvanced queries and families demonstration:");

    // Get enhanced block families
    let enhanced_families = get_enhanced_block_families();
    println!("  ✓ Enhanced block families found: {:?}", enhanced_families.keys().collect::<Vec<_>>());

    // Find blocks with complex properties
    let complex_blocks: Vec<_> = blocks_with_complex_properties(&[("delay".to_string(), vec!["2".to_string()])]).collect();
    println!("  ✓ Blocks with delay=2: {:?}", complex_blocks.iter().map(|b| b.id()).collect::<Vec<_>>());

    // Analyze property correlation
    let correlations = analyze_property_correlation();
    if let Some(corr) = correlations.get("facing") {
        println!("  ✓ Property correlations for 'facing': {:?}", corr);
    }

    // Find similar blocks
    let similar_blocks = find_similar_blocks("minecraft:repeater", 1);
    println!("  ✓ Blocks similar to 'minecraft:repeater': {:?}", similar_blocks.iter().map(|(b, count)| (b.id(), count)).collect::<Vec<_>>());

    // Get advanced property stats
    let advanced_stats = get_advanced_property_stats();
    println!("  ✓ Advanced property stats:");
    println!("    - Most diverse property: {} with {} values", advanced_stats.most_diverse_property.0, advanced_stats.most_diverse_property.1);
    println!("    - Most correlated properties: {:?}", advanced_stats.most_correlated_properties);
    
    // Find blocks by property
    let delay_blocks: Vec<_> = find_blocks_by_property("delay", "1").collect();
    println!("  ✓ Blocks with delay=1: {:?}", delay_blocks.iter().map(|b| b.id()).collect::<Vec<_>>());
    
    // Find blocks matching predicate
    let redstone_blocks: Vec<_> = find_blocks_matching(|block| {
        block.has_property("powered")
    }).collect();
    println!("  ✓ Blocks with 'powered' property: {:?}", redstone_blocks.iter().map(|b| b.id()).collect::<Vec<_>>());
    
    // Search with patterns
    let minecraft_blocks: Vec<_> = search_blocks("minecraft:*").collect();
    println!("  ✓ All minecraft blocks: {:?}", minecraft_blocks.iter().map(|b| b.id()).collect::<Vec<_>>());
    
    // Get property values
    if let Some(facing_values) = get_property_values("facing") {
        println!("  ✓ All possible 'facing' values: {:?}", facing_values);
    }
    
    // Count blocks
    let blocks_with_props = count_blocks_where(|b| !b.properties.is_empty());
    println!("  ✓ Blocks with properties: {}", blocks_with_props);
    
    // Block families
    let families = get_block_families();
    println!("  ✓ Block families found: {:?}", families.keys().collect::<Vec<_>>());
    
    // Statistics
    let stats = get_property_stats();
    println!("  ✓ Property statistics:");
    println!("    - Total unique properties: {}", stats.total_unique_properties);
    println!("    - Most common property: {} (appears in {} blocks)", stats.most_common_property.0, stats.most_common_property.1);
    println!("    - Blocks with no properties: {}", stats.blocks_with_no_properties);
    println!("    - Average properties per block: {:.1}", stats.average_properties_per_block);
    
    // Demonstrate Milestone 6: Fetcher Framework
    println!("\nFetcher Framework demonstration:");
    
    // Show extra data from fetchers
    for block in ["minecraft:stone", "minecraft:dirt", "minecraft:grass_block"] {
        if let Some(block_facts) = BLOCKS.get(block) {
            println!("  Block: {}", block);
            
            // Show mock data if available
            if let Some(mock_value) = block_facts.extras.mock_data {
                println!("    - Mock data: {}", mock_value);
            }
            
            // Show color data if available
            if let Some(color) = block_facts.extras.color {
                println!("    - RGB color: {:?}", color.rgb);
                println!("    - Oklab color: [{:.2}, {:.2}, {:.2}]", color.oklab[0], color.oklab[1], color.oklab[2]);
            }
        }
    }
    
    // Test generated query helpers from fetchers (if available)
    println!("\n  Testing color-based queries:");
    
    // Test closest color query (if the generated function exists)
    if let Some(closest) = blockpedia::BlockFacts::closest_to_color([128, 128, 128]) {
        println!("    - Closest block to gray (128,128,128): {}", closest.id());
    }
    
    // Test color range query
    let similar_colors = blockpedia::BlockFacts::blocks_in_color_range([100, 100, 100], 50.0);
    let similar_ids: Vec<_> = similar_colors.iter().map(|b| b.id()).collect();
    println!("    - Blocks with similar colors to dark gray: {:?}", similar_ids);
    
    // Demonstrate Milestone 7: Error Handling & Validation
    println!("\nError Handling & Validation demonstration:");
    
    // Show validation catching invalid block IDs
    match blockpedia::errors::validation::validate_block_id("invalid!block:name") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Invalid block ID rejected: {}", e),
    }
    
    // Show validation catching invalid property names
    match blockpedia::errors::validation::validate_property_name("invalid-prop!") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Invalid property name rejected: {}", e),
    }
    
    // Demonstrate safe query functions with helpful error messages
    match blockpedia::queries::validated::find_blocks_by_property_safe("nonexistent_prop", "value") {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Safe query caught error: {}", e),
    }
    
    // Demonstrate error recovery suggestions
    let suggestions = blockpedia::errors::recovery::suggest_similar_blocks("stone");
    println!("  ✓ Error recovery suggests for 'stone': {:?}", suggestions);
    
    // Show comprehensive validation with multiple errors
    let invalid_properties = vec![
        ("delay".to_string(), "5".to_string()), // Invalid value
        ("invalid_prop".to_string(), "value".to_string()), // Invalid property
    ];
    
    match blockpedia::queries::validated::validate_block_properties_safe("minecraft:repeater", &invalid_properties) {
        Ok(_) => println!("  ERROR: Should have failed!"),
        Err(e) => println!("  ✓ Comprehensive validation caught multiple errors:\n    {}", e),
    }
    
    // Demonstrate structured error types
    let error1 = blockpedia::BlockpediaError::block_not_found("test:block");
    let error2 = blockpedia::BlockpediaError::block_not_found("test:block");
    println!("  ✓ Structured errors are comparable: {}", error1 == error2);
    
    // Show error types with proper formatting
    let prop_error = blockpedia::BlockpediaError::invalid_property_value(
        "minecraft:repeater", "delay", "5", vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]
    );
    println!("  ✓ Detailed error message: {}", prop_error);
}
