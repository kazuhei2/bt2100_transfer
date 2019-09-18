use std::f64;

use crate::bt2100tf::hlg::*;

pub fn print() {
    println!("hello from lut");
    println!("{}", oetf(0.5));
}
