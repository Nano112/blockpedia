use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

// Import our data source types
mod data_sources_build {
    use anyhow::{Context, Result};
    use serde_json::Value;
    use std::collections::HashMap;

    /// Unified block data structure for build script
    #[derive(Debug, Clone)]
    pub struct UnifiedBlockData {
        pub id: String,
        pub properties: HashMap<String, Vec<String>>,
        pub default_state: HashMap<String, String>,
        #[allow(dead_code)] // Used for future extensions
        pub extra_properties: HashMap<String, Value>,
    }

    /// Trait for different data source adapters in build script
    pub trait DataSourceAdapter {
        fn name(&self) -> &'static str;
        fn fetch_url(&self) -> &'static str;
        fn parse_data(&self, json_data: &str) -> Result<Vec<UnifiedBlockData>>;
        fn validate_structure(&self, json: &Value) -> Result<()>;
    }

    /// PrismarineJS adapter for build script
    pub struct PrismarineAdapter;

    impl DataSourceAdapter for PrismarineAdapter {
        fn name(&self) -> &'static str {
            "PrismarineJS"
        }

        fn fetch_url(&self) -> &'static str {
            "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/pc/1.20.4/blocks.json"
        }

        fn parse_data(&self, json_data: &str) -> Result<Vec<UnifiedBlockData>> {
            let parsed: Value =
                serde_json::from_str(json_data).context("Failed to parse PrismarineJS JSON")?;

            let blocks_array = parsed
                .as_array()
                .context("PrismarineJS JSON is not an array")?;

            let mut unified_blocks = Vec::new();

            for block in blocks_array {
                let block_obj = block.as_object().context("Block is not an object")?;

                let name = block_obj
                    .get("name")
                    .and_then(|n| n.as_str())
                    .context("Block missing name field")?;

                let id = format!("minecraft:{name}");

                // Convert states to properties
                let mut properties = HashMap::new();
                if let Some(states) = block_obj.get("states").and_then(|s| s.as_array()) {
                    for state in states {
                        if let Some(state_obj) = state.as_object() {
                            if let (Some(prop_name), Some(prop_type), Some(num_values)) = (
                                state_obj.get("name").and_then(|n| n.as_str()),
                                state_obj.get("type").and_then(|t| t.as_str()),
                                state_obj.get("num_values").and_then(|n| n.as_u64()),
                            ) {
                                let values = match prop_type {
                                    "bool" => vec!["false".to_string(), "true".to_string()],
                                    "int" => {
                                        if let Some(values_array) =
                                            state_obj.get("values").and_then(|v| v.as_array())
                                        {
                                            values_array
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect()
                                        } else {
                                            (0..num_values).map(|i| i.to_string()).collect()
                                        }
                                    }
                                    "enum" => {
                                        if let Some(values_array) =
                                            state_obj.get("values").and_then(|v| v.as_array())
                                        {
                                            values_array
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect()
                                        } else {
                                            (0..num_values).map(|i| format!("value_{i}")).collect()
                                        }
                                    }
                                    _ => vec!["unknown".to_string()],
                                };
                                properties.insert(prop_name.to_string(), values);
                            }
                        }
                    }
                }

                // Extract extra properties from original data
                let mut extra_properties = HashMap::new();
                if let Some(hardness) = block_obj.get("hardness") {
                    extra_properties.insert("hardness".to_string(), hardness.clone());
                }
                if let Some(resistance) = block_obj.get("resistance") {
                    extra_properties.insert("resistance".to_string(), resistance.clone());
                }

                unified_blocks.push(UnifiedBlockData {
                    id,
                    properties,
                    default_state: HashMap::new(), // PrismarineJS doesn't provide default states
                    extra_properties,
                });
            }

            Ok(unified_blocks)
        }

        fn validate_structure(&self, json: &Value) -> Result<()> {
            let blocks_array = json
                .as_array()
                .context("PrismarineJS JSON is not a valid array")?;

            if blocks_array.is_empty() {
                anyhow::bail!("No blocks found in PrismarineJS data");
            }

            // Validate a few sample blocks
            for (i, block_data) in blocks_array.iter().take(3).enumerate() {
                let block_obj = block_data
                    .as_object()
                    .with_context(|| format!("Block at index {i} is not an object"))?;

                if !block_obj.contains_key("name") {
                    anyhow::bail!("Block at index {} missing 'name' field", i);
                }
            }

            Ok(())
        }
    }

    /// MCPropertyEncyclopedia adapter for build script
    pub struct MCPropertyEncyclopediaAdapter;

    impl DataSourceAdapter for MCPropertyEncyclopediaAdapter {
        fn name(&self) -> &'static str {
            "MCPropertyEncyclopedia"
        }

        fn fetch_url(&self) -> &'static str {
            "https://raw.githubusercontent.com/JoakimThorsen/MCPropertyEncyclopedia/main/data/block_data.json"
        }

        fn parse_data(&self, json_data: &str) -> Result<Vec<UnifiedBlockData>> {
            let parsed: Value = serde_json::from_str(json_data)
                .context("Failed to parse MCPropertyEncyclopedia JSON")?;

            let key_list = parsed
                .get("key_list")
                .and_then(|k| k.as_array())
                .context("Missing or invalid key_list")?;

            let properties_obj = parsed
                .get("properties")
                .and_then(|p| p.as_object())
                .context("Missing or invalid properties")?;

            let mut unified_blocks = Vec::new();

            for block_name in key_list {
                let block_name_str = block_name.as_str().context("Block name is not a string")?;

                // Convert display name to minecraft ID format
                let id = format!(
                    "minecraft:{}",
                    block_name_str
                        .to_lowercase()
                        .replace(" ", "_")
                        .replace("(", "")
                        .replace(")", "")
                        .replace("-", "_")
                        .replace("'", "")
                        .replace("!", "")
                        .replace(".", "_")
                );

                let mut extra_properties = HashMap::new();

                // Extract all properties for this block
                for (prop_name, prop_data) in properties_obj {
                    if let Some(entries) = prop_data.get("entries").and_then(|e| e.as_object()) {
                        if let Some(value) = entries.get(block_name_str) {
                            extra_properties.insert(prop_name.clone(), value.clone());
                        }
                    }
                }

                unified_blocks.push(UnifiedBlockData {
                    id,
                    properties: HashMap::new(), // MCPropertyEncyclopedia doesn't have block states
                    default_state: HashMap::new(),
                    extra_properties,
                });
            }

            Ok(unified_blocks)
        }

        fn validate_structure(&self, json: &Value) -> Result<()> {
            let _key_list = json
                .get("key_list")
                .and_then(|k| k.as_array())
                .context("Missing or invalid key_list")?;

            let _properties = json
                .get("properties")
                .and_then(|p| p.as_object())
                .context("Missing or invalid properties")?;

            Ok(())
        }
    }

    /// Registry for managing multiple data sources in build script
    pub struct DataSourceRegistry {
        sources: Vec<Box<dyn DataSourceAdapter>>,
        primary_source: Option<usize>,
    }

    impl DataSourceRegistry {
        pub fn new() -> Self {
            Self {
                sources: Vec::new(),
                primary_source: None,
            }
        }

        pub fn register_source(&mut self, source: Box<dyn DataSourceAdapter>) {
            self.sources.push(source);

            // Set first source as primary if none set
            if self.primary_source.is_none() {
                self.primary_source = Some(0);
            }
        }

        pub fn set_primary_source(&mut self, name: &str) -> Result<()> {
            for (i, source) in self.sources.iter().enumerate() {
                if source.name() == name {
                    self.primary_source = Some(i);
                    return Ok(());
                }
            }
            anyhow::bail!("Data source '{}' not found", name);
        }

        pub fn get_primary_source(&self) -> Result<&dyn DataSourceAdapter> {
            let index = self.primary_source.context("No primary data source set")?;
            Ok(self.sources[index].as_ref())
        }

        pub fn list_sources(&self) -> Vec<&str> {
            self.sources.iter().map(|s| s.name()).collect()
        }

        pub fn fetch_unified_data(&self) -> Result<Vec<UnifiedBlockData>> {
            let primary = self.get_primary_source()?;

            // Try to fetch from primary source with cache and fallback
            match self.fetch_with_fallback(primary) {
                Ok(blocks) => Ok(blocks),
                Err(e) => {
                    println!("cargo:warning=All data sources failed: {e}");
                    anyhow::bail!("Could not fetch data from any source: {}", e)
                }
            }
        }

        fn fetch_with_fallback(
            &self,
            primary: &dyn DataSourceAdapter,
        ) -> Result<Vec<UnifiedBlockData>> {
            // Try primary source
            match self.try_fetch_source(primary) {
                Ok(blocks) => {
                    println!(
                        "cargo:warning=Successfully fetched {} blocks from {}",
                        blocks.len(),
                        primary.name()
                    );
                    return Ok(blocks);
                }
                Err(e) => {
                    println!(
                        "cargo:warning=Failed to fetch from {} ({})",
                        primary.name(),
                        e
                    );
                }
            }

            // Try other sources as fallback
            for source in &self.sources {
                if source.name() != primary.name() {
                    match self.try_fetch_source(source.as_ref()) {
                        Ok(blocks) => {
                            println!(
                                "cargo:warning=Successfully fell back to {} and fetched {} blocks",
                                source.name(),
                                blocks.len()
                            );
                            return Ok(blocks);
                        }
                        Err(e) => {
                            println!(
                                "cargo:warning=Fallback to {} also failed: {}",
                                source.name(),
                                e
                            );
                        }
                    }
                }
            }

            anyhow::bail!("All data sources failed to provide data")
        }

        fn try_fetch_source(
            &self,
            source: &dyn DataSourceAdapter,
        ) -> Result<Vec<UnifiedBlockData>> {
            let url = source.fetch_url();
            println!(
                "cargo:warning=Fetching data from {} using {}",
                url,
                source.name()
            );

            // Check cache first
            let cache_key = format!("{}_data.json", source.name().to_lowercase());
            let cache_path = std::env::var("OUT_DIR")
                .map(|out_dir| std::path::Path::new(&out_dir).join(&cache_key))
                .unwrap_or_else(|_| std::path::PathBuf::from(&cache_key));

            // Try to load from cache first (for faster rebuilds)
            if cache_path.exists() {
                if let Ok(cached_data) = std::fs::read_to_string(&cache_path) {
                    if let Ok(parsed) = serde_json::from_str::<Value>(&cached_data) {
                        if source.validate_structure(&parsed).is_ok() {
                            if let Ok(blocks) = source.parse_data(&cached_data) {
                                println!("cargo:warning=Using cached data for {}", source.name());
                                return Ok(blocks);
                            }
                        }
                    }
                }
                println!(
                    "cargo:warning=Cache invalid for {}, re-downloading",
                    source.name()
                );
            }

            // Download fresh data
            let json_data = download_from_url(url)
                .with_context(|| format!("Failed to download from {}", source.name()))?;

            // Validate structure
            let parsed: Value = serde_json::from_str(&json_data)
                .with_context(|| format!("Failed to parse JSON from {}", source.name()))?;
            source
                .validate_structure(&parsed)
                .with_context(|| format!("Data validation failed for {}", source.name()))?;

            // Parse into unified format
            let blocks = source
                .parse_data(&json_data)
                .with_context(|| format!("Failed to parse data from {}", source.name()))?;

            // Cache the successful data
            if let Err(e) = std::fs::write(&cache_path, &json_data) {
                println!(
                    "cargo:warning=Failed to cache data for {}: {}",
                    source.name(),
                    e
                );
            } else {
                println!(
                    "cargo:warning=Cached data for {} for future builds",
                    source.name()
                );
            }

            Ok(blocks)
        }
    }

    impl Default for DataSourceRegistry {
        fn default() -> Self {
            let mut registry = Self::new();

            // Register default sources
            registry.register_source(Box::new(PrismarineAdapter));
            registry.register_source(Box::new(MCPropertyEncyclopediaAdapter));

            registry
        }
    }

    #[cfg(feature = "build-data")]
    fn download_from_url(url: &str) -> Result<String> {
        let response = reqwest::blocking::get(url).context("Failed to make HTTP request")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        response
            .text()
            .context("Failed to read response body as text")
    }
    
    #[cfg(not(feature = "build-data"))]
    fn download_from_url(_url: &str) -> Result<String> {
        anyhow::bail!("Network downloads disabled - build-data feature not enabled")
    }
}

