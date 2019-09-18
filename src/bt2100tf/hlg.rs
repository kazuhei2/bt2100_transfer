use std::f64;

pub struct DisplayProp {
    pub gamma: f64,
    pub peak_luminance: u16,
    pub black_luminance: u16,
}

/// There are 3 constants in BT.2100 TABLE 5
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 1.0 - 4.0 * HLG_A;
// HLG_C = 0.5 - HLG_A * (4.0 * HLG_A).ln()
const HLG_C: f64 = 0.5599107;

const Y_R_COEF: f64 = 0.2627;
const Y_G_COEF: f64 = 0.6780;
const Y_B_COEF: f64 = 0.0593;

pub fn oetf(color: f64) -> f64 {
    if color <= 1.0 / 12.0 {
        (color * 3.0).sqrt()
    } else {
        HLG_A * (12.0 * color - HLG_B).ln() + HLG_C
    }
}

pub fn inverse_oetf(color: f64) -> f64 {
    if color <= 1.0 / 2.0 {
        color * color / 3.0
    } else {
        let tmp = (color - HLG_C) / HLG_A;
        (tmp.exp() + HLG_B) / 12.0
    }
}

pub fn ootf(display_prop: &DisplayProp, rgb: &mut Vec<f64>) {
    let y = Y_R_COEF * rgb[0] +
            Y_G_COEF * rgb[1] +
            Y_B_COEF * rgb[2];
    let y_pow = y.powf(display_prop.gamma - 1.0);

    let lw = display_prop.peak_luminance as f64;
    let lb = display_prop.black_luminance as f64;

    for e in rgb {
        *e =  (lw - lb) * y_pow * *e + lb;
    }
}
