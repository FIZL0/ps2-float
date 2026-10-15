//! `libm::sce`, the SDK build's flavour: close to the true functions, identical to the generic code where the
//! two builds share it, and different from it (by an ulp) exactly where they don't.
//! Bit-exactness against the game that links it is proven in that game's repository with `ps2-oracle`.
#![allow(clippy::float_arithmetic)] // host references

use ps2_float::libm::{self, sce};

#[test]
fn close_to_the_true_functions() {
    for i in 1..=1000 {
        let x = i as f32 / 100.0;
        assert!((sce::powf(x, 1.1) / x.powf(1.1) - 1.0).abs() < 1e-6, "powf({x}, 1.1)");
        let c = i as f32 / 1000.0 * 2.0 - 1.0;
        assert!((sce::acosf(c) - c.acos()).abs() < 2e-6, "acosf({c})");
        let (y, x) = ((i as f32 * 0.37).sin(), (i as f32 * 0.37).cos());
        assert!((sce::atan2f(y, x) - y.atan2(x)).abs() < 2e-6, "atan2f({y}, {x})");
    }
}

#[test]
fn shares_the_generic_kernels_and_differs_where_the_builds_do() {
    let mut differ = [0u32; 3];
    for i in 0..20_000u32 {
        let x = f32::from_bits(0x3c00_0000 + i * 2_936); // 2^-7 .. 1
        assert_eq!(sce::sinf(x).to_bits(), libm::sinf(x).to_bits());
        assert_eq!(sce::atanf(x).to_bits(), libm::atanf(x).to_bits());
        differ[0] += (sce::acosf(-x) != libm::acosf(-x)) as u32;
        differ[1] += (sce::powf(x + 0.5, 1.7) != libm::powf(x + 0.5, 1.7)) as u32;
        differ[2] += (sce::atan2f(-x, -0.75) != libm::atan2f(-x, -0.75)) as u32;
    }
    assert!(differ.iter().all(|&n| n > 0), "the two builds never disagreed: {differ:?}");
}

#[test]
fn edge_cases_of_this_build() {
    assert_eq!(sce::powf(4.0, 0.5), 2.0, "a square root, by sqrtf");
    assert_eq!(sce::powf(-2.0, 0.5), f32::MAX, "0/0 on the EE: +max");
    assert_eq!(sce::powf(2.0, 0.0), 1.0);
    assert_eq!(sce::acosf(1.0), 0.0);
    assert_eq!(sce::acosf(-1.0), f32::from_bits(0x4049_0fda), "π, one ulp low in this build");
    assert_eq!(sce::atan2f(0.0, -1.0), f32::from_bits(0x4049_0fda));
    assert_eq!(sce::atan2f(1.0, 0.0), f32::from_bits(0x3fc9_0fda));
}
