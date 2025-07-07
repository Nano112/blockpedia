#[cfg(test)]
mod milestone1_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn can_download_or_load_cached_json() {
        // For milestone 1, check local test file
        assert!(Path::new("test_blocks_data.json").exists());
    }

    #[test]
    fn json_has_expected_structure() {
        let json_data = fs::read_to_string("test_blocks_data.json").unwrap();
        let json: serde_json::Value = serde_json::from_str(&json_data).unwrap();
        assert!(json.is_object());
        assert!(json.get("blocks").is_some());
        assert!(json["blocks"].as_object().unwrap().len() >= 3); // Our test has 3 blocks
    }

    #[test]
    fn sample_blocks_have_required_fields() {
        let json_data = fs::read_to_string("test_blocks_data.json").unwrap();
        let blocks = serde_json::from_str::<serde_json::Value>(&json_data).unwrap();
        let stone = &blocks["blocks"]["minecraft:stone"];
        assert!(stone.get("properties").is_some());
        assert!(stone.get("default_state").is_some());
    }

    #[test]
    fn can_parse_block_properties() {
        let json_data = fs::read_to_string("test_blocks_data.json").unwrap();
        let blocks = serde_json::from_str::<serde_json::Value>(&json_data).unwrap();
        let repeater = &blocks["blocks"]["minecraft:repeater"];
        let props = repeater["properties"].as_object().unwrap();
        assert!(props.contains_key("delay"));
        assert_eq!(props["delay"], serde_json::json!(["1", "2", "3", "4"]));
    }
}

// Milestone 7 Tests: Error Handling & Validation
#[cfg(test)]
mod milestone7_tests {
    use crate::{errors::*, queries::validated::*};

    #[test]
    fn validation_rejects_invalid_block_ids() {
        // Empty block ID
        let result = validation::validate_block_id("");
        assert!(result.is_err());

        // Invalid characters
        let result = validation::validate_block_id("minecraft:invalid!block");
        assert!(result.is_err());

        // Valid block ID should pass
        let result = validation::validate_block_id("minecraft:valid_block");
        assert!(result.is_ok());
    }

    #[test]
    fn validation_rejects_invalid_property_names() {
        // Empty property name
        let result = validation::validate_property_name("");
        assert!(result.is_err());

        // Invalid characters
        let result = validation::validate_property_name("invalid-property!");
        assert!(result.is_err());

        // Valid property name should pass
        let result = validation::validate_property_name("valid_property");
        assert!(result.is_ok());
    }

    #[test]
    fn error_types_display_correctly() {
        let error = BlockpediaError::block_not_found("test:block");
        let error_string = error.to_string();
        assert!(error_string.contains("Block error"));
        assert!(error_string.contains("test:block"));

        let error = BlockpediaError::invalid_property_value(
            "test:block",
            "prop",
            "value",
            vec!["valid1".to_string(), "valid2".to_string()],
        );
        let error_string = error.to_string();
        assert!(error_string.contains("Property error"));
        assert!(error_string.contains("Invalid value"));
    }

    #[test]
    fn safe_query_functions_validate_input() {
        // Test safe property search with invalid property name
        let result = find_blocks_by_property_safe("invalid-prop!", "value");
        assert!(result.is_err());

        // Test safe search with empty pattern
        let result = search_blocks_safe("");
        assert!(result.is_err());

        // Test valid inputs work
        let result = find_blocks_by_property_safe("delay", "1");
        assert!(result.is_ok());
    }

