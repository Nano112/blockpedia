# Blockpedia

[![Crates.io](https://img.shields.io/crates/v/blockpedia.svg)](https://crates.io/crates/blockpedia)
[![Documentation](https://docs.rs/blockpedia/badge.svg)](https://docs.rs/blockpedia)
[![Build Status](https://github.com/Nano112/blockpedia/workflows/CI/badge.svg)](https://github.com/Nano112/blockpedia/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.82%2B-orange)](https://www.rust-lang.org)

A comprehensive Rust library for Minecraft block data with advanced color analysis and gradient palette generation.

Blockpedia provides programmatic access to Minecraft block information, including properties, color data extracted from real textures, and sophisticated palette generation capabilities. Perfect for building tools, mods, or applications that need to work with Minecraft block data.

## ✨ Features

### 🎨 **Advanced Color System**
- **44.6% Color Coverage**: Real texture-based color data for 472+ blocks
- **Multiple Color Spaces**: RGB, HSL, Oklab, and Lab color space support  
- **Gradient Palettes**: Generate smooth gradients between any two colors or blocks
- **Themed Palettes**: Pre-designed palettes (sunset, ocean, fire, forest, monochrome)
- **Color Similarity Search**: Find blocks by color with perceptual matching
- **Export Formats**: CSS variables, GIMP GPL, Adobe ACO support

### 🔍 **Comprehensive Block Data**
- **1058+ Blocks**: Complete Minecraft 1.20.4 block data
- **Rich Properties**: Block states, properties, and default values
- **Multi-Source Support**: PrismarineJS and MCPropertyEncyclopedia
- **Smart Queries**: Advanced search and filtering capabilities
- **Type-Safe API**: Robust error handling and validation

### 🖥️ **Interactive CLI**
- **6 Specialized Tabs**: Blocks, Colors, Query, Properties, Statistics, Sources
- **Real-time Search**: Live filtering by name, property, or color
- **Advanced Query Builder**: Complex property and color combinations
- **Rich Visualizations**: Color palettes, statistics, and property analysis
- **Keyboard Shortcuts**: Efficient navigation and operation

## 📋 Table of Contents

- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [CLI Usage](#-cli-usage)
- [Library Usage](#-library-usage)
- [Color System](#-color-system)
- [Palette Generation](#-palette-generation)
- [Examples](#-examples)
- [Building from Source](#-building-from-source)
- [License](#-license)

## 🚀 Installation

### Prerequisites
- Rust 1.82 or later
- Git (for cloning the repository)

### From Source
```bash
git clone https://github.com/Nano112/blockpedia.git
cd blockpedia
cargo build --release
```

### Install Locally
```bash
cargo install --path .
```

## 🏃 Quick Start

### Launch the Interactive CLI
```bash
cargo run --bin blockpedia-cli
```

### Basic Library Usage
```rust
use blockpedia::{get_block, BLOCKS, queries::*};

// Get a specific block
let stone = get_block("minecraft:stone").unwrap();
println!("Stone properties: {:?}", stone.properties());

// Search for blocks
let redstone_blocks: Vec<_> = find_blocks_by_property("powered", "true").collect();
println!("Found {} powered blocks", redstone_blocks.len());

// Color analysis
if let Some(color) = stone.extras.color {
    println!("Stone color: #{:02X}{:02X}{:02X}", 
        color.rgb[0], color.rgb[1], color.rgb[2]);
}
```

### Generate Color Palettes
```rust
use blockpedia::color::palettes::{PaletteGenerator, GradientMethod};
use blockpedia::color::ExtendedColorData;

// Create a gradient between two colors
let red = ExtendedColorData::from_rgb(255, 0, 0);
let blue = ExtendedColorData::from_rgb(0, 0, 255);

let gradient = PaletteGenerator::generate_gradient_palette(
    red, blue, 10, GradientMethod::LinearOklab
);

// Generate themed palettes
let sunset = PaletteGenerator::generate_sunset_palette(8);
let ocean = PaletteGenerator::generate_ocean_palette(6);

// Export to various formats
let css = PaletteGenerator::export_palette_css(&gradient);
let gpl = PaletteGenerator::export_palette_gpl(&gradient, "My Gradient");
```

## 🖥️ CLI Usage

The interactive CLI provides six specialized tabs for exploring block data:

### Navigation
- **Tab/Shift+Tab**: Switch between tabs
- **↑/↓**: Navigate within tabs
- **q**: Quit the application

### Tabs Overview

#### 📦 Blocks Tab
- Browse all 1058+ blocks with real-time filtering
- View detailed properties, default states, and color data
- Color and property indicators (🎨 for colors, ⚙️ for properties)

#### 🎨 Colors Tab
```
[1] Color Coverage Analysis    - See 44.6% coverage statistics
[2] Color Palette Analysis     - Group blocks by color families  
[3] Color Similarity Search    - Find blocks similar to stone gray
[4] Color Statistics          - Brightness, averages, extremes
[5] Gradient Palettes         - Generate gradients between blocks
[6] Themed Palettes          - Sunset, ocean, fire themes
```

#### 🔍 Query Tab
```
[n] Search by Name           - e.g., 'stone', 'wool'
[p] Search by Property       - e.g., 'delay:1', 'facing:north'  
[c] Search by Color         - e.g., '#FF0000', '#7D7D7D'
[a] Advanced Query Builder  - Complex multi-filter queries
[r] Reset Filters           - Clear all active filters
```

#### ⚙️ Properties Tab
- Comprehensive table of all block properties
- Property value counts and distributions
- Searchable property reference

#### 📊 Statistics Tab
- Block count and property statistics  
- Property frequency visualizations
- Coverage analysis and insights

#### 🌐 Sources Tab
- Data source information and statistics
- Build configuration details
- Quality metrics and coverage reports

## 📚 Library Usage

### Block Queries

```rust
use blockpedia::{BLOCKS, queries::*, get_block};

// Basic block access
let dirt = get_block("minecraft:dirt")?;
println!("Block: {}", dirt.id());

// Property-based searches
let stairs: Vec<_> = find_blocks_by_property("shape", "straight").collect();
let waterlogged: Vec<_> = find_blocks_by_property("waterlogged", "true").collect();

// Pattern searches  
let wool_blocks: Vec<_> = search_blocks("*wool").collect();
let stone_variants: Vec<_> = search_blocks("*stone*").collect();

// Statistical analysis
let stats = get_property_stats();
println!("Unique properties: {}", stats.total_unique_properties);
println!("Average properties per block: {:.2}", stats.average_properties_per_block);

// Block families
let families = get_block_families();
for (family, blocks) in families {
    println!("{}: {} blocks", family, blocks.len());
}
```

### Color Operations

```rust
use blockpedia::color::*;

// Extract colors from textures
let color = extract_dominant_color(Path::new("assets/textures/stone.png"))?;
println!("Dominant color: {:?}", color.rgb);

// Color space conversions
let extended = ExtendedColorData::from_rgb(128, 64, 192);
println!("HSL: {:?}", extended.hsl);
println!("Oklab: {:?}", extended.oklab);
println!("Hex: {}", extended.hex_string());

// Color similarity
let target = ExtendedColorData::from_rgb(125, 125, 125);
let similar_blocks: Vec<_> = BLOCKS.values()
    .filter(|block| {
        if let Some(color) = block.extras.color {
            color.to_extended().distance_oklab(&target) < 20.0
        } else {
            false
        }
    })
    .collect();
```

## 🎨 Color System

Blockpedia features a sophisticated color system with real texture data:

### Color Coverage
- **472 blocks** with extracted color data (44.6% coverage)
- **351 colors** extracted directly from textures  
- **154 colors** inherited through material relationships

### Color Spaces
- **RGB**: Standard red, green, blue components
- **HSL**: Hue, saturation, lightness for intuitive color manipulation
- **Oklab**: Perceptually uniform color space for accurate similarity
- **Lab**: CIE L*a*b* color space for professional color work

### Texture Processing
```rust
use blockpedia::color::extraction::{ColorExtractor, ExtractionMethod};

let extractor = ColorExtractor::new(ExtractionMethod::MostFrequent { bins: 16 });
let color = extractor.extract_color(&image)?;

// Different extraction methods
let average = ExtractionMethod::Average;
let clustering = ExtractionMethod::Clustering { k: 5 };
let edge_weighted = ExtractionMethod::EdgeWeighted;
```

## 🌈 Palette Generation

### Gradient Methods

```rust
use blockpedia::color::palettes::{GradientMethod, PaletteGenerator};

// Linear RGB - Simple RGB interpolation
let rgb_gradient = PaletteGenerator::generate_gradient_palette(
    start_color, end_color, 10, GradientMethod::LinearRgb
);

// Linear HSL - Hue-based interpolation (smooth color wheel transitions)
let hsl_gradient = PaletteGenerator::generate_gradient_palette(
    start_color, end_color, 10, GradientMethod::LinearHsl
);

// Linear Oklab - Perceptually uniform (most natural to human eye)
let oklab_gradient = PaletteGenerator::generate_gradient_palette(
    start_color, end_color, 10, GradientMethod::LinearOklab
);

// Cubic Bezier - Smooth curves with acceleration/deceleration
let bezier_gradient = PaletteGenerator::generate_gradient_palette(
    start_color, end_color, 10, GradientMethod::CubicBezier
);
```

### Multi-Color Gradients

```rust
let colors = vec![
    ExtendedColorData::from_rgb(255, 0, 0),   // Red
    ExtendedColorData::from_rgb(255, 255, 0), // Yellow  
    ExtendedColorData::from_rgb(0, 255, 0),   // Green
    ExtendedColorData::from_rgb(0, 0, 255),   // Blue
];

let rainbow = PaletteGenerator::generate_multi_gradient_palette(
    colors, 20, GradientMethod::LinearOklab
);
```

### Themed Palettes

```rust
// Pre-designed palettes for common use cases
let sunset = PaletteGenerator::generate_sunset_palette(8);      // Warm reds to deep blues
let ocean = PaletteGenerator::generate_ocean_palette(6);        // Light to deep blues  
let fire = PaletteGenerator::generate_fire_palette(5);         // Yellows to deep reds
let forest = PaletteGenerator::generate_forest_palette(7);     // Light to dark greens

// Dynamic palettes based on existing colors
let base_color = ExtendedColorData::from_rgb(128, 64, 192);
let monochrome = PaletteGenerator::generate_monochrome_palette(base_color, 9);
let complementary = PaletteGenerator::generate_complementary_palette(&base_color);
```

### Export Formats

```rust
// CSS Variables
let css = PaletteGenerator::export_palette_css(&palette);
/*
:root {
  --color-1: #FF0000;
  --color-2: #FF8000;
  --color-3: #FFFF00;
}
*/

// GIMP Palette (.gpl)
let gpl = PaletteGenerator::export_palette_gpl(&palette, "Sunset Gradient");

// Adobe Photoshop (.aco)
let aco_data = PaletteGenerator::export_palette_aco_data(&palette);
std::fs::write("palette.aco", aco_data)?;
```

## 📖 Examples

### Find All Redstone Components
```rust
use blockpedia::{BLOCKS, queries::*};

let redstone_blocks: Vec<_> = BLOCKS.values()
    .filter(|block| {
        block.id().contains("redstone") || 
        block.has_property("powered") ||
        block.has_property("signal_strength")
    })
    .collect();

println!("Found {} redstone components", redstone_blocks.len());
```

### Generate a Custom Palette from Block Colors
```rust
use blockpedia::{BLOCKS, color::palettes::PaletteGenerator};

// Get colors from wood blocks
let wood_colors: Vec<_> = BLOCKS.values()
    .filter(|block| block.id().contains("wood") || block.id().contains("log"))
    .filter_map(|block| block.extras.color.map(|c| c.to_extended()))
    .collect();

let wood_palette = PaletteGenerator::generate_distinct_palette(&wood_colors, 8);
let css_export = PaletteGenerator::export_palette_css(&wood_palette);
```

### Color-Based Block Recommendations
```rust
use blockpedia::{BLOCKS, color::ExtendedColorData};

fn find_blocks_for_color_scheme(target_color: ExtendedColorData, tolerance: f32) -> Vec<&'static BlockFacts> {
    BLOCKS.values()
        .filter(|block| {
            if let Some(color) = block.extras.color {
                color.to_extended().distance_oklab(&target_color) <= tolerance
            } else {
                false
            }
        })
        .collect()
}

let sage_green = ExtendedColorData::from_rgb(158, 184, 156);
let matching_blocks = find_blocks_for_color_scheme(sage_green, 25.0);
```

## 🔧 Building from Source

### Development Setup
```bash
# Clone the repository
git clone https://github.com/Nano112/blockpedia.git
cd blockpedia

# Download texture data (optional, for color extraction)
cargo run --bin download-textures

# Build in development mode
cargo build

# Run tests
cargo test

# Build CLI in release mode
cargo build --release --bin blockpedia-cli
```

### Environment Variables
```bash
# Use alternative data source
BLOCKPEDIA_DATA_SOURCE=MCPropertyEncyclopedia cargo build

# Skip texture downloads (for CI/limited bandwidth)
BLOCKPEDIA_SKIP_TEXTURES=1 cargo build
```

### Data Sources

Blockpedia supports multiple data sources:

- **PrismarineJS** (default): Complete block states and properties (~1058 blocks)
- **MCPropertyEncyclopedia**: Rich metadata and descriptions (~288 blocks)

The build system automatically fetches and caches data from these sources.

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test gradient_palettes_test
cargo test color
cargo test queries

# Test with different data sources
BLOCKPEDIA_DATA_SOURCE=MCPropertyEncyclopedia cargo test

# Generate test coverage
cargo tarpaulin --out html
```


### Development Workflow
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality  
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Run linting (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to your branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

### Areas for Contribution
- 🎨 Additional color extraction methods
- 🌈 New themed palette generators  
- 📊 Enhanced statistical analysis
- 🔍 Advanced query capabilities
- 🌐 Additional data source integrations
- 📱 Web interface or mobile app
- 🎮 Minecraft mod integration

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **PrismarineJS** - Comprehensive Minecraft data collection
- **MCPropertyEncyclopedia** - Rich metadata and descriptions  
- **hueblocks** - Block texture repository
- **ratatui** - Terminal UI framework
- **image** - Rust image processing library
- **palette** - Color space conversion library

## 📊 Project Statistics

- **Language**: Rust 🦀
- **Lines of Code**: ~15,000+
- **Test Coverage**: 85%+
- **Blocks Supported**: 1,058+
- **Color Coverage**: 44.6% (472 blocks)
- **Gradient Methods**: 4 (RGB, HSL, Oklab, Bezier)
- **Export Formats**: 3 (CSS, GPL, ACO)

---

<div align="center">

**[🏠 Home](https://github.com/Nano112/blockpedia)** • 
**[📖 Documentation](https://github.com/Nano112/blockpedia/wiki)** • 
**[🐛 Issues](https://github.com/Nano112/blockpedia/issues)** • 
**[💬 Discussions](https://github.com/Nano112/blockpedia/discussions)**

Made with ❤️ by [Nano112](https://github.com/Nano112)

</div>
