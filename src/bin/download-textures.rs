use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 Downloading Minecraft block textures from hueblocks...");

    // Create textures directory
    let textures_dir = Path::new("assets/textures");
    fs::create_dir_all(textures_dir).context("Failed to create textures directory")?;

    // Get list of available textures
    let client = reqwest::Client::new();
    let url =
        "https://api.github.com/repos/1280px/hueblocks/contents/data/blocksets/blocks?ref=legacy";

    println!("📋 Fetching texture list from GitHub API...");
    let response = client
        .get(url)
        .header("User-Agent", "blockpedia-texture-downloader")
        .send()
        .await
        .context("Failed to fetch texture list")?;

    let files: Vec<Value> = response
        .json()
        .await
        .context("Failed to parse GitHub API response")?;

    println!("Found {} texture files", files.len());

    let mut downloaded = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for file in files {
        if let (Some(name), Some(download_url)) =
            (file["name"].as_str(), file["download_url"].as_str())
        {
            if !name.ends_with(".png") {
                continue;
            }

            let local_path = textures_dir.join(name);

            // Skip if file already exists
            if local_path.exists() {
                skipped += 1;
                continue;
            }

            print!("📥 Downloading {}... ", name);

            match download_texture(&client, download_url, &local_path).await {
                Ok(_) => {
                    println!("✅");
                    downloaded += 1;
                }
                Err(e) => {
                    println!("❌ Failed: {}", e);
                    failed += 1;
                }
            }

            // Be nice to GitHub's API
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    println!();
    println!("📊 Download Summary:");
    println!("  Downloaded: {}", downloaded);
    println!("  Skipped (already exists): {}", skipped);
    println!("  Failed: {}", failed);
    println!("  Total files: {}", downloaded + skipped + failed);

    if failed > 0 {
        println!("⚠️  Some downloads failed. You can re-run this command to retry.");
    } else {
        println!("🎉 All textures downloaded successfully!");

        // Quick color extraction test
        println!();
        println!("🎨 Testing color extraction on a few textures...");
        test_color_extraction(&textures_dir).await?;
    }

    Ok(())
}

async fn download_texture(client: &reqwest::Client, url: &str, local_path: &Path) -> Result<()> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to download texture")?;

    let bytes = response
        .bytes()
        .await
        .context("Failed to read texture bytes")?;

    fs::write(local_path, bytes).context("Failed to write texture file")?;

    Ok(())
}

async fn test_color_extraction(textures_dir: &Path) -> Result<()> {
    use blockpedia::color::extract_dominant_color;

    // Test a few common blocks
    let test_blocks = [
        "stone.png",
        "dirt.png",
        "grass_block_top.png",
        "oak_log.png",
    ];

    for block_name in test_blocks {
        let texture_path = textures_dir.join(block_name);
        if texture_path.exists() {
            match extract_dominant_color(&texture_path) {
                Ok(color) => {
                    println!(
                        "  {} -> RGB({}, {}, {}) | {} | HSL({:.0}°, {:.1}%, {:.1}%)",
                        block_name,
                        color.rgb[0],
                        color.rgb[1],
                        color.rgb[2],
                        color.hex_string(),
                        color.hsl[0],
                        color.hsl[1] * 100.0,
                        color.hsl[2] * 100.0
                    );
                }
                Err(e) => {
                    println!("  {} -> Error: {}", block_name, e);
                }
            }
        }
    }

    Ok(())
}
