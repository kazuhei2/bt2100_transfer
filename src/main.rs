use std::f64;

mod bt2100tf;

fn main() {
    println!("{}", bt2100tf::hlg::oetf(0.5));
    println!("{}", bt2100tf::hlg::inverse_oetf(0.5));

    let mut rgb: Vec<f64> = Vec::new();
    rgb.push(0.12);
    rgb.push(0.34);
    rgb.push(0.56);

    println!("Before: rgb is {:?}", rgb);
    bt2100tf::hlg::ootf(&mut rgb);
    println!("After: rgb is {:?}", rgb);
}