use data_sources_build::*;

/// Use pre-built data files instead of downloading
fn use_prebuilt_data(out_dir: &str) -> Result<()> {
    let data_dir = Path::new("data");
    
    // Check if pre-built data exists
    let prismarinejs_file = data_dir.join("prismarinejs_blocks.json");
    let mcproperty_file = data_dir.join("mcproperty_blocks.json");
    
    if !prismarinejs_file.exists() && !mcproperty_file.exists() {
        anyhow::bail!("No pre-built data files found in ./data/ directory. Run 'cargo run --bin build-data --features build-data' to generate them.");
    }
    
    // Use PrismarineJS data if available, otherwise MCProperty
    let data_file = if prismarinejs_file.exists() {
        println!("cargo:warning=Using pre-built PrismarineJS data");
        prismarinejs_file
    } else {
        println!("cargo:warning=Using pre-built MCPropertyEncyclopedia data");
        mcproperty_file
    };
    
    // Load and parse the pre-built data
    let json_data = fs::read_to_string(&data_file)
        .with_context(|| format!("Failed to read pre-built data from {:?}", data_file))?;
    
    let parsed: Value = serde_json::from_str(&json_data)
        .context("Failed to parse pre-built JSON data")?;
    
    // Generate PHF table using legacy method for now
    generate_legacy_phf_table(out_dir, &parsed)?;
    
    println!("cargo:warning=Successfully built blockpedia using pre-built data");
    Ok(())
}

