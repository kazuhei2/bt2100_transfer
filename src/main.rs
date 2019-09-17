use std::f64;

/// There are 3 constants in BT.2100 TABLE 5
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 1.0 - 4.0 * HLG_A;
// HLG_C = 0.5 - HLG_A * (4.0 * HLG_A).ln()
const HLG_C: f64 = 0.5599107;

const Y_R_COEF: f64 = 0.2627;
const Y_G_COEF: f64 = 0.6780;
const Y_B_COEF: f64 = 0.0593;

struct DisplayProp {
    gamma: f64,
    peak_luminance: u16,
    black_luminance: u16,
}

static display_prop: DisplayProp = DisplayProp {
    gamma: 1.2,
    peak_luminance: 1000,
    black_luminance: 5,
};

fn main() {
    println!("{}", hlg_oetf(0.5));
    println!("{}", hlg_inverse_oetf(0.5));

    let mut rgb: Vec<f64> = Vec::new();
    rgb.push(0.12);
    rgb.push(0.34);
    rgb.push(0.56);

    println!("Before: rgb is {:?}", rgb);
    hlg_ootf(&mut rgb);
    println!("After: rgb is {:?}", rgb);
}

fn hlg_oetf(color: f64) -> f64 {
    if color <= 1.0 / 12.0 {
        (color * 3.0).sqrt()
    } else {
        HLG_A * (12.0 * color - HLG_B).ln() + HLG_C
    }
}

fn hlg_inverse_oetf(color: f64) -> f64 {
    if color <= 1.0 / 2.0 {
        color * color / 3.0
    } else {
        let tmp = (color - HLG_C) / HLG_A;
        (tmp.exp() + HLG_B) / 12.0
    }
}

fn hlg_ootf(rgb: &mut Vec<f64>) {
    let y = Y_R_COEF * rgb[0] +
            Y_G_COEF * rgb[1] +
            Y_B_COEF * rgb[2];
    let y_pow = y.powf(display_prop.gamma - 1.0);
    let lw = display_prop.peak_luminance as f64;
    let lb = display_prop.black_luminance as f64;
    for e in rgb {
        let e_before = *e;
        *e =  (lw - lb) * y_pow * *e + lb;
    }
}
