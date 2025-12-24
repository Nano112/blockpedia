#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use crate::{color::ExtendedColorData, query_builder::*, BLOCKS};

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[derive(Serialize, Deserialize)]
pub struct WasmBlockInfo {
    pub id: String,
    pub rgb: [u8; 3],
    pub hsl: [f32; 3],
    pub oklab: [f32; 3],
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[derive(Serialize, Deserialize)]
pub struct WasmGradientResult {
    pub blocks: Vec<WasmBlockInfo>,
    pub success: bool,
    pub error: Option<String>,
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn init() {
    console_error_panic_hook::set_once();

    #[cfg(feature = "wee_alloc")]
    {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }

    log!("Blockpedia WASM initialized with {} blocks", BLOCKS.len());
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn get_all_colored_blocks() -> JsValue {
    let colored_blocks: Vec<WasmBlockInfo> = BLOCKS
        .values()
        .filter_map(|block| {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                Some(WasmBlockInfo {
                    id: block.id().to_string(),
                    rgb: ext_color.rgb,
                    hsl: ext_color.hsl,
                    oklab: ext_color.oklab,
                })
            } else {
                None
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&colored_blocks).unwrap()
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn generate_gradient_between_blocks(
    start_block_id: &str,
    end_block_id: &str,
    steps: usize,
    color_space: &str,
    easing: &str,
) -> JsValue {
    let color_space = match color_space {
        "rgb" => ColorSpace::Rgb,
        "hsl" => ColorSpace::Hsl,
        "oklab" => ColorSpace::Oklab,
        "lab" => ColorSpace::Lab,
        _ => ColorSpace::Oklab,
    };

    let easing_fn = match easing {
        "linear" => EasingFunction::Linear,
        "ease-in" => EasingFunction::EaseIn,
        "ease-out" => EasingFunction::EaseOut,
        "ease-in-out" => EasingFunction::EaseInOut,
        "sine" => EasingFunction::Sine,
        "exponential" => EasingFunction::Exponential,
        _ => EasingFunction::Linear,
    };

    let config = GradientConfig::new(steps)
        .with_color_space(color_space)
        .with_easing(easing_fn);

    match (BLOCKS.get(start_block_id), BLOCKS.get(end_block_id)) {
        (Some(start_block), Some(end_block)) => {
            if let (Some(_start_color), Some(_end_color)) =
                (start_block.extras.color, end_block.extras.color)
            {
                let gradient = BlockQuery::generate_gradient_between_blocks(
                    start_block_id,
                    end_block_id,
                    config,
                );

                let blocks: Vec<WasmBlockInfo> = gradient
                    .collect()
                    .into_iter()
                    .filter_map(|block| {
                        if let Some(color) = block.extras.color {
                            let ext_color = color.to_extended();
                            Some(WasmBlockInfo {
                                id: block.id().to_string(),
                                rgb: ext_color.rgb,
                                hsl: ext_color.hsl,
                                oklab: ext_color.oklab,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                let result = WasmGradientResult {
                    blocks,
                    success: true,
                    error: None,
                };

                serde_wasm_bindgen::to_value(&result).unwrap()
            } else {
                let result = WasmGradientResult {
                    blocks: Vec::new(),
                    success: false,
                    error: Some("One or both blocks don't have color data".to_string()),
                };
                serde_wasm_bindgen::to_value(&result).unwrap()
            }
        }
        _ => {
            let result = WasmGradientResult {
                blocks: Vec::new(),
                success: false,
                error: Some(format!(
                    "Block(s) not found: {} or {}",
                    start_block_id, end_block_id
                )),
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        }
    }
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn generate_gradient_between_colors(
    start_r: u8,
    start_g: u8,
    start_b: u8,
    end_r: u8,
    end_g: u8,
    end_b: u8,
    steps: usize,
    color_space: &str,
    easing: &str,
) -> JsValue {
    let color_space = match color_space {
        "rgb" => ColorSpace::Rgb,
        "hsl" => ColorSpace::Hsl,
        "oklab" => ColorSpace::Oklab,
        "lab" => ColorSpace::Lab,
        _ => ColorSpace::Oklab,
    };

    let easing_fn = match easing {
        "linear" => EasingFunction::Linear,
        "ease-in" => EasingFunction::EaseIn,
        "ease-out" => EasingFunction::EaseOut,
        "ease-in-out" => EasingFunction::EaseInOut,
        "sine" => EasingFunction::Sine,
        "exponential" => EasingFunction::Exponential,
        _ => EasingFunction::Linear,
    };

    let config = GradientConfig::new(steps)
        .with_color_space(color_space)
        .with_easing(easing_fn);

    let start_color = ExtendedColorData::from_rgb(start_r, start_g, start_b);
    let end_color = ExtendedColorData::from_rgb(end_r, end_g, end_b);

    let gradient = AllBlocks::new()
        .with_color()
        .generate_gradient_between_colors(start_color, end_color, config);

    let blocks: Vec<WasmBlockInfo> = gradient
        .collect()
        .into_iter()
        .filter_map(|block| {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                Some(WasmBlockInfo {
                    id: block.id().to_string(),
                    rgb: ext_color.rgb,
                    hsl: ext_color.hsl,
                    oklab: ext_color.oklab,
                })
            } else {
                None
            }
        })
        .collect();

    let result = WasmGradientResult {
        blocks,
        success: true,
        error: None,
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn sort_blocks_by_color_gradient(block_ids: Vec<String>) -> JsValue {
    let blocks: Vec<&'static crate::BlockFacts> = block_ids
        .into_iter()
        .filter_map(|id| BLOCKS.get(&id).copied())
        .collect();

    if blocks.len() <= 1 {
        let blocks_info: Vec<WasmBlockInfo> = blocks
            .into_iter()
            .filter_map(|block| {
                if let Some(color) = block.extras.color {
                    let ext_color = color.to_extended();
                    Some(WasmBlockInfo {
                        id: block.id().to_string(),
                        rgb: ext_color.rgb,
                        hsl: ext_color.hsl,
                        oklab: ext_color.oklab,
                    })
                } else {
                    None
                }
            })
            .collect();

        return serde_wasm_bindgen::to_value(&blocks_info).unwrap();
    }

    // Sort using the color gradient sorting algorithm
    // Use a dummy query and filter to only the blocks we want
    let block_ids: std::collections::HashSet<&str> = blocks.iter().map(|b| b.id()).collect();
    let sorted = AllBlocks::new()
        .with_color()
        .collect()
        .into_iter()
        .filter(|block| block_ids.contains(block.id()))
        .collect::<Vec<_>>();

    // Now create a new query from the filtered blocks and sort
    let sorted = if sorted.len() <= 1 {
        sorted
    } else {
        // Implement the same sorting logic here
        let mut result = Vec::new();
        let mut remaining = sorted;

        if !remaining.is_empty() {
            result.push(remaining.remove(0)); // Start with first block

            while !remaining.is_empty() {
                let current_color = result.last().unwrap().extras.color.unwrap().to_extended();

                // Find the closest remaining color
                let mut best_index = 0;
                let mut best_distance = f32::INFINITY;

                for (i, block) in remaining.iter().enumerate() {
                    let distance = block
                        .extras
                        .color
                        .unwrap()
                        .to_extended()
                        .distance_oklab(&current_color);
                    if distance < best_distance {
                        best_distance = distance;
                        best_index = i;
                    }
                }

                result.push(remaining.remove(best_index));
            }
        }
        result
    };

    let blocks_info: Vec<WasmBlockInfo> = sorted
        .into_iter()
        .filter_map(|block| {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                Some(WasmBlockInfo {
                    id: block.id().to_string(),
                    rgb: ext_color.rgb,
                    hsl: ext_color.hsl,
                    oklab: ext_color.oklab,
                })
            } else {
                None
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&blocks_info).unwrap()
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn get_block_info(block_id: &str) -> JsValue {
    match BLOCKS.get(block_id) {
        Some(block) => {
            if let Some(color) = block.extras.color {
                let ext_color = color.to_extended();
                let block_info = WasmBlockInfo {
                    id: block.id().to_string(),
                    rgb: ext_color.rgb,
                    hsl: ext_color.hsl,
                    oklab: ext_color.oklab,
                };
                serde_wasm_bindgen::to_value(&Some(block_info)).unwrap()
            } else {
                serde_wasm_bindgen::to_value(&None::<WasmBlockInfo>).unwrap()
            }
        }
        None => serde_wasm_bindgen::to_value(&None::<WasmBlockInfo>).unwrap(),
    }
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn get_color_spaces() -> JsValue {
    let color_spaces = vec!["rgb", "hsl", "oklab", "lab"];
    serde_wasm_bindgen::to_value(&color_spaces).unwrap()
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn get_easing_functions() -> JsValue {
    let easing_functions = vec![
        "linear",
        "ease-in",
        "ease-out",
        "ease-in-out",
        "sine",
        "exponential",
    ];
    serde_wasm_bindgen::to_value(&easing_functions).unwrap()
}
