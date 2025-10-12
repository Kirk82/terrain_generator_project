use image::{flat::SampleLayout, *};
use opensimplex_noise_rs::*;
use spatial2d::*;
fn main() {
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 { x: 1000, y: 1000 });

    let noise_generator = OpenSimplexNoise::new(None);

    for (my_value, my_position) in terrain_map.iter_with_pos_mut() {
        let noise_value = noise_generator.eval_2d(my_position.x.into(), my_position.y.into());

        
        for blue in noise_value
        
        for colour in noise_value {
            if noise_value % 10 == 0 {
                noise_value.blue()
            } else {
                if noise_value.range(0.2, 0.3) {
                    noise_value.yellow()
                } else {
                    if noise_value.range(0.4, 0.6) {
                        noise_value.green()
                    } else {
                        if noise_value.range(0.7, 0.9) {
                            noise_value.black()
                        } else {
                            if noise_value.range(1.0, 1.0) {
                                noise_value.green()
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Hello World");
}
