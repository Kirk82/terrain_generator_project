use image::*;
use opensimplex_noise_rs::*;
use spatial2d::*;

fn main() {
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 { x: 1000, y: 1000 });

    let noise_generator = OpenSimplexNoise::new(None);

    for (value, position) in terrain_map.iter_with_pos_mut() {
        let noise_value = noise_generator.eval_2d(position.x.into(), position.y.into());
        *value = noise_value as f32;
    }

    fn get_colour() -> Rgb<f32> {
        let mut height: f32 = 0.0;
        if height <= 0.1 {
            Rgb([0.0, 0.0, 1.0])
        } else {
            if height <= 0.3 {
                Rgb([1.0, 1.0, 0.0])
            } else {
                if height <= 0.4 {
                    Rgb([1.0, 1.0, 1.0])
                } else {
                    None
                }
            }
        }
    };

    println!("Hello World");
}
