use std::f64;

mod bt2100tf;
use bt2100tf::hlg::*;

fn main() {
    println!("{}", oetf(0.5));
    println!("{}", inverse_oetf(0.5));

    let mut rgb: Vec<f64> = Vec::new();
    rgb.push(0.12);
    rgb.push(0.34);
    rgb.push(0.56);

    println!("Before: rgb is {:?}", rgb);
    ootf(&mut rgb);
    println!("After: rgb is {:?}", rgb);
}
