# 🧱 Block Palette Examples & Use Cases

This document provides comprehensive examples and practical applications for Blockpedia's block palette generation features.

## 📋 Table of Contents

1. [Natural Biome Palettes](#-natural-biome-palettes)
2. [Architectural Style Palettes](#️-architectural-style-palettes)
3. [Block Gradient Palettes](#-block-gradient-palettes)
4. [Monochrome Palettes](#-monochrome-palettes)
5. [Complementary Palettes](#-complementary-palettes)
6. [Color Range Search](#-color-range-search)
7. [Export Formats](#-export-formats)
8. [Advanced Use Cases](#-advanced-use-cases)
9. [Tips & Best Practices](#-tips--best-practices)

---

## 🌿 Natural Biome Palettes

Natural biome palettes are curated collections of blocks that match specific Minecraft environments and natural themes.

### Forest Palette Examples

```rust
// Generate a forest-themed palette
if let Some(forest_palette) = BlockPaletteGenerator::generate_natural_palette("forest") {
    println!("Forest palette contains {} blocks", forest_palette.blocks.len());
    // Typical blocks: oak_log, oak_leaves, grass_block, coarse_dirt, moss_block
}
```

**Use Cases:**
- **Treehouses**: Use primary blocks (oak logs) for structure, secondary (leaves) for camouflage
- **Ranger Stations**: Blend buildings naturally into forest landscapes
- **Nature Preserves**: Create realistic woodland environments
- **Elvish Architecture**: Fantasy builds that feel naturally grown

### Desert Palette Examples

```rust
// Desert themes work great for Middle Eastern architecture
if let Some(desert_palette) = BlockPaletteGenerator::generate_natural_palette("desert") {
    // Typical blocks: sand, sandstone, smooth_sandstone, cut_sandstone, red_sand, terracotta
}
```

**Use Cases:**
- **Pyramids**: Layered sandstone variants for realistic aging
- **Oasis Towns**: Transition from sand to more verdant blocks
- **Trade Routes**: Desert outposts and caravanserai
- **Ancient Ruins**: Weathered sandstone combinations

### Ocean Palette Examples

```rust
// Ocean palettes for aquatic builds
if let Some(ocean_palette) = BlockPaletteGenerator::generate_natural_palette("ocean") {
    // Typical blocks: water, prismarine, dark_prismarine, sea_lantern, kelp, sand
}
```

**Use Cases:**
- **Underwater Cities**: Prismarine for structures, sea lanterns for lighting
- **Aquariums**: Natural-looking underwater environments
- **Coastal Buildings**: Transition from ocean to land
- **Submarine Bases**: High-tech underwater installations

### Mountain Palette Examples

```rust
// Rocky, mineral-rich palettes
if let Some(mountain_palette) = BlockPaletteGenerator::generate_natural_palette("mountain") {
    // Typical blocks: stone, cobblestone, andesite, granite, diorite, gravel
}
```

**Use Cases:**
- **Dwarven Halls**: Carved stone architecture with mineral accents
- **Mining Towns**: Industrial settlements in rocky terrain
- **Fortresses**: Imposing stone structures on clifftops
- **Quarries**: Realistic mining operations

### Nether Palette Examples

```rust
// Hellish, otherworldly themes
if let Some(nether_palette) = BlockPaletteGenerator::generate_natural_palette("nether") {
    // Typical blocks: netherrack, nether_bricks, blackstone, crimson_planks, warped_planks
}
```

**Use Cases:**
- **Demon Fortresses**: Dark, imposing architecture
- **Fire Temples**: Religious structures with hellish themes
- **Volcanic Landscapes**: Lava-adjacent terrain
- **End-Game Dungeons**: High-difficulty adventure areas

### End Palette Examples

```rust
// Ethereal, alien environments
if let Some(end_palette) = BlockPaletteGenerator::generate_natural_palette("end") {
    // Typical blocks: end_stone, purpur_block, end_stone_bricks, obsidian, chorus_flower
}
```

**Use Cases:**
- **Space Stations**: Alien, high-tech architecture
- **Mystical Towers**: Magical, otherworldly structures
- **Void Temples**: Ethereal religious buildings
- **Alien Landscapes**: Non-terrestrial environments

---

## 🏗️ Architectural Style Palettes

Architectural palettes provide professionally curated block combinations for specific building styles.

### Medieval Style Examples

```rust
if let Some(medieval_palette) = BlockPaletteGenerator::generate_architectural_palette("medieval") {
    // Primary: cobblestone (foundations, walls)
    // Secondary: oak_planks (floors, frames)
    // Accent: stone_bricks (decorative elements)
}
```

**Detailed Applications:**
- **Castle Walls**: Cobblestone base, stone brick details, oak planks for interior
- **Village Houses**: Oak frame construction with cobblestone foundations
- **Market Squares**: Mixed materials for authentic medieval feel
- **Defensive Structures**: Layered stone types for weathered appearance

**Pro Tips:**
- Layer different stone variants for realistic aging effects
- Use oak planks sparingly for warm accents
- Add moss and cracked variants for ancient feel
- Mix in some gravel paths for authentic terrain

### Modern Style Examples

```rust
if let Some(modern_palette) = BlockPaletteGenerator::generate_architectural_palette("modern") {
    // Primary: white_concrete (clean surfaces)
    // Secondary: light_gray_concrete (supporting elements)
    // Accent: glass (transparency and light)
}
```

**Detailed Applications:**
- **Skyscrapers**: Glass and concrete in 2:1 ratios
- **Minimalist Homes**: Clean white surfaces with gray accents
- **Office Buildings**: Large glass panels with concrete structure
- **Art Museums**: Neutral backgrounds to showcase content

**Pro Tips:**
- Use strict geometric shapes
- Minimize color palette to 3-4 blocks max
- Emphasize clean lines and right angles
- Add iron or quartz details for premium feel

### Rustic Style Examples

```rust
if let Some(rustic_palette) = BlockPaletteGenerator::generate_architectural_palette("rustic") {
    // Primary: stripped_oak_log (natural wood)
    // Secondary: cobblestone (sturdy foundations)
    // Accent: hay_bale (agricultural elements)
}
```

**Detailed Applications:**
- **Farmhouses**: Wood frame with stone foundations
- **Barns**: Large timber construction with hay storage
- **Country Inns**: Cozy, welcoming architecture
- **Windmills**: Traditional agricultural structures

**Pro Tips:**
- Mix stripped and regular logs for aging effects
- Use different wood types for color variation
- Add natural path materials like dirt and gravel
- Include functional elements like hay bales and fences

### Industrial Style Examples

```rust
if let Some(industrial_palette) = BlockPaletteGenerator::generate_architectural_palette("industrial") {
    // Primary: iron_block (structural elements)
    // Secondary: gray_concrete (utilitarian surfaces)
    // Accent: redstone_block (functional components)
}
```

**Detailed Applications:**
- **Factories**: Large-scale industrial production
- **Steampunk Builds**: Victorian-era technology aesthetic
- **Mining Operations**: Heavy machinery and processing
- **Urban Infrastructure**: Bridges, power plants, utilities

**Pro Tips:**
- Add redstone components for functional machinery
- Use anvils and cauldrons for workshop details
- Include observer blocks for high-tech elements
- Create weathering with different gray concrete variants

---

## 🌈 Block Gradient Palettes

Gradient palettes create smooth color transitions between blocks, perfect for organic shapes and artistic builds.

### Natural Landscape Gradients

```rust
// Stone to grass transition for natural terrain
let stone_block = BLOCKS.get("minecraft:stone").unwrap();
let grass_block = BLOCKS.get("minecraft:grass_block").unwrap();

if let Some(gradient) = BlockPaletteGenerator::generate_block_gradient(stone_block, grass_block, 7) {
    // Creates smooth transition: stone → cobblestone → dirt variants → grass
}
```

**Applications:**
- **Hillside Terracing**: Natural-looking landscape layers
- **Riverbank Transitions**: Smooth water-to-land blending
- **Mountain Foothills**: Gradual elevation changes
- **Garden Borders**: Soft transitions between different areas

### Wood Tone Gradients

```rust
// Light to dark wood for depth effects
let oak_block = BLOCKS.get("minecraft:oak_planks").unwrap();
let dark_oak_block = BLOCKS.get("minecraft:dark_oak_planks").unwrap();

if let Some(wood_gradient) = BlockPaletteGenerator::generate_block_gradient(oak_block, dark_oak_block, 5) {
    // Creates wood tone progression for realistic shading
}
```

**Applications:**
- **Log Cabin Shading**: Create depth with wood tones
- **Ship Hull Details**: Weathering effects on wooden ships
- **Furniture Gradients**: Realistic wood grain effects
- **Tree Trunk Variations**: Natural bark color changes

### Color Wheel Gradients

```rust
// Rainbow effects and artistic builds
let red_block = BLOCKS.get("minecraft:red_wool").unwrap();
let blue_block = BLOCKS.get("minecraft:blue_wool").unwrap();

if let Some(rainbow) = BlockPaletteGenerator::generate_block_gradient(red_block, blue_block, 10) {
    // Creates rainbow bridge or artistic color progression
}
```

**Applications:**
- **Rainbow Bridges**: Classic Minecraft build with smooth color flow
- **Sunset Skies**: Simulate natural color gradients
- **Art Installations**: Large-scale pixel art with smooth transitions
- **Team Color Zones**: Gradual transitions between faction areas

---

## ⚫ Monochrome Palettes

Monochrome palettes provide tonal variations of a single color family, perfect for sophisticated and cohesive designs.

### Gray Scale Builds

```rust
// Stone-based monochrome for modern minimalism
let stone_block = BLOCKS.get("minecraft:stone").unwrap();

if let Some(gray_mono) = BlockPaletteGenerator::generate_monochrome_palette(stone_block, 6) {
    // Provides: darkest stone → mid grays → lightest stone variants
}
```

**Applications:**
- **Modern Art Museums**: Neutral backgrounds for exhibits
- **Minimalist Architecture**: Clean, uncluttered designs
- **Futuristic Bases**: High-tech, sterile environments
- **Photography Studios**: Neutral backdrops

### Warm Wood Monochrome

```rust
// Oak-based monochrome for cozy builds
let oak_block = BLOCKS.get("minecraft:oak_planks").unwrap();

if let Some(wood_mono) = BlockPaletteGenerator::generate_monochrome_palette(oak_block, 7) {
    // Provides: dark oak variants → natural oak → light wood tones
}
```

**Applications:**
- **Scandinavian Interiors**: Natural wood aesthetic
- **Cozy Cabins**: Warm, welcoming environments
- **Libraries**: Traditional wood-paneled spaces
- **Craftsman Homes**: Arts and crafts style architecture

### Bold Color Monochrome

```rust
// Red wool monochrome for dramatic effect
let red_block = BLOCKS.get("minecraft:red_wool").unwrap();

if let Some(red_mono) = BlockPaletteGenerator::generate_monochrome_palette(red_block, 5) {
    // Provides: deep burgundy → bright red → pink variants
}
```

**Applications:**
- **Theater Interiors**: Dramatic red velvet aesthetic
- **Valentine's Builds**: Romantic red themes
- **Warning Areas**: High-visibility marking
- **Team Headquarters**: Bold team color identity

---

## 🔄 Complementary Palettes

Complementary palettes use opposite colors on the color wheel for high-contrast, visually striking combinations.

### Red-Green Complementary

```rust
// High contrast for focal points
let red_block = BLOCKS.get("minecraft:red_wool").unwrap();

if let Some(comp_palette) = BlockPaletteGenerator::generate_complementary_palette(red_block) {
    // Provides: red primary + green complement + supporting colors
}
```

**Applications:**
- **Christmas Themes**: Traditional holiday colors
- **Sports Arenas**: Team rivalry color schemes
- **Warning Systems**: High-visibility contrast
- **Art Galleries**: Bold statement pieces

### Blue-Orange Complementary

```rust
// Cool-warm balance
let blue_block = BLOCKS.get("minecraft:blue_wool").unwrap();

if let Some(comp_palette) = BlockPaletteGenerator::generate_complementary_palette(blue_block) {
    // Provides: cool blue + warm orange + neutrals
}
```

**Applications:**
- **Sunset Builds**: Natural warm-cool contrast
- **Portal Themes**: Sci-fi color combinations
- **Beach Resorts**: Ocean blue with sandy orange
- **Fantasy Realms**: Magical contrasting elements

---

## 🔍 Color Range Search

Color range search finds blocks within a specific color tolerance, perfect for matching existing builds or finding alternatives.

### Practical Search Examples

```rust
// Find stone-gray alternatives
let target_gray = ExtendedColorData::from_rgb(128, 128, 128);
let gray_blocks = BlockPaletteGenerator::find_blocks_by_color_range(target_gray, 30.0, 10);

// Find warm earth tones
let target_brown = ExtendedColorData::from_rgb(139, 69, 19);
let earth_blocks = BlockPaletteGenerator::find_blocks_by_color_range(target_brown, 40.0, 8);

// Find ocean blues
let target_blue = ExtendedColorData::from_rgb(70, 130, 180);
let ocean_blocks = BlockPaletteGenerator::find_blocks_by_color_range(target_blue, 45.0, 12);
```

**Tolerance Guidelines:**
- **0-20**: Nearly identical colors (exact matching)
- **20-40**: Similar shades and tints (recommended)
- **40-60**: Same color family (loose matching)
- **60+**: Related colors (very loose)

**Use Cases:**
- **Build Matching**: Find blocks that match existing structures
- **Resource Substitution**: Replace expensive blocks with similar alternatives
- **Texture Packs**: Find blocks that work well with custom textures
- **Color Coordination**: Ensure all blocks work together harmoniously

---

## 📄 Export Formats

Block palettes can be exported in multiple formats for different use cases.

### Text Format Examples

```rust
if let Some(palette) = BlockPaletteGenerator::generate_natural_palette("forest") {
    let text_export = palette.to_text_list();
    println!("{}", text_export);
}
```

**Output Example:**
```
# Forest Biome
Natural forest colors with browns, greens, and earth tones

- Oak Log (#8B7355): Excellent for foundations, walls, and main structures
- Oak Leaves (#6B8E23): Good for supporting elements and medium-scale features
- Grass Block (#7CFC00): Use sparingly for highlights, borders, and eye-catching details
- Coarse Dirt (#8B4513): Perfect for gradual color changes and smooth blending
```

**Best For:**
- Discord and forum sharing
- Build planning documents
- Shopping lists for creative mode
- Tutorial and guide creation

### JSON Format Examples

```rust
if let Some(palette) = BlockPaletteGenerator::generate_natural_palette("forest") {
    let json_export = palette.to_json();
    println!("{}", json_export);
}
```

**Output Structure:**
```json
{
  "name": "Forest Biome",
  "description": "Natural forest colors with browns, greens, and earth tones",
  "theme": "Natural",
  "blocks": [
    {
      "id": "minecraft:oak_log",
      "name": "Oak Log",
      "color": "#8B7355",
      "role": "Primary",
      "usage": "Excellent for foundations, walls, and main structures"
    }
  ]
}
```

**Best For:**
- Mod integration and automation
- Web application development
- Database storage for large projects
- Custom tool development

---

## 🚀 Advanced Use Cases

### Large-Scale City Planning

```rust
// Create district-specific palettes
let residential = BlockPaletteGenerator::generate_architectural_palette("rustic");
let commercial = BlockPaletteGenerator::generate_architectural_palette("modern");
let industrial = BlockPaletteGenerator::generate_architectural_palette("industrial");

// Use gradients for transition zones
if let (Some(res), Some(com)) = (residential, commercial) {
    // Create gradient between residential and commercial areas
}
```

### Seasonal Theme Builds

```rust
// Create seasonal variations of the same build
let spring = BlockPaletteGenerator::generate_natural_palette("forest");
let summer = BlockPaletteGenerator::generate_natural_palette("ocean");
let autumn = BlockPaletteGenerator::generate_natural_palette("desert");
let winter = BlockPaletteGenerator::generate_natural_palette("mountain");
```

### Team Building Projects

```rust
// Assign complementary palettes to different teams
let team_red_base = BLOCKS.get("minecraft:red_wool").unwrap();
let team_blue_base = BLOCKS.get("minecraft:blue_wool").unwrap();

let team_red_palette = BlockPaletteGenerator::generate_complementary_palette(team_red_base);
let team_blue_palette = BlockPaletteGenerator::generate_complementary_palette(team_blue_base);
```

### Historical Architecture Recreation

```rust
// Ancient Egyptian
let egyptian = BlockPaletteGenerator::generate_natural_palette("desert");

// Roman/Greek
let classical = BlockPaletteGenerator::generate_natural_palette("mountain");

// Medieval European
let medieval = BlockPaletteGenerator::generate_architectural_palette("medieval");

// Industrial Revolution
let industrial = BlockPaletteGenerator::generate_architectural_palette("industrial");
```

---

## 💡 Tips & Best Practices

### Color Theory Fundamentals

1. **60-30-10 Rule**: Use 60% primary color, 30% secondary, 10% accent
2. **Warm vs Cool**: Balance warm and cool tones for visual interest
3. **Contrast**: Use high contrast for focal points, low contrast for backgrounds
4. **Hierarchy**: Lighter colors advance, darker colors recede

### Building Techniques

1. **Layering**: Use multiple blocks from the same palette for depth
2. **Transition Zones**: Use gradients between different colored areas
3. **Accent Placement**: Use accent colors sparingly for maximum impact
4. **Scale Consideration**: Lighter colors for details, darker for large surfaces

### Performance Considerations

1. **Chunk Loading**: Similar colored blocks reduce visual complexity
2. **Resource Gathering**: Plan palette blocks based on availability
3. **Multiplayer**: Coordinate palettes with team members
4. **Backup Plans**: Always have alternative blocks in case of shortage

### Documentation Standards

1. **Save Palettes**: Export important palettes for future reference
2. **Version Control**: Track palette changes for large projects
3. **Team Sharing**: Use JSON format for programmatic team coordination
4. **Build Logs**: Document which palettes were used where

### Common Pitfalls

1. **Over-saturation**: Too many bright colors can be overwhelming
2. **Under-contrast**: Insufficient contrast makes builds look flat
3. **Ignoring Context**: Consider surrounding terrain and builds
4. **Palette Drift**: Stick to chosen palettes throughout the project
