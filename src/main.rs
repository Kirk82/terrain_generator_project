

use image::{flat::SampleLayout, *};
use opensimplex_noise_rs::*;
use spatial2d::*;
use rng::*;

fn main() {
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 { x: 1000, y: 1000 });

    let noise_generator = OpenSimplexNoise::new(None);

    let rng_generator = Rng::new();

    let rgb = image::Rgb;

    for (my_value, my_position) in terrain_map.iter_with_pos_mut() {
        let noise_value = noise_generator.eval_2d(my_position.x.into(), my_position.y.into());
        let noise_value;
        my_value = noise_value;
    }

    fn get_colour() -> image::Rgb<>gb {
if my_value == rng_generator.gen_range(0.0..1.0), {
    rgb[0.3, 0, 0.3]}
};
    

    println!("Hello World");
}
