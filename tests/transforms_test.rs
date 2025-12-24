use blockpedia::*;

#[test]
fn test_direction_rotation() {
    // Test basic direction rotation
    assert_eq!(Direction::North.rotate_clockwise(), Direction::East);
    assert_eq!(Direction::East.rotate_clockwise(), Direction::South);
    assert_eq!(Direction::South.rotate_clockwise(), Direction::West);
    assert_eq!(Direction::West.rotate_clockwise(), Direction::North);

    // Test that Up/Down don't change with horizontal rotation
    assert_eq!(Direction::Up.rotate_clockwise(), Direction::Up);
    assert_eq!(Direction::Down.rotate_clockwise(), Direction::Down);
}

#[test]
fn test_direction_apply_rotation() {
    let north = Direction::North;

    assert_eq!(north.apply_rotation(Rotation::None), Direction::North);
    assert_eq!(north.apply_rotation(Rotation::Clockwise90), Direction::East);
    assert_eq!(north.apply_rotation(Rotation::Half), Direction::South);
    assert_eq!(
        north.apply_rotation(Rotation::Clockwise270),
        Direction::West
    );
}

#[test]
fn test_direction_opposite() {
    assert_eq!(Direction::North.opposite(), Direction::South);
    assert_eq!(Direction::South.opposite(), Direction::North);
    assert_eq!(Direction::East.opposite(), Direction::West);
    assert_eq!(Direction::West.opposite(), Direction::East);
    assert_eq!(Direction::Up.opposite(), Direction::Down);
    assert_eq!(Direction::Down.opposite(), Direction::Up);
}

#[test]
fn test_rotate_simple_block() -> Result<()> {
    // Create a repeater with specific properties
    let repeater =
        BlockState::parse("minecraft:repeater[delay=2,facing=north,locked=true,powered=true]")?;

    // Rotate it 90 degrees clockwise
    let rotated = repeater.rotate_clockwise()?;

    println!("Original: {}", repeater);
    println!("Rotated:  {}", rotated);

    // The facing should change from north to east
    assert!(rotated.to_string().contains("facing=east"));

    // Other properties should remain the same
    assert!(rotated.to_string().contains("delay=2"));
    assert!(rotated.to_string().contains("locked=true"));
    assert!(rotated.to_string().contains("powered=true"));

    Ok(())
}

#[test]
fn test_rotate_180_degrees() -> Result<()> {
    let stairs =
        BlockState::parse("minecraft:oak_stairs[facing=north,half=bottom,shape=straight]")?;
    let rotated = stairs.rotate_180()?;

    println!("Original 180: {}", stairs);
    println!("Rotated 180:  {}", rotated);

    // Should face south after 180 degree rotation
    assert!(rotated.to_string().contains("facing=south"));

    Ok(())
}

#[test]
fn test_rotate_full_circle() -> Result<()> {
    let original =
        BlockState::parse("minecraft:oak_stairs[facing=north,half=bottom,shape=straight]")?;

    let step1 = original.rotate_clockwise()?; // Should be east
    let step2 = step1.rotate_clockwise()?; // Should be south
    let step3 = step2.rotate_clockwise()?; // Should be west
    let step4 = step3.rotate_clockwise()?; // Should be north again

    println!("Original: {}", original);
    println!("Step 1:   {}", step1);
    println!("Step 2:   {}", step2);
    println!("Step 3:   {}", step3);
    println!("Step 4:   {}", step4);

    // After a full rotation, should be back to original
    assert_eq!(original.to_string(), step4.to_string());

    Ok(())
}

