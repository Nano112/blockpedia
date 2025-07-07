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
    
    println!("Data build complete! Files saved to ./data/");
    println!("You can now build blockpedia with the 'use-prebuilt' feature.");
    
    Ok(())
}

fn download_prismarinejs_data(output_dir: &Path) -> Result<()> {
    let url = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/pc/1.20.4/blocks.json";
    
    let response = reqwest::blocking::get(url)
        .context("Failed to download PrismarineJS data")?;
    
    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }
    
    let data = response.text().context("Failed to read response body")?;
    
    // Validate the data
    let parsed: serde_json::Value = serde_json::from_str(&data)
        .context("Failed to parse PrismarineJS JSON")?;
    
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
    
    let response = reqwest::blocking::get(url)
        .context("Failed to download MCPropertyEncyclopedia data")?;
    
    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }
    
    let data = response.text().context("Failed to read response body")?;
    
    // Validate the data
    let parsed: serde_json::Value = serde_json::from_str(&data)
        .context("Failed to parse MCPropertyEncyclopedia JSON")?;
    
    if !parsed.is_object() || parsed.get("key_list").is_none() {
        anyhow::bail!("MCPropertyEncyclopedia data format is invalid");
    }
    
    let key_list = parsed["key_list"].as_array()
        .context("key_list is not an array")?;
    
    println!("✓ Downloaded {} blocks from MCPropertyEncyclopedia", key_list.len());
    
    let output_file = output_dir.join("mcproperty_blocks.json");
    fs::write(&output_file, &data)
        .with_context(|| format!("Failed to write to {:?}", output_file))?;
    
    Ok(())
}