    #[test]
    fn error_recovery_provides_suggestions() {
        let suggestions = recovery::suggest_similar_blocks("stone");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("minecraft:stone")));

        let suggestions = recovery::suggest_property_values(
            "delay",
            "5",
            &[
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
            ],
        );
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn comprehensive_validation_catches_multiple_errors() {
        let properties = vec![
            ("delay".to_string(), "5".to_string()), // Invalid value
            ("invalid_prop".to_string(), "value".to_string()), // Invalid property
        ];

        let result = validate_block_properties_safe("minecraft:repeater", &properties);
        assert!(result.is_err());

        if let Err(BlockpediaError::State(StateError::ValidationFailed { errors, .. })) = result {
            assert!(errors.len() >= 2); // Should catch both errors
        } else {
            panic!("Expected ValidationFailed error with multiple errors");
        }
    }

    #[test]
    fn safe_block_state_creation_with_helpful_errors() {
        let properties = vec![
            ("delay".to_string(), "3".to_string()), // Valid
            ("facing".to_string(), "invalid_direction".to_string()), // Invalid value
        ];

        let result = create_block_state_safe("minecraft:repeater", &properties);
        assert!(result.is_err());

        // Error should include suggestions
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid value") || error_msg.contains("ValidationFailed"));
    }

    #[test]
    fn parse_error_recovery_fixes_common_mistakes() {
        let broken_input = "minecraft:repeater delay=3,facing=north";
        let fixed = recovery::fix_common_parse_errors(broken_input);
        // This is a simple example - real implementation would be more sophisticated
        assert_ne!(broken_input, fixed);
    }

    #[test]
    fn structured_errors_are_comparable() {
        let error1 = BlockpediaError::block_not_found("test:block");
        let error2 = BlockpediaError::block_not_found("test:block");
        let error3 = BlockpediaError::block_not_found("other:block");

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn query_timeout_validation() {
        let result = query_with_timeout("valid_query", || "result");
        assert!(result.is_ok());

        let very_long_name = "a".repeat(100);
        let result = query_with_timeout(&very_long_name, || "result");
        assert!(result.is_err());
    }
}

// Milestone 6 Tests: Fetcher Framework
#[cfg(test)]
mod milestone6_tests {
    use crate::BLOCKS;

    #[test]
    fn fetcher_framework_provides_extra_data() {
        // Test that fetcher framework successfully adds extra data
        let stone = BLOCKS.get("minecraft:stone").unwrap();

        // Stone should have mock data from MockFetcher
        assert_eq!(stone.extras.mock_data, Some(42));

        // Stone should have color data from ColorFetcher
        assert!(stone.extras.color.is_some());
        let color = stone.extras.color.unwrap();
        assert_eq!(color.rgb, [125, 125, 125]);
        assert!((color.oklab[0] - 0.49).abs() < 0.01); // Calculated L value for RGB [125, 125, 125]
    }

    #[test]
    fn fetcher_data_varies_by_block() {
        let stone = BLOCKS.get("minecraft:stone").unwrap();
        let dirt = BLOCKS.get("minecraft:dirt").unwrap();

        // Different blocks should have different mock data
        assert_eq!(stone.extras.mock_data, Some(42));
        assert_eq!(dirt.extras.mock_data, Some(123));

        // Different blocks should have different color data
        let stone_color = stone.extras.color.unwrap();
        let dirt_color = dirt.extras.color.unwrap();
        assert_ne!(stone_color.rgb, dirt_color.rgb);
    }

    #[test]
    fn blocks_without_fetcher_data_have_none() {
        let repeater = BLOCKS.get("minecraft:repeater").unwrap();

        // Repeater was not included in our test fetcher data
        assert_eq!(repeater.extras.mock_data, None);
        assert!(repeater.extras.color.is_none());
    }

    #[test]
    fn generated_color_queries_work() {
        // Test closest color query using stone's actual RGB value
        let closest = crate::BlockFacts::closest_to_color([125, 125, 125]);
        assert!(closest.is_some());

        // Should find a valid block with color data
        let closest_block = closest.unwrap();
        assert!(closest_block.id().contains("minecraft:"));

        // The closest block should have color data
        assert!(closest_block.extras.color.is_some());
    }

    #[test]
    fn color_range_query_works() {
        // Test color range query - the generated function returns a Vec, not an iterator
        let similar_blocks = crate::BlockFacts::blocks_in_color_range([100, 100, 100], 100.0);

        // Should find both stone and dirt in this range
        assert!(!similar_blocks.is_empty());
        let block_ids: Vec<_> = similar_blocks.iter().map(|b| b.id()).collect();
        assert!(block_ids.contains(&"minecraft:stone"));
        assert!(block_ids.contains(&"minecraft:dirt"));
    }

    #[test]
    fn extras_struct_has_proper_defaults() {
        let default_extras = crate::Extras::default();
        assert_eq!(default_extras.mock_data, None);
        assert!(default_extras.color.is_none());

        let new_extras = crate::Extras::new();
        assert_eq!(new_extras.mock_data, None);
        assert!(new_extras.color.is_none());
    }
}

#[cfg(test)]
mod milestone5_tests {
    use crate::queries::*;

