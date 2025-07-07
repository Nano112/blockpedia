use std::collections::HashMap;

// Core data structures
#[derive(Debug, Clone)]
pub struct BlockFacts {
    pub id: &'static str,
    pub properties: &'static [(&'static str, &'static [&'static str])],
    pub default_state: &'static [(&'static str, &'static str)],
    pub extras: Extras,
}

#[derive(Debug, Clone)]
pub struct Extras {
    // Future extension point for fetcher data
    pub mock_data: Option<i32>,
    pub color: Option<ColorData>,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorData {
    pub rgb: [u8; 3],
    pub oklab: [f32; 3],
}

impl ColorData {
    /// Convert to ExtendedColorData for palette operations
    pub fn to_extended(&self) -> color::ExtendedColorData {
        color::ExtendedColorData::from_rgb(self.rgb[0], self.rgb[1], self.rgb[2])
    }
}

impl From<color::ExtendedColorData> for ColorData {
    fn from(extended: color::ExtendedColorData) -> Self {
        ColorData {
            rgb: extended.rgb,
            oklab: extended.oklab,
        }
    }
}

impl Default for Extras {
    fn default() -> Self {
        Extras {
            mock_data: None,
            color: None,
        }
    }
}

impl Extras {
    pub const fn new() -> Self {
        Extras {
            mock_data: None,
            color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockState {
    block_id: String,
    properties: HashMap<String, String>,
}

impl BlockFacts {
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn properties(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        for (key, values) in self.properties {
            map.insert(
                key.to_string(),
                values.iter().map(|s| s.to_string()).collect(),
            );
        }
        map
    }

    pub fn has_property(&self, property: &str) -> bool {
        self.properties.iter().any(|(key, _)| *key == property)
    }

    pub fn get_property_values(&self, property: &str) -> Option<Vec<String>> {
        self.properties
            .iter()
            .find(|(key, _)| *key == property)
            .map(|(_, values)| values.iter().map(|s| s.to_string()).collect())
    }

    pub fn get_property(&self, property: &str) -> Option<&str> {
        self.default_state
            .iter()
            .find(|(key, _)| *key == property)
            .map(|(_, value)| *value)
    }
}

impl BlockState {
    pub fn new(block_id: &str) -> Result<Self> {
        // Validate block ID format first
        errors::validation::validate_block_id(block_id)?;

        // Validate block ID exists in our data
        if !BLOCKS.contains_key(block_id) {
            return Err(BlockpediaError::block_not_found(block_id));
        }

        Ok(BlockState {
            block_id: block_id.to_string(),
            properties: HashMap::new(),
        })
    }

    pub fn with(mut self, property: &str, value: &str) -> Result<Self> {
        // Validate property name format
        errors::validation::validate_property_name(property)?;

        // Validate property value format
        errors::validation::validate_property_value(value)?;

        // Get the block data to validate property and value
        let block_facts = BLOCKS
            .get(&self.block_id)
            .ok_or_else(|| BlockpediaError::block_not_found(&self.block_id))?;

        // Check if the property exists for this block
        if !block_facts.has_property(property) {
            return Err(BlockpediaError::property_not_found(
                &self.block_id,
                property,
            ));
        }

        // Check if the value is valid for this property
        let valid_values = block_facts.get_property_values(property).ok_or_else(|| {
            BlockpediaError::Property(errors::PropertyError::NoValues(property.to_string()))
        })?;

        if !valid_values.contains(&value.to_string()) {
            return Err(BlockpediaError::invalid_property_value(
                &self.block_id,
                property,
                value,
                valid_values,
            ));
        }

        self.properties
            .insert(property.to_string(), value.to_string());
        Ok(self)
    }

    /// Create a BlockState from the default state of a block
    pub fn from_default(block_facts: &BlockFacts) -> Result<Self> {
        let mut state = BlockState {
            block_id: block_facts.id().to_string(),
            properties: HashMap::new(),
        };

        // Set all default properties
        for (property, value) in block_facts.default_state {
            state
                .properties
                .insert(property.to_string(), value.to_string());
        }

        Ok(state)
    }

    /// Parse a blockstate string like "minecraft:repeater[delay=3,facing=north]"
    pub fn parse(blockstate_str: &str) -> Result<Self> {
        if let Some(bracket_pos) = blockstate_str.find('[') {
            // Block with properties
            let block_id = &blockstate_str[..bracket_pos];
            let properties_str = &blockstate_str[bracket_pos + 1..];

            if !properties_str.ends_with(']') {
                return Err(BlockpediaError::parse_failed(
                    blockstate_str,
                    "missing closing bracket",
                ));
            }

            let properties_str = &properties_str[..properties_str.len() - 1];
            let mut state = BlockState::new(block_id)?;

            if !properties_str.is_empty() {
                for prop_pair in properties_str.split(',') {
                    let parts: Vec<&str> = prop_pair.split('=').collect();
                    if parts.len() != 2 {
                        return Err(BlockpediaError::parse_failed(
                            blockstate_str,
                            &format!("invalid property format: {}", prop_pair),
                        ));
                    }
                    state = state.with(parts[0].trim(), parts[1].trim())?;
                }
            }

            Ok(state)
        } else {
            // Simple block without properties
            BlockState::new(blockstate_str)
        }
    }
}

impl std::fmt::Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.properties.is_empty() {
            write!(f, "{}", self.block_id)
        } else {
            let mut props = Vec::new();
            for (key, value) in &self.properties {
                props.push(format!("{}={}", key, value));
            }
            props.sort();
            write!(f, "{}[{}]", self.block_id, props.join(","))
        }
    }
}

// Include the generated block table
include!(concat!(env!("OUT_DIR"), "/block_table.rs"));

// Query utilities module
pub mod queries;
pub use queries::*;

// Fetcher framework module
pub mod fetchers;
pub use fetchers::*;

// Error handling module
pub mod errors;
pub use errors::{BlockpediaError, Result};

// Data sources module for multi-source support
pub mod data_sources;
pub use data_sources::*;

// Color processing module
pub mod color;
pub use color::ExtendedColorData;

// Query builder module for chained filtering
pub mod query_builder;
pub use query_builder::{
    AllBlocks, BlockQuery, ColorSamplingMethod, ColorSpace, EasingFunction, GradientConfig,
};

/// Get a block by its string ID
pub fn get_block(id: &str) -> Option<&'static BlockFacts> {
    BLOCKS.get(id).copied()
}

/// Get all blocks as an iterator
pub fn all_blocks() -> impl Iterator<Item = &'static BlockFacts> {
    BLOCKS.values().copied()
}

// WASM bindings
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod wasm;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm::*;

// Include tests
mod tests;
