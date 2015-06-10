extern crate image;

use std::path::Path;
use image::GenericImage;
use image::Pixel;
use std::fs::File;
use std::io::Write;

fn main() {
    let img = image::open(Path::new("image.jpeg")).unwrap();
    let (x,y) = img.dimensions();
    let mut intensities : Vec<Vec<f32>> = Vec::with_capacity(y as usize);

    for (xp, yp, pixel) in img.pixels() {
        let data = pixel.to_rgb().data;
        let rgb_intensity : u16 = data[0] as u16 + data[1] as u16 + data[2] as u16;
        let norm_intensity : f32 = rgb_intensity as f32 / 255.0;
        if xp == 0 {
            intensities.push(Vec::new());
        }
        intensities[yp as usize].push(norm_intensity);
    }

    let path = Path::new("test.raw");
    let mut f = match File::create(path) {
        Err(..) => panic!("Couldn't create file"),
        Ok(f) => f,
    };

    for col in intensities {
        let pcm_buf = column_to_pcm(col, x as usize, 48000);
        for val in pcm_buf {
            let n: [u8; 4] = unsafe { std::mem::transmute(val) };
            match f.write_all(&n) {
                Err(..) => panic!("Couldn't write to file"),
                _ => (),
            };
        }
    }
}

fn column_to_pcm(col_intensities : Vec<f32>, y : usize, sample_rate : u32) -> Vec<f32> {
    let pi = std::f32::consts::PI;
    let top_f = sample_rate as f32 / 2.0;
    let y_slice : f32 = top_f / (y as f32);
    let column_width : usize = (sample_rate as f32 / 25.0) as usize;
    let mut buf_out : Vec<f32> = Vec::new();
    let mut sample : f32 = 0.0;

    for i in 0..column_width {
        let alpha : f32 = pi * (i as f32 / column_width as f32);
        let envelope_mult : f32 = alpha.sin();

        for j in 0..y {
            let f = top_f - (y_slice * j as f32);
            let omega : f32 = 2.0 * pi * f * (i as f32 / sample_rate as f32) * envelope_mult;
            sample += col_intensities[j] * omega.sin();
        }

        sample *= 0.95;
        buf_out.push(sample);
    }
    buf_out
}
