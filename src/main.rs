use std::path::Path;

use image::*;
use opensimplex_noise_rs::*;
use rng::*;
use spatial2d::*;

fn main() {
    //setting the size of the y and x axis
    let x_size: u32 = 1000;

    let y_size = 1000;

    //creating a matrix based on x and y variable sizes
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 {
        x: x_size,
        y: y_size,
    });

    //creating a an image buffer based on the x and y variable sizes
    let mut image_buffer = RgbImage::new(x_size, y_size);

    //creating a variable for the Rng crate
    let rng_generator = Rng::new();

    //creating a variable to generate a random for the noise generator
    let random_seed: i64 = rng_generator.gen_value();

    //creating a random noise generator variable to be used in the loop
    let noise_generator = OpenSimplexNoise::new(Some(random_seed));

    let matrix_centre: UVec2 = (UVec2 {
        x: (x_size),
        y: (y_size),
    }) / 2;

    let max_distance = 500.0 * (2.0_f32.sqrt());

    //looping through the matrix and assigning noise values to each value in the matrix
    for (value, position) in terrain_map.iter_with_pos_mut() {
        let noise_value: f64 =
            noise_generator.eval_2d((position.x as f64) / 40.0, (position.y as f64) / 40.0);

        let distance = matrix_centre.distance_euclidian(position);

        let normalised_distance = distance / max_distance;

        let inverted_distance = 1.0 - normalised_distance;

        *value = (((noise_value as f32) + 1.0) / 2.0) * inverted_distance;
    }

    //looping through the matrix again and assigning different colours to different values of the noise map and then assigning each pixel to the image buffer
    for (height, position) in terrain_map.iter_with_pos() {
        let pixel = get_colour(*height);

        image_buffer.put_pixel(position.x, position.y, pixel);
    }

    //defining the path the image will be saved to
    let path = Path::new("terrain.png");

    let image = DynamicImage::ImageRgb8(image_buffer);

    image
        .save(path)
        .unwrap_or_else(|e| panic!("Failed to save image at {path:#?}: {e}"));

    println!(
        "max distance {} \n centre point {}",
        max_distance, matrix_centre
    );
}

fn get_colour(height: f32) -> Rgb<u8> {
    if height <= 0.1 {
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
                    if height <= 1.0 {
                        Rgb([255, 255, 255])
                    } else {
                        if height >= 1.1 {}
                        panic!()
                    }
                }
            }
        }
    }
}
