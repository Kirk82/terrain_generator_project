use image::*;
use opensimplex_noise_rs::*;
use spatial2d::*;

fn main() {
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 { x: 1000, y: 1000 });

    terrian_map.save("test_terrain.png").unwrap();

    let noise_generator = OpenSimplexNoise::new(None);

    for (value, position) in terrain_map.iter_with_pos_mut() {
        let noise_value = noise_generator.eval_2d(position.x.into(), position.y.into());
        *value = noise_value as f32;
    }

    for (height, position) in terrain_map.iter_with_pos() {
        let pixel = get_colour(*height);
        let mut image_buffer = Rgb32FImage::new(position.x, position.y);
        image_buffer.put_pixel(position.x, position.y, pixel);
    }
}

fn get_colour(height: f32) -> Rgb<f32> {
    if height <= 0.1 {
        Rgb([0.0, 0.0, 1.0])
    } else {
        if height <= 0.4 {
            Rgb([1.0, 1.0, 0.0])
        } else {
            if height <= 0.6 {
                Rgb([1.0, 1.0, 1.0])
            } else {
                if height <= 0.8 {
                    Rgb([1.0, 0.0, 0.0])
                } else {
                    if height <= 1.0 {
                        Rgb([0.0, 0.0, 0.0])
                    } else {
                        if height >= 0.9 {}
                        panic!()
                    }
                }
            }
        }
    }
}
