use blockpedia::{
    color::{
        palettes::{GradientMethod, PaletteGenerator},
        ExtendedColorData,
    },
    BLOCKS,
};

fn close(actual: [f32; 3], expected: [f32; 3], epsilon: f32) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!((a - b).abs() < epsilon, "{actual:?} != {expected:?}");
    }
}

#[test]
fn srgb_primaries_match_reference_oklab_coordinates() {
    // Björn Ottosson's D65/linear-sRGB matrices (2021 revision).
    // https://bottosson.github.io/posts/oklab/
    for (rgb, expected) in [
        ([255, 0, 0], [0.6279554, 0.2248631, 0.1258463]),
        ([0, 255, 0], [0.8664396, -0.2338876, 0.1794985]),
        ([0, 0, 255], [0.4520137, -0.0324570, -0.3115281]),
        ([128, 128, 128], [0.5998708, 0.0, 0.0]),
    ] {
        close(
            ExtendedColorData::from_rgb(rgb[0], rgb[1], rgb[2]).oklab,
            expected,
            0.00003,
        );
    }
}

#[test]
fn oklab_and_oklch_represent_the_same_color() {
    for rgb in [[200, 34, 77], [0, 255, 0], [25, 120, 230]] {
        let c = ExtendedColorData::from_rgb(rgb[0], rgb[1], rgb[2]);
        let hue = c.oklch[2].to_radians();
        close(
            c.oklab,
            [c.oklch[0], c.oklch[1] * hue.cos(), c.oklch[1] * hue.sin()],
            0.00001,
        );
    }
}

#[test]
fn oklab_gradient_preserves_saturated_endpoints() {
    let a = ExtendedColorData::from_rgb(0, 0, 255);
    let b = ExtendedColorData::from_rgb(255, 0, 0);
    let gradient =
        PaletteGenerator::generate_gradient_palette(a, b, 7, GradientMethod::LinearOklab);
    assert_eq!(gradient.first().unwrap().rgb, a.rgb);
    assert_eq!(gradient.last().unwrap().rgb, b.rgb);
}

#[test]
fn neutral_perceptual_midpoint_is_gamma_encoded() {
    let gradient = PaletteGenerator::generate_gradient_palette(
        ExtendedColorData::from_rgb(0, 0, 0),
        ExtendedColorData::from_rgb(255, 255, 255),
        3,
        GradientMethod::LinearOklab,
    );
    assert_eq!(gradient[1].rgb, [99, 99, 99]);
}

#[test]
fn cached_block_coordinates_agree_with_runtime_conversion() {
    let mut checked = 0;
    for block in BLOCKS.values() {
        if let Some(color) = block.extras.color {
            close(color.oklab, color.to_extended().oklab, 0.00001);
            checked += 1;
        }
    }
    assert!(
        checked > 100,
        "Expected the checked-in RGB cache to be used"
    );
}

#[test]
fn equal_color_gradients_round_trip_the_rgb_cube() {
    for r in (0..=255).step_by(51) {
        for g in (0..=255).step_by(51) {
            for b in (0..=255).step_by(51) {
                let color = ExtendedColorData::from_rgb(r, g, b);
                let gradient = PaletteGenerator::generate_gradient_palette(
                    color,
                    color,
                    3,
                    GradientMethod::LinearOklab,
                );
                assert!(gradient.iter().all(|c| c.rgb == color.rgb));
            }
        }
    }
}
