use blockpedia::query_builder::*;
use blockpedia::color::ExtendedColorData;

fn main() {
    println!("=== Blockpedia Gradient Generation Demo ===\n");
    
    // Demo 1: Different Color Spaces
    println!("1. Gradient Generation in Different Color Spaces");
    println!("   Creating a 5-step gradient from red to blue:\n");
    
    let start_color = ExtendedColorData::from_rgb(255, 0, 0); // Red
    let end_color = ExtendedColorData::from_rgb(0, 0, 255);   // Blue
    
    let color_spaces = [
        ("RGB", ColorSpace::Rgb),
        ("HSL", ColorSpace::Hsl),
        ("Oklab", ColorSpace::Oklab),
        ("Lab", ColorSpace::Lab),
    ];
    
    for (name, color_space) in &color_spaces {
        let config = GradientConfig::new(5)
            .with_color_space(*color_space)
            .with_easing(EasingFunction::Linear);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(start_color, end_color, config);
        
        println!("   {} Color Space: {} blocks found", name, gradient.len());
        for (i, block) in gradient.collect().iter().enumerate() {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                println!("     {}: {} - RGB({}, {}, {})", 
                    i + 1, block.id(), ext_color.rgb[0], ext_color.rgb[1], ext_color.rgb[2]);
            }
        }
        println!();
    }
    
    // Demo 2: Different Easing Functions
    println!("2. Gradient Generation with Different Easing Functions");
    println!("   Creating a 6-step gradient from black to white:\n");
    
    let black = ExtendedColorData::from_rgb(0, 0, 0);
    let white = ExtendedColorData::from_rgb(255, 255, 255);
    
    let easing_functions = [
        ("Linear", EasingFunction::Linear),
        ("Ease In", EasingFunction::EaseIn),
        ("Ease Out", EasingFunction::EaseOut),
        ("Ease In-Out", EasingFunction::EaseInOut),
        ("Sine", EasingFunction::Sine),
    ];
    
    for (name, easing) in &easing_functions {
        let config = GradientConfig::new(6)
            .with_color_space(ColorSpace::Rgb)
            .with_easing(*easing);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(black, white, config);
        
        println!("   {} Easing: {} blocks found", name, gradient.len());
        for (i, block) in gradient.collect().iter().enumerate() {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                let brightness = (ext_color.rgb[0] as u32 + ext_color.rgb[1] as u32 + ext_color.rgb[2] as u32) / 3;
                println!("     {}: {} - Brightness: {}", 
                    i + 1, block.id().replace("minecraft:", ""), brightness);
            }
        }
        println!();
    }
    
    // Demo 3: Different Sampling Methods
    println!("3. Gradient Generation with Different Sampling Methods");
    println!("   Creating a 4-step gradient from green to purple:\n");
    
    let green = ExtendedColorData::from_rgb(0, 255, 0);
    let purple = ExtendedColorData::from_rgb(128, 0, 128);
    
    let sampling_methods = [
        ("Dominant", ColorSamplingMethod::Dominant),
        ("Average", ColorSamplingMethod::Average),
        ("Clustering", ColorSamplingMethod::Clustering { k: 3 }),
        ("Most Frequent", ColorSamplingMethod::MostFrequent { bins: 16 }),
    ];
    
    for (name, sampling) in &sampling_methods {
        let config = GradientConfig::new(4)
            .with_color_space(ColorSpace::Oklab)
            .with_easing(EasingFunction::Linear)
            .with_sampling(*sampling);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(green, purple, config);
        
        println!("   {} Sampling: {} blocks found", name, gradient.len());
        for (i, block) in gradient.collect().iter().enumerate() {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                println!("     {}: {} - RGB({}, {}, {})", 
                    i + 1, block.id().replace("minecraft:", ""), 
                    ext_color.rgb[0], ext_color.rgb[1], ext_color.rgb[2]);
            }
        }
        println!();
    }
    
    // Demo 4: Different Gradient Sizes
    println!("4. Gradient Generation with Different Sizes");
    println!("   Creating gradients of varying lengths from orange to cyan:\n");
    
    let orange = ExtendedColorData::from_rgb(255, 165, 0);
    let cyan = ExtendedColorData::from_rgb(0, 255, 255);
    
    let sizes = [2, 5, 10, 15];
    
    for &size in &sizes {
        let config = GradientConfig::new(size)
            .with_color_space(ColorSpace::Hsl)
            .with_easing(EasingFunction::EaseInOut);
        
        let gradient = AllBlocks::new()
            .with_color()
            .generate_gradient_between_colors(orange, cyan, config);
        
        println!("   {}-step gradient: {} blocks found", size, gradient.len());
        let blocks = gradient.collect();
        for (i, block) in blocks.iter().enumerate() {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                println!("     {}: {} - HSL({:.0}, {:.0}%, {:.0}%)", 
                    i + 1, block.id().replace("minecraft:", ""), 
                    ext_color.hsl[0], ext_color.hsl[1] * 100.0, ext_color.hsl[2] * 100.0);
            }
        }
        println!();
    }
    
    // Demo 5: Multi-color gradient
    println!("5. Multi-Color Gradient");
    println!("   Creating a gradient through existing block colors:\n");
    
    let config = GradientConfig::new(8)
        .with_color_space(ColorSpace::Oklab)
        .with_easing(EasingFunction::Linear);
    
    let multi_gradient = AllBlocks::new()
        .with_color()
        .limit(15) // Use subset of colored blocks
        .generate_multi_gradient(config);
    
    println!("   Multi-color gradient: {} blocks found", multi_gradient.len());
    for (i, block) in multi_gradient.collect().iter().enumerate() {
        if let Some(color) = block.extras.color {
            let ext_color = color.to_extended();
            println!("     {}: {} - RGB({}, {}, {})", 
                i + 1, block.id().replace("minecraft:", ""), 
                ext_color.rgb[0], ext_color.rgb[1], ext_color.rgb[2]);
        }
    }
    println!();
    
    // Demo 6: Color-sorted gradient
    println!("6. Color-Sorted Gradient");
    println!("   Arranging blocks by color similarity:\n");
    
    let sorted_blocks = AllBlocks::new()
        .with_color()
        .limit(12)
        .sort_by_color_gradient();
    
    println!("   Color-sorted blocks: {} blocks found", sorted_blocks.len());
    for (i, block) in sorted_blocks.collect().iter().enumerate() {
        if let Some(color) = block.extras.color {
            let ext_color = color.to_extended();
            println!("     {}: {} - RGB({}, {}, {})", 
                i + 1, block.id().replace("minecraft:", ""), 
                ext_color.rgb[0], ext_color.rgb[1], ext_color.rgb[2]);
        }
    }
    
    println!("\n=== Demo Complete ===");
    println!("The gradient system supports:");
    println!("  • 4 color spaces: RGB, HSL, Oklab, Lab");
    println!("  • 7 easing functions: Linear, EaseIn, EaseOut, EaseInOut, CubicBezier, Sine, Exponential");
    println!("  • 5 sampling methods: Dominant, Average, Clustering, EdgeWeighted, MostFrequent");
    println!("  • Variable gradient sizes (1 to any number)");
    println!("  • Multi-color gradients through existing block colors");
    println!("  • Color-based sorting for smooth transitions");
}
