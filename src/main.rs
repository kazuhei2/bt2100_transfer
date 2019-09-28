use std::f64;

mod bt2100tf;
use bt2100tf::hlg::*;

mod lut;

fn main() {
    println!("{}", oetf(0.5));
    println!("{}", inverse_oetf(0.5));

    let mut rgb: Vec<f64> = Vec::new();
    rgb.push(0.12);
    rgb.push(0.34);
    rgb.push(0.56);

    println!("Before: rgb is {:?}", rgb);

    let display_prop = DisplayProp::new(1.2, 1000, 5);
    display_prop.ootf(&mut rgb);

    println!("After: rgb is {:?}", rgb);

    let lut = lut::LutBuilder::new().grid_num(33).finalize();

    let mut sample_1d = lut.create_1d_sample();
    println!("created:");
    println!("{:?}", sample_1d);
    lut.normalize(&mut sample_1d);
    println!("normalized:");
    println!("{:?}", sample_1d);
    lut.hlg_oetf(&mut sample_1d);
    println!("oetf output:");
    println!("{:?}", sample_1d);
    lut.unnormalize(&mut sample_1d);
    println!("unnormalized:");
    println!("{:?}", sample_1d);

    let sample_3d = lut.create_3d_sample();
    println!("{:?}", sample_3d);
}
