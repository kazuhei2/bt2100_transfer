mod bt2100tf;
use bt2100tf::hlg;
use bt2100tf::hlg::DisplayProp;

mod lut;

fn main() {
    let display_prop = DisplayProp::new(1.2, 1000, 5);

    let lut = lut::LutBuilder::new().grid_num(33).finalize();

    let mut sample_1d = lut.create_1d_sample();
    let orig_1d = lut.create_1d_sample();

    lut.normalize(&mut sample_1d);
    hlg::oetf_all(&mut sample_1d);
    lut.unnormalize(&mut sample_1d);

    for i in 0..sample_1d.len() {
        println!("{},->,{}", orig_1d[i], sample_1d[i])
    }

    let mut sample_3d = lut.create_3d_sample();
    let orig_3d = lut.create_3d_sample();
    for mut c in &mut sample_3d {
        lut.normalize(&mut c);
    }
    display_prop.ootf_all(&mut sample_3d);
    for i in 0..sample_3d.len() {
        println!("{},{},{},->,{},{},{}", orig_3d[i][0], orig_3d[i][1],
            orig_3d[i][2], sample_3d[i][0], sample_3d[i][1], sample_3d[i][2])
    }
}
