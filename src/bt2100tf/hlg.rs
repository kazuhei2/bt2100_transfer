use std::f64;

/// There are 3 constants in BT.2100 TABLE 5
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 1.0 - 4.0 * HLG_A;
// HLG_C = 0.5 - HLG_A * (4.0 * HLG_A).ln()
const HLG_C: f64 = 0.5599107;

const Y_R_COEF: f64 = 0.2627;
const Y_G_COEF: f64 = 0.6780;
const Y_B_COEF: f64 = 0.0593;

pub struct DisplayProp {
    pub gamma: f64,
    pub peak_luminance: u16,
    pub black_luminance: u16,
}

impl DisplayProp {
    pub fn new(gamma: f64, peak: u16, black: u16) -> DisplayProp {
        DisplayProp {
            gamma: gamma,
            peak_luminance: peak,
            black_luminance: black
        }
    }

    pub fn ootf(&self, rgb: &mut Vec<f64>) {
        let y = Y_R_COEF * rgb[0] +
                Y_G_COEF * rgb[1] +
                Y_B_COEF * rgb[2];
        let y_pow = y.powf(self.gamma - 1.0);

        let lw = self.peak_luminance as f64;
        let lb = self.black_luminance as f64;

        for e in rgb {
            *e =  (lw - lb) * y_pow * *e + lb;
        }
    }
}

pub fn oetf(color: f64) -> f64 {
    if color <= 1.0 / 12.0 {
        (color * 3.0).sqrt()
    } else {
        HLG_A * (12.0 * color - HLG_B).ln() + HLG_C
    }
}

pub fn inverse_oetf(color: f64) -> f64 {
    if color <= 0.5 {
        color * color / 3.0
    } else {
        let tmp = (color - HLG_C) / HLG_A;
        (tmp.exp() + HLG_B) / 12.0
    }
}