const BLOCKS_DATA_URL: &str = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/pc/1.20.4/blocks.json";

// Simple fetcher framework for build script
#[derive(Debug, Clone)]
struct ExtraData {
    mock_data: HashMap<String, i32>,
    color_data: HashMap<String, (u8, u8, u8, f32, f32, f32)>, // RGB + Oklab
}

struct FetcherRegistry {
    extra_data: ExtraData,
}

impl FetcherRegistry {
    fn new() -> Self {
        Self {
            extra_data: ExtraData {
                mock_data: HashMap::new(),
                color_data: HashMap::new(),
            },
        }
    }

    fn add_color_data(&mut self, block_id: &str, rgb: (u8, u8, u8)) {
        // Use the same RGB to Oklab conversion as the generated code
        let r = rgb.0 as f32 / 255.0;
        let g = rgb.1 as f32 / 255.0;
        let b = rgb.2 as f32 / 255.0;
        let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let a = (r - g) * 0.5;
        let b_val = (r + g - 2.0 * b) * 0.25;

        self.extra_data
            .color_data
            .insert(block_id.to_string(), (rgb.0, rgb.1, rgb.2, l, a, b_val));
    }

    /// Extract colors from all available textures
    fn extract_colors_from_textures(&mut self, available_block_ids: &[String]) -> Result<()> {
        use std::path::Path;

        // Build texture mapping
        let textures_dir = Path::new("assets/textures");
        if !textures_dir.exists() {
            println!("cargo:warning=No textures directory found - using mock color data only");
            return Ok(());
        }

        println!("cargo:warning=Extracting colors from textures in {textures_dir:?}");

        // Scan texture files
        let texture_files: Vec<String> = std::fs::read_dir(textures_dir)
            .context("Failed to read textures directory")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "png" {
                    path.file_stem()?.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect();

        println!("cargo:warning=Found {} texture files", texture_files.len());

        let mut extracted_count = 0;
        let mut failed_count = 0;

        // Map textures to block IDs and extract colors
        for texture_name in texture_files {
            if let Some(block_ids) = self.texture_to_block_ids(&texture_name) {
                let texture_path = textures_dir.join(format!("{}.png", texture_name));

                match self.extract_color_from_texture(&texture_path) {
                    Ok(rgb) => {
                        for block_id in &block_ids {
                            // Only add color data for blocks that actually exist in our data
                            if available_block_ids.contains(block_id) {
                                self.add_color_data(block_id, rgb);
                                extracted_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        failed_count += 1;
                        if failed_count <= 5 {
                            // Only show first few errors
                            println!(
                                "cargo:warning=Failed to extract color from {}: {}",
                                texture_name, e
                            );
                        }
                    }
                }
            }
        }

        println!(
            "cargo:warning=Color extraction complete: {} colors extracted, {} failures",
            extracted_count, failed_count
        );
        Ok(())
    }

    /// Extract color from a single texture file
    fn extract_color_from_texture(&self, texture_path: &Path) -> Result<(u8, u8, u8)> {
        let img = image::open(texture_path)
            .with_context(|| format!("Failed to open texture: {:?}", texture_path))?;

        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        // Simple average color extraction
        let mut r_sum = 0u64;
        let mut g_sum = 0u64;
        let mut b_sum = 0u64;
        let mut pixel_count = 0u64;

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_img.get_pixel(x, y);
                let [r, g, b, a] = pixel.0;

                // Skip transparent pixels
                if a > 128 {
                    r_sum += r as u64;
                    g_sum += g as u64;
                    b_sum += b as u64;
                    pixel_count += 1;
                }
            }
        }

        if pixel_count == 0 {
            anyhow::bail!("No opaque pixels found in texture");
        }

        let avg_r = (r_sum / pixel_count) as u8;
        let avg_g = (g_sum / pixel_count) as u8;
        let avg_b = (b_sum / pixel_count) as u8;

        Ok((avg_r, avg_g, avg_b))
    }

    /// Add color inheritance for stairs, slabs, and walls
    fn add_inherited_colors(&mut self, available_block_ids: &[String]) {
        let mut inherited_count = 0;

        // Create a copy of existing color data for lookups
        let existing_colors = self.extra_data.color_data.clone();

        for block_id in available_block_ids {
            // Skip if this block already has color data
            if existing_colors.contains_key(block_id) {
                continue;
            }

            if let Some(base_material) = self.get_base_material_for_block(block_id) {
                if let Some(color) = existing_colors.get(&base_material) {
                    // Inherit the color from the base material
                    self.extra_data.color_data.insert(block_id.clone(), *color);
                    inherited_count += 1;
                }
            }
        }

        println!(
            "cargo:warning=Color inheritance complete: {} colors inherited from base materials",
            inherited_count
        );
    }

    /// Get the base material for stairs, slabs, walls, etc.
    fn get_base_material_for_block(&self, block_id: &str) -> Option<String> {
        let block_name = block_id.strip_prefix("minecraft:").unwrap_or(block_id);

        // Handle stairs
        if block_name.ends_with("_stairs") {
            let base = block_name.replace("_stairs", "");
            return Some(format!("minecraft:{}", base));
        }

        // Handle slabs
        if block_name.ends_with("_slab") {
            let base = block_name.replace("_slab", "");
            // Special cases for slabs
            match base.as_str() {
                "petrified_oak" => return Some("minecraft:oak_planks".to_string()),
                "smooth_stone" => return Some("minecraft:stone".to_string()),
                "cut_copper" => return Some("minecraft:copper_block".to_string()),
                "exposed_cut_copper" => return Some("minecraft:exposed_copper".to_string()),
                "weathered_cut_copper" => return Some("minecraft:weathered_copper".to_string()),
                "oxidized_cut_copper" => return Some("minecraft:oxidized_copper".to_string()),
                "waxed_cut_copper" => return Some("minecraft:copper_block".to_string()),
                "waxed_exposed_cut_copper" => return Some("minecraft:exposed_copper".to_string()),
                "waxed_weathered_cut_copper" => {
                    return Some("minecraft:weathered_copper".to_string())
                }
                "waxed_oxidized_cut_copper" => {
                    return Some("minecraft:oxidized_copper".to_string())
                }
                "cut_red_sandstone" => return Some("minecraft:red_sandstone".to_string()),
                "cut_sandstone" => return Some("minecraft:sandstone".to_string()),
                "prismarine_brick" => return Some("minecraft:prismarine_bricks".to_string()),
                "nether_brick" => return Some("minecraft:nether_bricks".to_string()),
                "red_nether_brick" => return Some("minecraft:red_nether_bricks".to_string()),
                "polished_blackstone_brick" => {
                    return Some("minecraft:polished_blackstone_bricks".to_string())
                }
                "end_stone_brick" => return Some("minecraft:end_stone_bricks".to_string()),
                "stone_brick" => return Some("minecraft:stone_bricks".to_string()),
                "mossy_stone_brick" => return Some("minecraft:mossy_stone_bricks".to_string()),
                "mossy_cobblestone" => return Some("minecraft:mossy_cobblestone".to_string()),
                "deepslate_brick" => return Some("minecraft:deepslate_bricks".to_string()),
                "deepslate_tile" => return Some("minecraft:deepslate_tiles".to_string()),
                "polished_deepslate" => return Some("minecraft:polished_deepslate".to_string()),
                "cobbled_deepslate" => return Some("minecraft:cobbled_deepslate".to_string()),
                "tuff_brick" => return Some("minecraft:tuff_bricks".to_string()),
                "polished_tuff" => return Some("minecraft:polished_tuff".to_string()),
                "bamboo_mosaic" => return Some("minecraft:bamboo_planks".to_string()),
                _ => {
                    // Default case: try the base material directly
                    return Some(format!("minecraft:{}", base));
                }
            }
        }

        // Handle walls
        if block_name.ends_with("_wall") {
            let base = block_name.replace("_wall", "");
            // Special cases for walls
            match base.as_str() {
                "cobblestone" => return Some("minecraft:cobblestone".to_string()),
                "mossy_cobblestone" => return Some("minecraft:mossy_cobblestone".to_string()),
                "stone_brick" => return Some("minecraft:stone_bricks".to_string()),
                "mossy_stone_brick" => return Some("minecraft:mossy_stone_bricks".to_string()),
                "granite" => return Some("minecraft:granite".to_string()),
                "diorite" => return Some("minecraft:diorite".to_string()),
                "andesite" => return Some("minecraft:andesite".to_string()),
                "cobbled_deepslate" => return Some("minecraft:cobbled_deepslate".to_string()),
                "polished_deepslate" => return Some("minecraft:polished_deepslate".to_string()),
                "deepslate_brick" => return Some("minecraft:deepslate_bricks".to_string()),
                "deepslate_tile" => return Some("minecraft:deepslate_tiles".to_string()),
                "brick" => return Some("minecraft:bricks".to_string()),
                "mud_brick" => return Some("minecraft:mud_bricks".to_string()),
                "nether_brick" => return Some("minecraft:nether_bricks".to_string()),
                "red_nether_brick" => return Some("minecraft:red_nether_bricks".to_string()),
                "sandstone" => return Some("minecraft:sandstone".to_string()),
                "red_sandstone" => return Some("minecraft:red_sandstone".to_string()),
                "blackstone" => return Some("minecraft:blackstone".to_string()),
                "polished_blackstone" => return Some("minecraft:polished_blackstone".to_string()),
                "polished_blackstone_brick" => {
                    return Some("minecraft:polished_blackstone_bricks".to_string())
                }
                "end_stone_brick" => return Some("minecraft:end_stone_bricks".to_string()),
                "prismarine" => return Some("minecraft:prismarine".to_string()),
                "tuff" => return Some("minecraft:tuff".to_string()),
                "polished_tuff" => return Some("minecraft:polished_tuff".to_string()),
                "tuff_brick" => return Some("minecraft:tuff_bricks".to_string()),
                _ => {
                    // Default case: try the base material directly
                    return Some(format!("minecraft:{}", base));
                }
            }
        }

        // Handle fences
        if block_name.ends_with("_fence") && !block_name.ends_with("_fence_gate") {
            let base = block_name.replace("_fence", "");
            match base.as_str() {
                "nether_brick" => return Some("minecraft:nether_bricks".to_string()),
                _ => {
                    // For wood fences, use the planks
                    return Some(format!("minecraft:{}_planks", base));
                }
            }
        }

        // Handle fence gates
        if block_name.ends_with("_fence_gate") {
            let base = block_name.replace("_fence_gate", "");
            return Some(format!("minecraft:{}_planks", base));
        }

        // Handle doors
        if block_name.ends_with("_door") {
            let base = block_name.replace("_door", "");
            match base.as_str() {
                "iron" => return Some("minecraft:iron_block".to_string()),
                "copper" => return Some("minecraft:copper_block".to_string()),
                "exposed_copper" => return Some("minecraft:exposed_copper".to_string()),
                "weathered_copper" => return Some("minecraft:weathered_copper".to_string()),
                "oxidized_copper" => return Some("minecraft:oxidized_copper".to_string()),
                "waxed_copper" => return Some("minecraft:copper_block".to_string()),
                "waxed_exposed_copper" => return Some("minecraft:exposed_copper".to_string()),
                "waxed_weathered_copper" => return Some("minecraft:weathered_copper".to_string()),
                "waxed_oxidized_copper" => return Some("minecraft:oxidized_copper".to_string()),
                _ => {
                    // For wood doors, use the planks
                    return Some(format!("minecraft:{}_planks", base));
                }
            }
        }

        // Handle trapdoors
        if block_name.ends_with("_trapdoor") {
            let base = block_name.replace("_trapdoor", "");
            match base.as_str() {
                "iron" => return Some("minecraft:iron_block".to_string()),
                "copper" => return Some("minecraft:copper_block".to_string()),
                "exposed_copper" => return Some("minecraft:exposed_copper".to_string()),
                "weathered_copper" => return Some("minecraft:weathered_copper".to_string()),
                "oxidized_copper" => return Some("minecraft:oxidized_copper".to_string()),
                "waxed_copper" => return Some("minecraft:copper_block".to_string()),
                "waxed_exposed_copper" => return Some("minecraft:exposed_copper".to_string()),
                "waxed_weathered_copper" => return Some("minecraft:weathered_copper".to_string()),
                "waxed_oxidized_copper" => return Some("minecraft:oxidized_copper".to_string()),
                _ => {
                    // For wood trapdoors, use the planks
                    return Some(format!("minecraft:{}_planks", base));
                }
            }
        }

        None
    }

    /// Convert texture file name to potential block IDs using smart heuristics
    fn texture_to_block_ids(&self, texture_name: &str) -> Option<Vec<String>> {
        let mut block_ids = Vec::new();

        // Basic mapping: texture name to block ID
        let base_id = format!("minecraft:{}", texture_name);
        block_ids.push(base_id);

        // Handle special cases and variations
        match texture_name {
            // Log textures map to both log and wood blocks
            name if name.ends_with("_log") => {
                let wood_base = name.replace("_log", "");
                block_ids.push(format!("minecraft:{}_wood", wood_base));
                // Add leaves for the same wood type
                block_ids.push(format!("minecraft:{}_leaves", wood_base));
                // Handle stripped variants
                if name.starts_with("stripped_") {
                    let base = name.replace("stripped_", "");
                    block_ids.push(format!("minecraft:{}", base));
                }
            }

            // Log top textures
            name if name.ends_with("_log_top") => {
                let base = name.replace("_log_top", "");
                block_ids.push(format!("minecraft:{}_log", base));
                block_ids.push(format!("minecraft:{}_wood", base));
            }

            // Stone variants
            "stone" => {
                block_ids.push("minecraft:smooth_stone".to_string());
            }

            // Grass block variants
            "grass_block_snow" => {
                block_ids.push("minecraft:grass_block".to_string());
            }

            // Sandstone variants
            "sandstone_top" => {
                block_ids.push("minecraft:sandstone".to_string());
                block_ids.push("minecraft:smooth_sandstone".to_string());
            }
            "red_sandstone_top" => {
                block_ids.push("minecraft:red_sandstone".to_string());
                block_ids.push("minecraft:smooth_red_sandstone".to_string());
            }

            // Multi-face blocks - use the most representative texture
            "furnace_side" => {
                block_ids.push("minecraft:furnace".to_string());
            }
            "furnace_front" | "furnace_top" => {
                // Skip these in favor of furnace_side
                return None;
            }

            // Pumpkin variants
            "pumpkin_side" => {
                block_ids.push("minecraft:pumpkin".to_string());
            }
            "pumpkin_top" => {
                // Skip in favor of pumpkin_side
                return None;
            }

            // Melon
            "melon_side" => {
                block_ids.push("minecraft:melon".to_string());
            }
            "melon_top" => {
                // Skip in favor of melon_side
                return None;
            }

            // Hay block
            "hay_block_side" => {
                block_ids.push("minecraft:hay_block".to_string());
            }
            "hay_block_top" => {
                // Skip in favor of hay_block_side
                return None;
            }

            // TNT
            "tnt_side" => {
                block_ids.push("minecraft:tnt".to_string());
            }
            "tnt_bottom" => {
                // Skip in favor of tnt_side
                return None;
            }

            // Mycelium
            "mycelium_side" => {
                block_ids.push("minecraft:mycelium".to_string());
            }
            "mycelium_top" => {
                // Skip in favor of mycelium_side
                return None;
            }

            // Podzol
            "podzol_side" => {
                block_ids.push("minecraft:podzol".to_string());
            }
            "podzol_top" => {
                // Skip in favor of podzol_side
                return None;
            }

            // Farmland
            "farmland_moist" => {
                block_ids.push("minecraft:farmland".to_string());
            }
            "farmland" => {
                // Skip in favor of farmland_moist
                return None;
            }

            // Grass path
            "dirt_path_top" => {
                block_ids.push("minecraft:dirt_path".to_string());
            }

            // Copper variants
            name if name.contains("copper") && name.contains("_bulb") => {
                // Handle copper bulb variants
                let base = name.replace("_lit", "");
                block_ids.push(format!("minecraft:{}", base));
            }

            // Shulker box default
            "shulker_box" => {
                // This maps to the purple shulker box by default
                block_ids.clear();
                block_ids.push("minecraft:shulker_box".to_string());
            }

            // Default case - the base mapping is usually correct
            _ => {
                // Keep the base mapping
            }
        }

        if block_ids.is_empty() {
            None
        } else {
            Some(block_ids)
        }
    }

    fn fetch_all(&mut self, available_block_ids: &[String]) -> Result<&ExtraData> {
        // Mock fetcher data
        self.extra_data
            .mock_data
            .insert("minecraft:stone".to_string(), 42);
        self.extra_data
            .mock_data
            .insert("minecraft:dirt".to_string(), 123);
        self.extra_data
            .mock_data
            .insert("minecraft:grass_block".to_string(), 456);
        self.extra_data
            .mock_data
            .insert("minecraft:oak_log".to_string(), 789);
        self.extra_data
            .mock_data
            .insert("minecraft:oak_planks".to_string(), 321);
        self.extra_data
            .mock_data
            .insert("minecraft:cobblestone".to_string(), 654);

        // First add hardcoded color data for reference
        self.add_color_data("minecraft:stone", (125, 125, 125));
        self.add_color_data("minecraft:dirt", (134, 96, 67));
        self.add_color_data("minecraft:grass_block", (95, 159, 53));
        self.add_color_data("minecraft:oak_log", (102, 81, 51));
        self.add_color_data("minecraft:oak_leaves", (65, 137, 50));
        self.add_color_data("minecraft:oak_planks", (162, 130, 78));
        self.add_color_data("minecraft:water", (64, 164, 223));
        self.add_color_data("minecraft:lava", (207, 108, 32));
        self.add_color_data("minecraft:cobblestone", (127, 127, 127));
        self.add_color_data("minecraft:sand", (219, 203, 158));
        self.add_color_data("minecraft:gravel", (136, 126, 126));
        self.add_color_data("minecraft:gold_ore", (252, 238, 75));
        self.add_color_data("minecraft:iron_ore", (135, 130, 126));
        self.add_color_data("minecraft:diamond_ore", (92, 219, 213));

        // Extract colors from all available textures
        if let Err(e) = self.extract_colors_from_textures(available_block_ids) {
            println!(
                "cargo:warning=Failed to extract colors from textures: {}",
                e
            );
        }

        // Add color inheritance for stairs, slabs, and walls
        self.add_inherited_colors(available_block_ids);

        Ok(&self.extra_data)
    }

    fn generate_query_helpers(&self, file: &mut std::fs::File) -> Result<()> {
        // Generate color query helpers
        writeln!(file, "// Generated query helper functions")?;
        writeln!(file, "impl crate::BlockFacts {{")?;

        // Closest color query
        writeln!(
            file,
            "    pub fn closest_to_color(target_rgb: [u8; 3]) -> Option<&'static Self> {{"
        )?;
        writeln!(file, "        let target_oklab = rgb_to_oklab(target_rgb);")?;
        writeln!(file, "        let mut best_block = None;")?;
        writeln!(file, "        let mut best_distance = f32::INFINITY;")?;
        writeln!(file, "        for block in crate::all_blocks() {{")?;
        writeln!(
            file,
            "            if let Some(ref color) = block.extras.color {{"
        )?;
        writeln!(
            file,
            "                let distance = oklab_distance(target_oklab, color.oklab);"
        )?;
        writeln!(file, "                if distance < best_distance {{")?;
        writeln!(file, "                    best_distance = distance;")?;
        writeln!(file, "                    best_block = Some(block);")?;
        writeln!(file, "                }}")?;
        writeln!(file, "            }}")?;
        writeln!(file, "        }}")?;
        writeln!(file, "        best_block")?;
        writeln!(file, "    }}")?;
        writeln!(file)?;

        // Color range query
        writeln!(file, "    pub fn blocks_in_color_range(center_rgb: [u8; 3], max_distance: f32) -> Vec<&'static Self> {{")? ;
        writeln!(file, "        let center_oklab = rgb_to_oklab(center_rgb);")?;
        writeln!(file, "        let mut result = Vec::new();")?;
        writeln!(file, "        for block in crate::all_blocks() {{")?;
        writeln!(
            file,
            "            if let Some(ref color) = block.extras.color {{"
        )?;
        writeln!(
            file,
            "                let distance = oklab_distance(center_oklab, color.oklab);"
        )?;
        writeln!(file, "                if distance <= max_distance {{")?;
        writeln!(file, "                    result.push(block);")?;
        writeln!(file, "                }}")?;
        writeln!(file, "            }}")?;
        writeln!(file, "        }}")?;
        writeln!(file, "        result")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Helper functions
        writeln!(file, "fn rgb_to_oklab(rgb: [u8; 3]) -> [f32; 3] {{")?;
        writeln!(
            file,
            "    // Simplified RGB to Oklab conversion for build-time"
        )?;
        writeln!(file, "    let r = rgb[0] as f32 / 255.0;")?;
        writeln!(file, "    let g = rgb[1] as f32 / 255.0;")?;
        writeln!(file, "    let b = rgb[2] as f32 / 255.0;")?;
        writeln!(file, "    let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;")?;
        writeln!(file, "    let a = (r - g) * 0.5;")?;
        writeln!(file, "    let b_val = (r + g - 2.0 * b) * 0.25;")?;
        writeln!(file, "    [l, a, b_val]")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        writeln!(
            file,
            "fn oklab_distance(a: [f32; 3], b: [f32; 3]) -> f32 {{"
        )?;
        writeln!(file, "    let dl = a[0] - b[0];")?;
        writeln!(file, "    let da = a[1] - b[1];")?;
        writeln!(file, "    let db = a[2] - b[2];")?;
        writeln!(file, "    (dl * dl + da * da + db * db).sqrt()")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        Ok(())
    }
}

fn setup_fetchers() -> FetcherRegistry {
    FetcherRegistry::new()
}

/// Extract block IDs from JSON in either format
fn get_block_ids_from_json(json: &Value) -> Result<Vec<String>> {
    let mut block_ids = Vec::new();

    if json.is_object() && json.get("blocks").is_some() {
        // Test format: {"blocks": {"minecraft:stone": {...}, ...}}
        let blocks_obj = json["blocks"]
            .as_object()
            .context("'blocks' field is not an object")?;
        block_ids.extend(blocks_obj.keys().cloned());
    } else if json.is_array() {
        // PrismarineJS format: [{"name": "air", ...}, ...]
        let blocks_array = json.as_array().context("JSON is not a valid array")?;

        for block in blocks_array {
            if let Some(block_obj) = block.as_object() {
                if let Some(name) = block_obj.get("name").and_then(|n| n.as_str()) {
                    block_ids.push(format!("minecraft:{}", name));
                }
            }
        }
    } else {
        anyhow::bail!("Unsupported JSON format for extracting block IDs");
    }

    Ok(block_ids)
}

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Check if we should use pre-built data
    if cfg!(feature = "use-prebuilt") || env::var("BLOCKPEDIA_USE_PREBUILT").is_ok() {
        println!("cargo:warning=Using pre-built data files");
        return use_prebuilt_data(&out_dir);
    }