#[test]
fn test_material_variant() -> Result<()> {
    // Start with oak stairs
    let oak_stairs =
        BlockState::parse("minecraft:oak_stairs[facing=north,half=bottom,shape=straight]")?;

    // Convert to stone stairs
    let stone_stairs = oak_stairs.with_material("stone")?;

    println!("Oak stairs:   {}", oak_stairs);
    println!("Stone stairs: {}", stone_stairs);

    // Should be stone_stairs now
    assert!(stone_stairs.to_string().contains("minecraft:stone_stairs"));

    // Properties should be preserved where possible
    assert!(stone_stairs.to_string().contains("facing=north"));
    assert!(stone_stairs.to_string().contains("half=bottom"));
    assert!(stone_stairs.to_string().contains("shape=straight"));

    Ok(())
}

#[test]
fn test_shape_variant() -> Result<()> {
    // Start with stone block
    let stone = BlockState::new("minecraft:stone")?;

    // Convert to stone stairs
    let stone_stairs = stone.with_shape(BlockShape::Stairs)?;

    println!("Stone block:  {}", stone);
    println!("Stone stairs: {}", stone_stairs);

    // Should be stone_stairs now
    assert!(stone_stairs.to_string().contains("minecraft:stone_stairs"));

    // Should have default stair properties
    assert!(stone_stairs.to_string().contains("facing=north"));
    assert!(stone_stairs.to_string().contains("half=bottom"));
    assert!(stone_stairs.to_string().contains("shape=straight"));

    Ok(())
}

#[test]
fn test_shape_variant_slab() -> Result<()> {
    // Start with stone block
    let stone = BlockState::new("minecraft:stone")?;

    // Convert to stone slab
    let stone_slab = stone.with_shape(BlockShape::Slab)?;

    println!("Stone block: {}", stone);
    println!("Stone slab:  {}", stone_slab);

    // Should be stone_slab now
    assert!(stone_slab.to_string().contains("minecraft:stone_slab"));

    Ok(())
}

#[test]
fn test_nonexistent_variant() {
    // Try to convert oak stairs to a material that doesn't have stairs
    let oak_stairs = BlockState::parse("minecraft:oak_stairs[facing=north]").unwrap();

    // This should fail because there's no "diamond_stairs"
    let result = oak_stairs.with_material("diamond");
    assert!(result.is_err());

    println!("Expected error: {:?}", result.unwrap_err());
}

#[test]
fn test_available_materials() -> Result<()> {
    // Get available materials for stairs
    let oak_stairs = BlockState::parse("minecraft:oak_stairs[facing=north]")?;
    let materials = oak_stairs.available_materials()?;

    println!("Available stair materials: {:?}", materials);

    // Should include common stair materials
    assert!(materials.contains(&"oak".to_string()));
    assert!(materials.contains(&"stone".to_string()));
    assert!(materials.len() > 5); // Should have many stair variants

    Ok(())
}

#[test]
fn test_available_shapes() -> Result<()> {
    // Get available shapes for oak (now using oak_planks with fixed material extraction)
    let oak_planks = BlockState::new("minecraft:oak_planks")?;
    let shapes = oak_planks.available_shapes()?;

    println!("Available oak shapes: {:?}", shapes);

    // Should include various oak variants
    assert!(shapes.contains(&BlockShape::Full));
    assert!(shapes.contains(&BlockShape::Stairs));
    assert!(shapes.contains(&BlockShape::Slab));
    assert!(shapes.len() > 3); // Should have multiple variants

    Ok(())
}

#[test]
fn test_complex_rotation_with_material_change() -> Result<()> {
    // Start with a complex block state
    let original = BlockState::parse(
        "minecraft:oak_stairs[facing=west,half=top,shape=inner_left,waterlogged=false]",
    )?;

    // Rotate and change material
    let rotated = original.rotate_clockwise()?;
    let stone_variant = rotated.with_material("stone")?;

    println!("Original: {}", original);
    println!("Rotated:  {}", rotated);
    println!("Stone:    {}", stone_variant);

    // Check that rotation worked (west -> north)
    assert!(rotated.to_string().contains("facing=north"));

    // Check that material change worked
    assert!(stone_variant.to_string().contains("minecraft:stone_stairs"));

    // Check that other properties are preserved
    assert!(stone_variant.to_string().contains("half=top"));

    Ok(())
}

