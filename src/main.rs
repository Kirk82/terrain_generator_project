use image::*;
use opensimplex_noise_rs::*;
use rng::*;
use spatial2d::*;

fn main() {
    //setting the size of the y and x axis
    let map_size = UVec2::new(1000, 1000);

    //creating a matrix based on map_size
    let mut terrain_map: Matrix<f32> = Matrix::new(map_size);

    //creating a an image buffer based on the x and y variable sizes
    let mut image_buffer = RgbImage::new(map_size.x, map_size.y);

    //creating a variable for the Rng crate
    let rng = Rng::new();

    //creating a variable to generate a random for the noise generator
    let random_seed: i64 = rng.gen_value();

    //creating a random noise generator variable to be used in the loop
    let noise_generator = OpenSimplexNoise::new(Some(random_seed));

    let matrix_centre = map_size / 2;

    let max_distance = 500.0 * (2.0_f32.sqrt());

    //looping through the matrix and assigning noise values to each value in the matrix
    for (height, position) in terrain_map.iter_with_pos_mut() {
        let noise_value =
            noise_generator.eval_2d((position.x as f64) / 40.0, (position.y as f64) / 40.0) as f32;

        let distance = matrix_centre.distance_euclidian(position);

        let normalised_distance = distance / max_distance;

        let inverted_distance = 1.0 - normalised_distance;

        *height = ((noise_value + 1.0) / 2.0) * inverted_distance;
    }

    //looping through the matrix again and assigning different colours to different values of the noise map
    //then assigning each pixel to the image buffer
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
    } else if height <= 0.15 {
        Rgb([255, 255, 0])
    } else if height <= 0.5 {
        Rgb([0, 255, 0])
    } else if height <= 0.7 {
        Rgb([150, 75, 0])
    } else if height <= 1.0 {
        Rgb([255, 255, 255])
    } else {
        panic!("Height should not be greater than 1.0")
    }
}
