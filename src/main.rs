use std::f64;

/// There are 3 constants in BT.2100 TABLE 5
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 1.0 - 4.0 * HLG_A;
// HLG_C = 0.5 - HLG_A * (4.0 * HLG_A).ln()
const HLG_C: f64 = 0.5599107;

fn main() {
    println!("{}", hlg_oetf(0.5));
}

fn hlg_oetf(color: f64) -> f64 {
    if color <= 1.0 / 12.0 {
        (color * 3.0).sqrt()
    } else {
        HLG_A * (12.0 * color - HLG_B).ln() + HLG_C
    }
}
