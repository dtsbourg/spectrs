extern crate image;

use std::path::Path;
use image::GenericImage;

fn main() {
    let img = image::open(Path::new("image.jpeg")).unwrap();
    println!("dimensions {:?}", img.dimensions());
}


