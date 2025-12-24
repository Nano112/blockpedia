#!/usr/bin/env cargo run --bin build-data
//! Standalone data builder for blockpedia
//!
//! This tool downloads and processes block data from various sources,
//! generating pre-built data files that can be included in the crate
//! to avoid needing network access during compilation.
//!
//! Note: This binary requires the "build-data" feature to be enabled
//! and uses reqwest with blocking support for HTTP requests.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("Blockpedia Data Builder");
    println!("======================");

    let output_dir = Path::new("data");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).context("Failed to create data directory")?;
    }

    // Download from PrismarineJS
    println!("Downloading PrismarineJS data...");
    download_prismarinejs_data(output_dir)?;

    // Download from MCPropertyEncyclopedia
    println!("Downloading MCPropertyEncyclopedia data...");
    download_mcproperty_data(output_dir)?;

    // Download Bedrock data
    println!("Downloading Bedrock Edition data...");
    download_bedrock_data(output_dir)?;

    println!("Data build complete! Files saved to ./data/");
    println!("You can now build blockpedia with the 'use-prebuilt' feature.");

    Ok(())
}

fn download_prismarinejs_data(output_dir: &Path) -> Result<()> {
    let url = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/pc/1.20.4/blocks.json";

    let response = reqwest::blocking::get(url).context("Failed to download PrismarineJS data")?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    let data = response.text().context("Failed to read response body")?;

    // Validate the data
    let parsed: serde_json::Value =
        serde_json::from_str(&data).context("Failed to parse PrismarineJS JSON")?;

    if !parsed.is_array() {
        anyhow::bail!("PrismarineJS data is not an array");
    }

    let blocks_count = parsed.as_array().unwrap().len();
    println!("✓ Downloaded {} blocks from PrismarineJS", blocks_count);

    let output_file = output_dir.join("prismarinejs_blocks.json");
    fs::write(&output_file, &data)
        .with_context(|| format!("Failed to write to {:?}", output_file))?;

    Ok(())
}

fn download_mcproperty_data(output_dir: &Path) -> Result<()> {
    let url = "https://raw.githubusercontent.com/JoakimThorsen/MCPropertyEncyclopedia/main/data/block_data.json";

    let response =
        reqwest::blocking::get(url).context("Failed to download MCPropertyEncyclopedia data")?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    let data = response.text().context("Failed to read response body")?;

    // Validate the data
    let parsed: serde_json::Value =
        serde_json::from_str(&data).context("Failed to parse MCPropertyEncyclopedia JSON")?;

    if !parsed.is_object() || parsed.get("key_list").is_none() {
        anyhow::bail!("MCPropertyEncyclopedia data format is invalid");
    }

    let key_list = parsed["key_list"]
        .as_array()
        .context("key_list is not an array")?;

    println!(
        "✓ Downloaded {} blocks from MCPropertyEncyclopedia",
        key_list.len()
    );

    let output_file = output_dir.join("mcproperty_blocks.json");
    fs::write(&output_file, &data)
        .with_context(|| format!("Failed to write to {:?}", output_file))?;

    Ok(())
}

fn download_bedrock_data(output_dir: &Path) -> Result<()> {
    // 1. Download blocks.json (for metadata like transparency)
    let url_blocks = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/bedrock/1.21.0/blocks.json";
    let response_blocks =
        reqwest::blocking::get(url_blocks).context("Failed to download Bedrock blocks.json")?;
    let data_blocks = response_blocks
        .text()
        .context("Failed to read blocks.json response")?;
    fs::write(output_dir.join("bedrock_blocks.json"), &data_blocks)?;
    println!("✓ Downloaded Bedrock blocks.json");

    // 2. Download blockStates.json (for properties and values)
    let url_states = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/bedrock/1.21.0/blockStates.json";
    let response_states = reqwest::blocking::get(url_states)
        .context("Failed to download Bedrock blockStates.json")?;
    let data_states = response_states
        .text()
        .context("Failed to read blockStates.json response")?;
    fs::write(output_dir.join("bedrock_block_states.json"), &data_states)?;
    println!("✓ Downloaded Bedrock blockStates.json");

    // 3. Download Java -> Bedrock mapping (blocksJ2B.json)
    let url_j2b = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/bedrock/1.21.0/blocksJ2B.json";
    let response_j2b =
        reqwest::blocking::get(url_j2b).context("Failed to download Bedrock blocksJ2B.json")?;
    let data_j2b = response_j2b
        .text()
        .context("Failed to read blocksJ2B.json response")?;
    fs::write(output_dir.join("bedrock_blocks_j2b.json"), &data_j2b)?;
    println!("✓ Downloaded Bedrock blocksJ2B.json");

    // 4. Download Bedrock -> Java mapping (blocksB2J.json)
    let url_b2j = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/bedrock/1.21.0/blocksB2J.json";
    let response_b2j =
        reqwest::blocking::get(url_b2j).context("Failed to download Bedrock blocksB2J.json")?;
    let data_b2j = response_b2j
        .text()
        .context("Failed to read blocksB2J.json response")?;
    fs::write(output_dir.join("bedrock_blocks_b2j.json"), &data_b2j)?;
    println!("✓ Downloaded Bedrock blocksB2J.json");

    Ok(())
}
