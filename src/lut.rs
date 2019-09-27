use std::f64;

use crate::bt2100tf::hlg;

pub struct Lut {
    pub in_bit_wid: usize,
    pub out_bit_wid: usize,
    pub grid_num: usize,
}

impl Lut {
    pub fn create_1d_sample(&self) -> Vec<f64> {
        let max = (1 << self.in_bit_wid) - 1;
        let interval = (max + 1) / (self.grid_num - 1);
        let mut ret: Vec<f64> =
            (0..max + 1).step_by(interval).map(|i| i as f64).collect();
        ret.push(max as f64);
        ret
    }

    pub fn create_3d_sample(&self) -> Vec<Vec<f64>> {
        let sample = self.create_1d_sample();
        let grid_num = self.grid_num;
        let mut ret = vec![vec![0.0_f64; 3]; grid_num.pow(3)];
        for i in (0..grid_num) {
            for j in (0..grid_num) {
                for k in (0..grid_num) {
                    ret[i*grid_num.pow(2) + j*grid_num + k][0] = sample[i];
                    ret[i*grid_num.pow(2) + j*grid_num + k][1] = sample[j];
                    ret[i*grid_num.pow(2) + j*grid_num + k][2] = sample[k];
                }
            }
        }
        ret
    }

    pub fn hlg_oetf(&self) {
        let max = (self.grid_num - 1) as f64;
        for i in 0..1 << self.in_bit_wid {
            println!("{},->,{}", i, hlg::oetf(i as f64 / max) * max);
        }
    }
}

pub struct LutBuilder {
    pub in_bit_wid: usize,
    pub out_bit_wid: usize,
    pub grid_num: usize,
}

impl LutBuilder {
    pub fn new() -> LutBuilder {
        LutBuilder {
            in_bit_wid: 10,
            out_bit_wid: 10,
            grid_num: 17,
        }
    }

    pub fn in_bit_wid(&mut self, coordinate: usize) -> &mut LutBuilder {
        self.in_bit_wid = coordinate;
        self
    }

    pub fn out_bit_wid(&mut self, coordinate: usize) -> &mut LutBuilder {
        self.out_bit_wid = coordinate;
        self
    }

    pub fn grid_num(&mut self, coordinate: usize) -> &mut LutBuilder {
        self.grid_num = coordinate;
        self
    }

    pub fn finalize(&self) -> Lut {
        Lut {
            in_bit_wid: self.in_bit_wid,
            out_bit_wid: self.out_bit_wid,
            grid_num: self.grid_num,
        }
    }
}
