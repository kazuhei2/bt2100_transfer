use std::f64;

use crate::bt2100tf::hlg;

pub struct Lut {
    pub in_bit_wid: u8,
    pub out_bit_wid: u8,
    pub grid_num: u16,
}

impl Lut {
    pub fn hlg_oetf(&self) {
        let max = (self.grid_num - 1) as f64;
        for i in 0..1 << self.in_bit_wid {
            println!("{},->,{}", i, hlg::oetf(i as f64 / max) * max);
        }
    }
}

pub struct LutBuilder {
    pub in_bit_wid: u8,
    pub out_bit_wid: u8,
    pub grid_num: u16,
}

impl LutBuilder {
    pub fn new() -> LutBuilder {
        LutBuilder {
            in_bit_wid: 10,
            out_bit_wid: 10,
            grid_num: 1024,
        }
    }

    pub fn in_bit_wid(&mut self, coordinate: u8) -> &mut LutBuilder {
        self.in_bit_wid = coordinate;
        self
    }

    pub fn out_bit_wid(&mut self, coordinate: u8) -> &mut LutBuilder {
        self.out_bit_wid = coordinate;
        self
    }

    pub fn grid_num(&mut self, coordinate: u16) -> &mut LutBuilder {
        self.grid_num = coordinate;
        self
    }

    pub fn finalize(&self) -> Lut {
        Lut {
            in_bit_wid: self.in_bit_wid,
            out_bit_wid: self.out_bit_wid,
            grid_num: self.grid_num
        }
    }
}
