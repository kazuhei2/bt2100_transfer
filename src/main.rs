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
    let orig_1d = lut.create_1d_sample();

    lut.normalize(&mut sample_1d);
    lut.hlg_oetf(&mut sample_1d);
    lut.unnormalize(&mut sample_1d);

    for i in 0..sample_1d.len() {
        println!("{},->,{}", orig_1d[i], sample_1d[i])
    }

    let mut sample_3d = lut.create_3d_sample();
    let orig_3d = lut.create_3d_sample();
    for mut c in &mut sample_3d {
        lut.normalize(&mut c);
    }
    lut.hlg_ootf(display_prop, &mut sample_3d);
    for i in 0..sample_3d.len() {
        println!("{},{},{},->,{},{},{}", orig_3d[i][0], orig_3d[i][1],
            orig_3d[i][2], sample_3d[i][0], sample_3d[i][1], sample_3d[i][2])
    }
}
