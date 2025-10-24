use std::path::Path;

use image::*;
use opensimplex_noise_rs::*;
use spatial2d::*;

fn main() {
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 { x: 1000, y: 1000 });

    let mut image_buffer = RgbImage::new(1000, 1000);

    let noise_generator = OpenSimplexNoise::new(None);

    for (value, position) in terrain_map.iter_with_pos_mut() {
        let noise_value: f64 =
            noise_generator.eval_2d((position.x as f64) / 40.0, (position.y as f64) / 40.0);
        *value = noise_value as f32;
    }

    for (height, position) in terrain_map.iter_with_pos() {
        let pixel = get_colour(*height);

        image_buffer.put_pixel(position.x, position.y, pixel);
    }

    let path = Path::new("terrain.png");

    let image = DynamicImage::ImageRgb8(image_buffer);

    image
        .save(path)
        .unwrap_or_else(|e| panic!("Failed to save image at {path:#?}: {e}"));
}

fn get_colour(height: f32) -> Rgb<u8> {
    if height <= 0.3 {
        Rgb([0, 0, 255])
    } else {
        if height <= 0.5 {
            Rgb([255, 255, 0])
        } else {
            if height <= 0.7 {
                Rgb([0, 255, 0])
            } else {
                if height <= 0.8 {
                    Rgb([150, 75, 0])
                } else {
                    if height <= 0.9 {
                        Rgb([255, 255, 255])
                    } else {
                        if height >= 0.9 {}
                        panic!()
                    }
                }
            }
        }
    }
}
