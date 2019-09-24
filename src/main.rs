use std::f64;

mod bt2100tf;
use bt2100tf::hlg::*;

mod lut;

const DISPLAY_PROP: DisplayProp = DisplayProp {
    gamma: 1.2,
    peak_luminance: 1000,
    black_luminance: 5,
};

fn main() {
    println!("{}", oetf(0.5));
    println!("{}", inverse_oetf(0.5));

    let mut rgb: Vec<f64> = Vec::new();
    rgb.push(0.12);
    rgb.push(0.34);
    rgb.push(0.56);

    println!("Before: rgb is {:?}", rgb);
    ootf(&DISPLAY_PROP, &mut rgb);
    println!("After: rgb is {:?}", rgb);

    let lut = lut::LutBuilder::new()
        .finalize();

    lut.hlg_oetf();
}
