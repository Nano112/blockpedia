use std::collections::HashMap;

/// Maps Minecraft block IDs to texture file names
#[derive(Debug, Clone)]
pub struct TextureMapping {
    mappings: HashMap<String, String>,
}

impl TextureMapping {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Get texture file for a block ID
    pub fn get_texture(&self, block_id: &str) -> Option<&String> {
        self.mappings.get(block_id)
    }

    /// Get all mappings
    pub fn all_mappings(&self) -> &HashMap<String, String> {
        &self.mappings
    }

    /// Get statistics about texture coverage
    pub fn get_coverage_stats(&self, total_blocks: usize) -> TextureCoverageStats {
        TextureCoverageStats {
            total_blocks,
            blocks_with_textures: self.mappings.len(),
            unique_textures: self.mappings.values().collect::<std::collections::HashSet<_>>().len(),
            coverage_percentage: (self.mappings.len() as f32 / total_blocks as f32) * 100.0,
        }
    }
}

impl Default for TextureMapping {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TextureCoverageStats {
    pub total_blocks: usize,
    pub blocks_with_textures: usize,
    pub unique_textures: usize,
    pub coverage_percentage: f32,
}

impl std::fmt::Display for TextureCoverageStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Texture Coverage: {}/{} blocks ({:.1}%) using {} unique textures",
            self.blocks_with_textures,
            self.total_blocks,
            self.coverage_percentage,
            self.unique_textures
        )
    }
}