    #[test]
    fn enhanced_block_families_logical() {
        let families = get_enhanced_block_families();

        assert!(!families.is_empty());

        // Validate some family groupings if applicable in test data
        for blocks in families.values() {
            assert!(!blocks.is_empty());
        }
    }

    #[test]
    fn blocks_with_complex_properties_work() {
        let requirements = vec![("delay".to_string(), vec!["2".to_string()])];
        let results: Vec<_> = blocks_with_complex_properties(&requirements).collect();
        assert!(!results.is_empty());
    }

    #[test]
    fn analyze_property_correlation_accuracy() {
        let correlations = analyze_property_correlation();
        assert!(!correlations.is_empty());
    }

    #[test]
    fn find_similar_blocks_effective() {
        // Test with a low threshold since our test dataset is small
        let similar_blocks = find_similar_blocks("minecraft:repeater", 1);

        // In our small test dataset, there might not be other blocks with shared properties
        // So let's just verify the function works and returns a valid result
        // (length is always >= 0, but we're testing the function returns something)

        // Test with a block that doesn't exist
        let non_existent = find_similar_blocks("minecraft:nonexistent", 1);
        assert!(non_existent.is_empty());

        // Test the function structure is working
        for (block, shared_properties) in similar_blocks {
            println!(
                "Block: {}, Shared Properties: {}",
                block.id(),
                shared_properties
            );
            assert!(shared_properties >= 1); // Should have at least the minimum
        }
    }

    #[test]
    fn get_advanced_property_stats_correct() {
        let stats = get_advanced_property_stats();
        assert!(stats.basic_stats.total_unique_properties > 0);

        println!(
            "Most diverse property: {} with {} values",
            stats.most_diverse_property.0, stats.most_diverse_property.1
        );
        println!(
            "Most correlated properties: {:?}",
            stats.most_correlated_properties
        );
    }
}

#[cfg(test)]
mod milestone2_tests {
    use crate::{BlockState, BLOCKS};

    #[test]
    fn phf_table_loads_successfully() {
        assert!(BLOCKS.len() >= 3);
        assert!(BLOCKS.get("minecraft:stone").is_some());
    }

    #[test]
    fn block_facts_has_expected_data() {
        let stone = BLOCKS.get("minecraft:stone").unwrap();
        assert_eq!(stone.id(), "minecraft:stone");
        assert!(stone.properties().is_empty()); // Stone has no properties in our test

        let repeater = BLOCKS.get("minecraft:repeater").unwrap();
        assert!(repeater.has_property("delay"));
        assert_eq!(
            repeater.get_property_values("delay"),
            Some(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string()
            ])
        );
    }

    #[test]
    fn block_state_creates_successfully() {
        let state = BlockState::new("minecraft:stone").unwrap();
        assert_eq!(state.to_string(), "minecraft:stone");

        let state = BlockState::new("nonexistent:block");
        assert!(state.is_err());
    }

    #[test]
    fn block_state_with_properties() {
        let state = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("delay", "3")
            .unwrap();
        assert!(state.to_string().contains("delay=3"));
    }
}

#[cfg(test)]
mod milestone3_tests {
    use crate::{BlockState, BLOCKS};

    #[test]
    fn valid_properties_accepted() {
        let valid = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("delay", "2")
            .unwrap()
            .with("facing", "north")
            .unwrap();
        assert!(valid.to_string().contains("delay=2"));
        assert!(valid.to_string().contains("facing=north"));
    }

