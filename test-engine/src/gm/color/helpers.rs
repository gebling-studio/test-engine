pub(super) fn srgb_to_linear(u: f32) -> f32 {
    if u <= 0.0404_5 {
        u / 12.92
    } else {
        f32::powf((u + 0.055) / 1.055, 2.4)
    }
}

pub(super) fn _linear_to_srgb(u: f32) -> f32 {
    if u <= 0.003_130_8 {
        u * 12.92
    } else {
        1.055 * f32::powf(u, 1.0 / 2.4) - 0.055
    }
}

#[test]
fn srgb_to_linear_test() {
    for i in 0..255 {
        let val = i as f32 / 255.0;
        let converted = _linear_to_srgb(srgb_to_linear(val));

        let diff = (val - converted).abs();

        assert!(diff <= 0.000_000_06, "{val} - {converted} = {diff}");
    }
}