#[test]
fn test_axis_rotation() -> Result<()> {
    // Test rotation of blocks with axis property (like logs)
    let log = BlockState::parse("minecraft:oak_log[axis=x]")?;
    let rotated = log.rotate_clockwise()?;

    println!("Original log: {}", log);
    println!("Rotated log:  {}", rotated);

    // X axis should rotate to Z axis
    assert!(rotated.to_string().contains("axis=z"));

    Ok(())
}

#[test]
fn test_stair_shape_rotation() -> Result<()> {
    // Test that stair shapes rotate correctly
    let stairs =
        BlockState::parse("minecraft:oak_stairs[facing=north,half=bottom,shape=inner_left]")?;
    let rotated = stairs.rotate_clockwise()?;

    println!("Original stairs: {}", stairs);
    println!("Rotated stairs:  {}", rotated);

    // Should rotate to east and inner_left should become inner_right
    assert!(rotated.to_string().contains("facing=east"));
    assert!(rotated.to_string().contains("shape=inner_right"));

    Ok(())
}

#[test]
fn test_error_handling() {
    // Test parsing invalid block state
    let result = BlockState::parse("minecraft:invalid_block[facing=north]");
    assert!(result.is_err());

    // Test rotating invalid block state
    let valid_block = BlockState::new("minecraft:stone").unwrap();
    // This should work fine
    assert!(valid_block.rotate_clockwise().is_ok());
}

#[test]
fn test_material_and_shape_extraction() -> Result<()> {
    // Test various block types to ensure proper material/shape extraction
    let test_cases = vec![
        ("minecraft:oak_stairs", "oak", BlockShape::Stairs),
        ("minecraft:stone_slab", "stone", BlockShape::Slab),
        (
            "minecraft:cobblestone_wall",
            "cobblestone",
            BlockShape::Wall,
        ),
        ("minecraft:oak_fence", "oak", BlockShape::Fence),
        ("minecraft:oak_fence_gate", "oak", BlockShape::FenceGate),
        ("minecraft:oak_door", "oak", BlockShape::Door),
        ("minecraft:oak_trapdoor", "oak", BlockShape::Trapdoor),
        ("minecraft:stone_button", "stone", BlockShape::Button),
        (
            "minecraft:stone_pressure_plate",
            "stone",
            BlockShape::PressurePlate,
        ),
        ("minecraft:stone", "stone", BlockShape::Full),
    ];

    for (block_id, expected_material, expected_shape) in test_cases {
        if let Some(_block) = get_block(block_id) {
            let state = BlockState::new(block_id)?;

            // Test shape variant (if the target exists)
            if let Ok(variants) = state.available_shapes() {
                println!(
                    "{} has {} shape variants: {:?}",
                    block_id,
                    variants.len(),
                    variants
                );
                assert!(variants.contains(&expected_shape));
            }

            // Test material variant (if the target exists)
            if let Ok(materials) = state.available_materials() {
                println!(
                    "{} has {} material variants: {:?}",
                    block_id,
                    materials.len(),
                    materials
                );
                if materials.len() > 1 {
                    assert!(materials.contains(&expected_material.to_string()));
                }
            }
        }
    }

    Ok(())
}

#[test]
fn test_convenience_methods() -> Result<()> {
    let repeater =
        BlockState::parse("minecraft:repeater[delay=2,facing=north,locked=true,powered=true]")?;

    // Test all convenience rotation methods
    let cw = repeater.rotate_clockwise()?;
    let half = repeater.rotate_180()?;
    let ccw = repeater.rotate_counter_clockwise()?;

    println!("Original: {}", repeater);
    println!("CW:       {}", cw);
    println!("180:      {}", half);
    println!("CCW:      {}", ccw);

    // Verify the rotations are correct
    assert!(cw.to_string().contains("facing=east"));
    assert!(half.to_string().contains("facing=south"));
    assert!(ccw.to_string().contains("facing=west"));

    Ok(())
}
