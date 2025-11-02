use image::*;
use opensimplex_noise_rs::*;
use rng::*;
use spatial2d::*;

fn main() {
    //setting the size of the y and x axis
    let size = UVec2::new(1000, 1000);

    let rng = Rng::new();

    let extra_distance = rng.gen_bool(0.5);

    //creating a matrix based on x and y variable sizes
    let mut terrain_map: Matrix<f32> = Matrix::new(UVec2 {
        x: size.x,
        y: size.y,
    });

    //creating a an image buffer based on the x and y variable sizes
    let mut image_buffer = RgbImage::new(size.x, size.y);

    //creating a variable for the Rng crate
    let rng_generator = Rng::new();

    //creating a variable to generate a random for the noise generator
    let random_seed: i64 = rng_generator.gen_value();

    //creating a random noise generator variable to be used in the loop
    let noise_generator = OpenSimplexNoise::new(Some(random_seed));

    let matrix_centre: UVec2 = (UVec2 {
        x: (size.x),
        y: (size.y),
    }) / 2;

    let max_distance_x = size.x / 2;

    let max_distance_y = size.y / 2;

    if extra_distance == true {
        max_distance_x * 2;
    } else if extra_distance == false {
        max_distance_y * 2;
    }

    let max_distance = UVec2::new(max_distance_x, max_distance_y);

    //looping through the matrix and assigning noise values to each value in the matrix
    for (value, position) in terrain_map.iter_with_pos_mut() {
        let first_noise_value: f64 =
            noise_generator.eval_2d((position.x as f64) / 10.0, (position.y as f64) / 10.0);

        let second_noise_value =
            noise_generator.eval_2d((position.x as f64) / 30.0, (position.y as f64) / 30.0);

        let third_noise_value =
            noise_generator.eval_2d((position.x as f64) / 10.0, (position.y as f64) / 10.0);

        let distance = matrix_centre.distance_euclidian(position);

        let normalised_x_distance = (distance / max_distance_x as f32).min(1.0);

        let normalised_y_distance = (distance / max_distance_y as f32).min(1.0);

        let inverted_distance;

        if extra_distance == true {
            let inverted_distance = 1.0 - normalised_x_distance;
        } else if extra_distance == false {
            let inverter_distance = 1.0 - normalised_y_distance;
        }

        let first_value = (((first_noise_value as f32) + 1.0) / 2.0) * inverted_distance;
        let second_value = (((second_noise_value as f32) + 1.0) / 2.0) * inverted_distance;
        let third_value = (((third_noise_value as f32) + 1.0) / 2.0) * inverted_distance;

        *value = first_value * second_value * third_value;

        let square_root = value.sqrt();

        *value = lerp(*value, square_root, inverted_distance);
    }

    //looping through the matrix again and assigning different colours to different values of the noise map and then assigning each pixel to the image buffer
    for (height, position) in terrain_map.iter_with_pos() {
        let pixel = get_colour(*height);

        image_buffer.put_pixel(position.x, position.y, pixel);
    }

    //defining the path the image will be saved to
    let path = "terrain.png";

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
        if height <= 0.2 {
            Rgb([190, 190, 0])
        } else {
            if height <= 0.4 {
                Rgb([230, 230, 0])
            } else {
                if height <= 0.5 {
                    Rgb([0, 160, 0])
                } else {
                    if height <= 0.8 {
                        Rgb([150, 75, 0])
                    } else {
                        if height <= 1.0 {
                            Rgb([255, 255, 255])
                        } else {
                            if height >= 1.0 {}
                            panic!()
                        }
                    }
                }
            }
        }
    }
}

fn lerp(from: f32, to: f32, s: f32) -> f32 {
    from + (to - from) * s
}