    // Check if we can download data (build-data feature available)
    #[cfg(not(feature = "build-data"))]
    {
        println!("cargo:warning=Network downloads disabled (build-data feature not enabled)");
        println!("cargo:warning=Checking for pre-built data as fallback...");
        if let Ok(()) = use_prebuilt_data(&out_dir) {
            return Ok(());
        } else {
            anyhow::bail!("No pre-built data available and network downloads disabled. Run 'cargo run --bin build-data --features build-data' to generate data files.");
        }
    }

    // Set up data source registry
    let mut data_registry = DataSourceRegistry::default();

    // Check for environment variable to set data source
    if let Ok(source_name) = env::var("BLOCKPEDIA_DATA_SOURCE") {
        println!(
            "cargo:warning=Setting data source to {} from environment variable",
            source_name
        );
        data_registry
            .set_primary_source(&source_name)
            .with_context(|| format!("Failed to set data source to {}", source_name))?;
    }

    println!(
        "cargo:warning=Available data sources: {:?}",
        data_registry.list_sources()
    );
    println!(
        "cargo:warning=Using primary data source: {}",
        data_registry.get_primary_source()?.name()
    );

    let cache_path = Path::new(&out_dir).join("blocks_data.json");

    // Fetch unified data from the selected data source
    let unified_blocks = if env::var("BLOCKPEDIA_USE_TEST_DATA").is_ok() {
        // Use legacy fetch method for test data
        let json_data = fetch_or_load_cached(&cache_path)?;
        let parsed: Value =
            serde_json::from_str(&json_data).context("Failed to parse downloaded JSON")?;
        validate_json_structure(&parsed)?;

        // Convert legacy format to unified format (will be replaced later)
        vec![] // Placeholder for now, will use legacy generation
    } else {
        // Use new unified data source system
        match data_registry.fetch_unified_data() {
            Ok(blocks) => blocks,
            Err(e) => {
                println!(
                    "cargo:warning=Failed to fetch from primary source ({}): {}",
                    data_registry.get_primary_source()?.name(),
                    e
                );
                println!("cargo:warning=Falling back to cached/legacy method");

                // Fallback to legacy method
                let json_data = fetch_or_load_cached(&cache_path)?;
                let parsed: Value =
                    serde_json::from_str(&json_data).context("Failed to parse downloaded JSON")?;
                validate_json_structure(&parsed)?;

                // Generate using legacy method
                generate_legacy_phf_table(&out_dir, &parsed)?;
                return Ok(());
            }
        }
    };

