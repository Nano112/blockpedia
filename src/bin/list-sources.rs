use blockpedia::BLOCKS;

fn main() {
    println!("🌐 Blockpedia Data Sources");
    println!("==========================");
    println!();
    
    // Detect current source based on block count
    let total_blocks = BLOCKS.len();
    let current_source = if total_blocks > 1000 {
        "PrismarineJS"
    } else if total_blocks > 200 {
        "MCPropertyEncyclopedia"  
    } else {
        "Test Data"
    };
    
    println!("📊 Current Build Information:");
    println!("  Active Source: {}", current_source);
    println!("  Total Blocks: {}", total_blocks);
    println!();
    
    println!("📋 Available Data Sources:");
    println!("  1. PrismarineJS");
    println!("     • Complete block states and properties");
    println!("     • ~1058 blocks");
    println!("     • Best for technical block data");
    println!("     • URL: https://github.com/PrismarineJS/minecraft-data");
    println!();
    
    println!("  2. MCPropertyEncyclopedia");
    println!("     • Rich metadata and descriptions");
    println!("     • ~288 blocks");
    println!("     • Best for detailed property information");
    println!("     • URL: https://github.com/JoakimThorsen/MCPropertyEncyclopedia");
    println!();
    
    println!("🔧 To switch data sources:");
    println!("  BLOCKPEDIA_DATA_SOURCE=PrismarineJS cargo build");
    println!("  BLOCKPEDIA_DATA_SOURCE=MCPropertyEncyclopedia cargo build");
    println!();
    
    // Quick analysis
    let blocks_with_properties = BLOCKS.values().filter(|b| !b.properties.is_empty()).count();
    let blocks_with_color = BLOCKS.values().filter(|b| b.extras.color.is_some()).count();
    let blocks_with_mock_data = BLOCKS.values().filter(|b| b.extras.mock_data.is_some()).count();
    
    println!("📈 Current Build Analysis:");
    println!("  • Blocks with properties: {} ({:.1}%)", 
        blocks_with_properties, 
        (blocks_with_properties as f64 / total_blocks as f64) * 100.0);
    println!("  • Blocks with color data: {} ({:.1}%)", 
        blocks_with_color,
        (blocks_with_color as f64 / total_blocks as f64) * 100.0);
    println!("  • Blocks with extra data: {} ({:.1}%)", 
        blocks_with_mock_data,
        (blocks_with_mock_data as f64 / total_blocks as f64) * 100.0);
}