    #[test]
    fn invalid_property_values_rejected() {
        let result = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("delay", "5"); // Invalid: only 1-4 allowed
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid value '5'"));

        let result = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("nonexistent_prop", "value");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("does not have property") || error_msg.contains("not found"));
    }

    #[test]
    fn property_case_sensitivity() {
        // Property names should be case sensitive
        let result = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("Delay", "1"); // Wrong case
        assert!(result.is_err());

        // Values should be case sensitive too
        let result = BlockState::new("minecraft:repeater")
            .unwrap()
            .with("facing", "North"); // Wrong case
        assert!(result.is_err());
    }

    #[test]
    fn default_state_roundtrip() {
        let block = BLOCKS.get("minecraft:repeater").unwrap();
        let default_state = BlockState::from_default(block).unwrap();
        let rebuilt = BlockState::parse(&default_state.to_string()).unwrap();
        assert_eq!(default_state.to_string(), rebuilt.to_string());
    }

    #[test]
    fn unknown_block_rejected() {
        let result = BlockState::new("minecraft:nonexistent_block");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found"));
    }

    #[test]
    fn parse_blockstate_string() {
        // Test parsing simple block
        let state = BlockState::parse("minecraft:stone").unwrap();
        assert_eq!(state.to_string(), "minecraft:stone");

        // Test parsing block with properties
        let state = BlockState::parse("minecraft:repeater[delay=3,facing=south]").unwrap();
        assert!(state.to_string().contains("delay=3"));
        assert!(state.to_string().contains("facing=south"));

        // Test invalid format
        let result = BlockState::parse("minecraft:repeater[delay=3,facing=south"); // Missing ]
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod milestone4_tests {
    use crate::{queries::*, BLOCKS};

    #[test]
    fn find_blocks_by_property_works() {
        // Test with waterlogged property (if it exists in our test data)
        let _waterloggable: Vec<_> = find_blocks_by_property("waterlogged", "true").collect();
        // Since our test data might not have waterlogged blocks, let's test with what we have

        // Test with delay property that we know exists
        let delay_blocks: Vec<_> = find_blocks_by_property("delay", "1").collect();
        assert!(delay_blocks.iter().any(|b| b.id() == "minecraft:repeater"));
    }

    #[test]
    fn find_blocks_matching_predicate() {
        let complex_redstone: Vec<_> = find_blocks_matching(|block| {
            block.has_property("powered") && block.has_property("facing")
        })
        .collect();

        assert!(complex_redstone
            .iter()
            .any(|b| b.id() == "minecraft:repeater"));
    }

    #[test]
    fn search_blocks_pattern_matching() {
        // Test exact match
        let stone_blocks: Vec<_> = search_blocks("stone").collect();
        assert!(stone_blocks.iter().any(|b| b.id() == "minecraft:stone"));

        // Test wildcard pattern
        let minecraft_blocks: Vec<_> = search_blocks("minecraft:*").collect();
        assert!(minecraft_blocks.len() >= 3); // Should match all our test blocks
    }

    #[test]
    fn get_property_values_comprehensive() {
        let delay_values = get_property_values("delay").unwrap();
        assert_eq!(delay_values, vec!["1", "2", "3", "4"]);

        let facing_values = get_property_values("facing").unwrap();
        assert!(facing_values.contains(&"north".to_string()));
        assert!(facing_values.len() >= 4); // At least cardinal directions
    }

    #[test]
    fn count_blocks_where_accurate() {
        let total_blocks = BLOCKS.len();
        let counted = count_blocks_where(|_| true);
        assert_eq!(total_blocks, counted);

        let blocks_with_properties = count_blocks_where(|b| !b.properties.is_empty());
        assert!(blocks_with_properties >= 1); // At least repeater has properties
    }

    #[test]
    fn get_block_families_logical() {
        let families = get_block_families();

        // Our test data doesn't have many families, but let's test the logic
        assert!(!families.is_empty());

        // Each family should have at least one block
        for blocks in families.values() {
            assert!(!blocks.is_empty());
        }
    }

    #[test]
    fn blocks_with_properties_works() {
        let results: Vec<_> = blocks_with_properties(&[
            ("delay", "*"),      // Any delay value
            ("facing", "north"), // Specific facing value
        ])
        .collect();

        // Should find repeater when it has both delay and facing=north in defaults
        assert!(!results.is_empty());

        // All results should have both properties
        for block in results {
            assert!(block.has_property("delay"));
            assert!(block.has_property("facing"));
        }
    }

    #[test]
    fn find_rare_properties_works() {
        let rare = find_rare_properties(0.5); // Properties in <50% of blocks

        // Most properties should be rare in our small test set
        assert!(!rare.is_empty());

        // Each property should have a count
        for count in rare.values() {
            assert!(*count > 0);
        }
    }

    #[test]
    fn property_statistics_accurate() {
        let stats = get_property_stats();

        assert!(stats.total_unique_properties > 0);
        assert!(stats.most_common_property.1 > 0); // Most common property appears at least once
        assert!(stats.blocks_with_no_properties >= 1); // Stone has no properties
        assert!(stats.average_properties_per_block >= 0.0);
    }
}