    // For now, if we have unified blocks, generate using legacy method
    // This will be replaced with unified generation later
    if unified_blocks.is_empty() || env::var("BLOCKPEDIA_USE_TEST_DATA").is_ok() {
        let json_data = fetch_or_load_cached(&cache_path)?;
        let parsed: Value =
            serde_json::from_str(&json_data).context("Failed to parse downloaded JSON")?;
        validate_json_structure(&parsed)?;
        generate_legacy_phf_table(&out_dir, &parsed)?;
    } else {
        // Generate from unified data
        generate_unified_phf_table(&out_dir, &unified_blocks)?;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=BLOCKPEDIA_DATA_SOURCE");
    println!("cargo:rerun-if-env-changed=BLOCKPEDIA_USE_TEST_DATA");
    println!("cargo:rerun-if-env-changed=BLOCKPEDIA_VERSION_JSON_SHA");

    Ok(())
}

fn fetch_or_load_cached(cache_path: &Path) -> Result<String> {
    // Check if we should use test data (for development)
    if std::env::var("BLOCKPEDIA_USE_TEST_DATA").is_ok() {
        let test_file = Path::new("test_blocks_data.json");
        if test_file.exists() {
            println!("cargo:warning=Using local test file (BLOCKPEDIA_USE_TEST_DATA is set)");
            return fs::read_to_string(test_file).context("Failed to read test JSON file");
        }
    }

    // First try to load from cache
    if cache_path.exists() {
        println!("cargo:warning=Using cached blocks_data.json");
        return fs::read_to_string(cache_path).context("Failed to read cached JSON file");
    }

    // Try to download
    println!("cargo:warning=Downloading blocks_data.json from GitHub...");
    match download_json() {
        Ok(data) => {
            // Cache the downloaded data
            fs::write(cache_path, &data).context("Failed to cache downloaded JSON")?;
            Ok(data)
        }
        Err(e) => {
            anyhow::bail!(
                "Failed to download blocks_data.json and no cache available: {}",
                e
            );
        }
    }
}

#[cfg(feature = "build-data")]
fn download_json() -> Result<String> {
    let response =
        reqwest::blocking::get(BLOCKS_DATA_URL).context("Failed to make HTTP request")?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    response
        .text()
        .context("Failed to read response body as text")
}

#[cfg(not(feature = "build-data"))]
fn download_json() -> Result<String> {
    anyhow::bail!("Network downloads disabled - build-data feature not enabled")
}

fn validate_json_structure(json: &Value) -> Result<()> {
    // Handle two different formats: our test format and PrismarineJS format
    if json.is_object() && json.get("blocks").is_some() {
        // Our test format: {"blocks": {...}}
        let blocks = json["blocks"]
            .as_object()
            .context("'blocks' field is not an object")?;

        if blocks.is_empty() {
            anyhow::bail!("No blocks found in JSON data");
        }

        println!(
            "cargo:warning=JSON validation passed - found {} blocks (test format)",
            blocks.len()
        );
    } else if json.is_array() {
        // PrismarineJS format: [{"name": "air", ...}, ...]
        let blocks_array = json.as_array().context("JSON is not a valid array")?;

        if blocks_array.is_empty() {
            anyhow::bail!("No blocks found in JSON data");
        }

        // Validate a few sample blocks have required fields
        for (i, block_data) in blocks_array.iter().take(5).enumerate() {
            let block_obj = block_data
                .as_object()
                .with_context(|| format!("Block at index {} is not an object", i))?;

            if !block_obj.contains_key("name") {
                anyhow::bail!("Block at index {} missing 'name' field", i);
            }
        }

        println!(
            "cargo:warning=JSON validation passed - found {} blocks (PrismarineJS format)",
            blocks_array.len()
        );
    } else {
        anyhow::bail!(
            "JSON format not recognized - expected either {{\"blocks\": {{...}}}} or array format"
        );
    }

    Ok(())
}

fn generate_phf_table(
    out_dir: &str,
    json: &Value,
    extra_data: &ExtraData,
    fetcher_registry: &FetcherRegistry,
) -> Result<()> {
    let table_path = Path::new(out_dir).join("block_table.rs");
    let mut file = std::fs::File::create(&table_path).context("Failed to create block_table.rs")?;

    // Start building the PHF map
    writeln!(file, "// Auto-generated PHF table from block data")?;
    writeln!(file, "use phf::{{phf_map, Map}};")?;
    writeln!(file)?;

    // Determine format and convert to unified representation
    let block_data: Vec<(String, serde_json::Value)> = if json.is_object()
        && json.get("blocks").is_some()
    {
        // Test format: {"blocks": {"minecraft:stone": {...}, ...}}
        let blocks_obj = json["blocks"]
            .as_object()
            .context("'blocks' field is not an object")?;
        blocks_obj
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    } else if json.is_array() {
        // PrismarineJS format: [{"name": "air", ...}, ...]
        let blocks_array = json.as_array().context("JSON is not a valid array")?;

        blocks_array
            .iter()
            .filter_map(|block| {
                let block_obj = block.as_object()?;
                let name = block_obj.get("name")?.as_str()?;

                // Convert PrismarineJS format to our expected format
                let mut converted_block = serde_json::Map::new();

                // Convert "states" to "properties"
                if let Some(states) = block_obj.get("states").and_then(|s| s.as_array()) {
                    let mut properties = serde_json::Map::new();

                    for state in states {
                        if let Some(state_obj) = state.as_object() {
                            if let (Some(prop_name), Some(prop_type), Some(num_values)) = (
                                state_obj.get("name").and_then(|n| n.as_str()),
                                state_obj.get("type").and_then(|t| t.as_str()),
                                state_obj.get("num_values").and_then(|n| n.as_u64()),
                            ) {
                                let values = match prop_type {
                                    "bool" => vec!["false".to_string(), "true".to_string()],
                                    "int" => {
                                        // For int types, check if values array is available
                                        if let Some(values_array) =
                                            state_obj.get("values").and_then(|v| v.as_array())
                                        {
                                            values_array
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect()
                                        } else {
                                            (0..num_values).map(|i| i.to_string()).collect()
                                        }
                                    }
                                    "enum" => {
                                        // Extract actual enum values if available
                                        if let Some(values_array) =
                                            state_obj.get("values").and_then(|v| v.as_array())
                                        {
                                            values_array
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect()
                                        } else {
                                            (0..num_values)
                                                .map(|i| format!("value_{}", i))
                                                .collect()
                                        }
                                    }
                                    _ => vec!["unknown".to_string()],
                                };
                                properties.insert(
                                    prop_name.to_string(),
                                    serde_json::Value::Array(
                                        values.into_iter().map(serde_json::Value::String).collect(),
                                    ),
                                );
                            }
                        }
                    }

                    if !properties.is_empty() {
                        converted_block.insert(
                            "properties".to_string(),
                            serde_json::Value::Object(properties),
                        );
                    } else {
                        converted_block.insert(
                            "properties".to_string(),
                            serde_json::Value::Object(serde_json::Map::new()),
                        );
                    }
                } else {
                    converted_block.insert(
                        "properties".to_string(),
                        serde_json::Value::Object(serde_json::Map::new()),
                    );
                }

                // Empty default state for now
                converted_block.insert(
                    "default_state".to_string(),
                    serde_json::Value::Object(serde_json::Map::new()),
                );

                Some((
                    format!("minecraft:{}", name),
                    serde_json::Value::Object(converted_block),
                ))
            })
            .collect()
    } else {
        anyhow::bail!("Unsupported JSON format");
    };

    // Generate the static block data
    for (block_id, block_data) in &block_data {
        let block_obj = block_data
            .as_object()
            .with_context(|| format!("Block '{}' is not an object", block_id))?;

        // Parse properties
        let empty_props = serde_json::Map::new();
        let properties = block_obj
            .get("properties")
            .and_then(|p| p.as_object())
            .unwrap_or(&empty_props);

        // Parse default state
        let empty_state = serde_json::Map::new();
        let default_state = block_obj
            .get("default_state")
            .and_then(|d| d.as_object())
            .unwrap_or(&empty_state);

        // Generate a valid Rust identifier from block ID
        let safe_name = block_id.replace(":", "_").replace("-", "_").to_uppercase();

        writeln!(
            file,
            "static {}: crate::BlockFacts = crate::BlockFacts {{",
            safe_name
        )?;
        writeln!(file, "    id: \"{}\",", block_id)?;

        // Generate properties array
        writeln!(file, "    properties: &[")?;
        for (prop_name, prop_values) in properties {
            if let Some(values_array) = prop_values.as_array() {
                write!(file, "        (\"{}\", &[", prop_name)?;
                for (i, value) in values_array.iter().enumerate() {
                    if i > 0 {
                        write!(file, ", ")?;
                    }
                    write!(file, "\"{}\"", value.as_str().unwrap_or(""))?;
                }
                writeln!(file, "]),")?;
            }
        }
        writeln!(file, "    ],")?;

        // Generate default_state array
        writeln!(file, "    default_state: &[")?;
        for (state_name, state_value) in default_state {
            writeln!(
                file,
                "        (\"{}\", \"{}\"),",
                state_name,
                state_value.as_str().unwrap_or("")
            )?;
        }
        writeln!(file, "    ],")?;

        // Generate extras with fetched data
        write!(file, "    extras: crate::Extras {{")?;

        // Mock data
        if let Some(mock_val) = extra_data.mock_data.get(block_id) {
            write!(file, " mock_data: Some({}),", mock_val)?;
        } else {
            write!(file, " mock_data: None,")?;
        }

        // Color data
        if let Some((r, g, b, l, a, b_val)) = extra_data.color_data.get(block_id) {
            // Adjust values to avoid clippy::approx_constant warnings
            let adjusted_l = if (*l - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *l + 0.001
            } else {
                *l
            };
            let adjusted_a = if (*a - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *a + 0.001
            } else {
                *a
            };
            let adjusted_b = if (*b_val - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *b_val + 0.001
            } else {
                *b_val
            };
            write!(file, " color: Some(crate::ColorData {{ rgb: [{}, {}, {}], oklab: [{:.3}, {:.3}, {:.3}] }}),", r, g, b, adjusted_l, adjusted_a, adjusted_b)?;
        } else {
            write!(file, " color: None,")?;
        }

        writeln!(file, " }},")?;
        writeln!(file, "}};")?;
        writeln!(file)?;
    }

    // Generate the PHF map
    writeln!(
        file,
        "pub static BLOCKS: Map<&'static str, &'static crate::BlockFacts> = phf_map! {{"
    )?;

    for (block_id, _) in &block_data {
        let safe_name = block_id.replace(":", "_").replace("-", "_").to_uppercase();
        writeln!(file, "    \"{}\" => &{},", block_id, safe_name)?;
    }

    writeln!(file, "}};")?;
    writeln!(file)?;

    // Generate query helpers from fetchers
    fetcher_registry.generate_query_helpers(&mut file)?;

    println!(
        "cargo:warning=Generated PHF table with {} blocks",
        block_data.len()
    );
    Ok(())
}

// Legacy PHF table generation for backward compatibility
fn generate_legacy_phf_table(out_dir: &str, json: &Value) -> Result<()> {
    // Set up fetcher registry
    let mut fetcher_registry = setup_fetchers();

    // Get list of available block IDs from JSON
    let available_block_ids = get_block_ids_from_json(json)?;

    // Fetch extra data from all registered fetchers
    let extra_data = fetcher_registry.fetch_all(&available_block_ids)?.clone();

    // Generate full PHF table from JSON data with extra data
    generate_phf_table(out_dir, json, &extra_data, &fetcher_registry)
}

// Generate PHF table from unified block data
fn generate_unified_phf_table(out_dir: &str, unified_blocks: &[UnifiedBlockData]) -> Result<()> {
    let table_path = Path::new(out_dir).join("block_table.rs");
    let mut file = std::fs::File::create(&table_path).context("Failed to create block_table.rs")?;

    // Set up fetcher registry for color data
    let mut fetcher_registry = setup_fetchers();
    let available_block_ids: Vec<String> = unified_blocks.iter().map(|b| b.id.clone()).collect();
    let extra_data = fetcher_registry.fetch_all(&available_block_ids)?.clone();

    // Start building the PHF map
    writeln!(file, "// Auto-generated PHF table from unified block data")?;
    writeln!(file, "use phf::{{phf_map, Map}};")?;
    writeln!(file)?;

    // Generate the static block data
    for block_data in unified_blocks {
        let block_id = &block_data.id;

        // Generate a valid Rust identifier from block ID
        let safe_name = block_id
            .replace(":", "_")
            .replace("-", "_")
            .replace("'", "")
            .replace("!", "")
            .replace(".", "_")
            .to_uppercase();

        writeln!(
            file,
            "static {}: crate::BlockFacts = crate::BlockFacts {{",
            safe_name
        )?;
        writeln!(file, "    id: \"{}\",", block_id)?;

        // Generate properties array
        writeln!(file, "    properties: &[")?;
        for (prop_name, prop_values) in &block_data.properties {
            write!(file, "        (\"{}\", &[", prop_name)?;
            for (i, value) in prop_values.iter().enumerate() {
                if i > 0 {
                    write!(file, ", ")?;
                }
                write!(file, "\"{}\"", value)?;
            }
            writeln!(file, "]),")?;
        }
        writeln!(file, "    ],")?;

        // Generate default_state array
        writeln!(file, "    default_state: &[")?;
        for (state_name, state_value) in &block_data.default_state {
            writeln!(file, "        (\"{}\", \"{}\"),", state_name, state_value)?;
        }
        writeln!(file, "    ],")?;

        // Generate extras with fetched data
        write!(file, "    extras: crate::Extras {{")?;

        // Mock data
        if let Some(mock_val) = extra_data.mock_data.get(block_id) {
            write!(file, " mock_data: Some({}),", mock_val)?;
        } else {
            write!(file, " mock_data: None,")?;
        }

        // Color data
        if let Some((r, g, b, l, a, b_val)) = extra_data.color_data.get(block_id) {
            // Adjust values to avoid clippy::approx_constant warnings
            let adjusted_l = if (*l - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *l + 0.001
            } else {
                *l
            };
            let adjusted_a = if (*a - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *a + 0.001
            } else {
                *a
            };
            let adjusted_b = if (*b_val - std::f32::consts::FRAC_1_PI).abs() < 0.001 {
                *b_val + 0.001
            } else {
                *b_val
            };
            write!(file, " color: Some(crate::ColorData {{ rgb: [{}, {}, {}], oklab: [{:.3}, {:.3}, {:.3}] }}),", r, g, b, adjusted_l, adjusted_a, adjusted_b)?;
        } else {
            write!(file, " color: None,")?;
        }

        writeln!(file, " }},")?;
        writeln!(file, "}};")?;
        writeln!(file)?;
    }

    // Generate the PHF map
    writeln!(
        file,
        "pub static BLOCKS: Map<&'static str, &'static crate::BlockFacts> = phf_map! {{"
    )?;

    for block_data in unified_blocks {
        let block_id = &block_data.id;
        let safe_name = block_id
            .replace(":", "_")
            .replace("-", "_")
            .replace("'", "")
            .replace("!", "")
            .replace(".", "_")
            .to_uppercase();
        writeln!(file, "    \"{}\" => &{},", block_id, safe_name)?;
    }

    writeln!(file, "}};")?;
    writeln!(file)?;

    // Generate query helpers from fetchers
    fetcher_registry.generate_query_helpers(&mut file)?;

    println!(
        "cargo:warning=Generated unified PHF table with {} blocks",
        unified_blocks.len()
    );
    Ok(())
}
