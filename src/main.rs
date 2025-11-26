use image::*;
use opensimplex_noise_rs::*;
use rng::*;
use spatial2d::*;

fn main() {
    //creating a variable for the Rng crate
    let size = 1000;
    let scale: f32 = 160.0;
    let layer_count: u32 = 8;
    let island_count: u32 = 6;
    let island_size = 500.0;

    //setting the size of the y and x axis
    let grid_size = UVec2::splat(size);

    //creating a matrix based on x and y variable sizes
    let mut terrain_map: Matrix<f32> = Matrix::new(grid_size);

    //creating a an image buffer based on the x and y variable sizes
    let mut image_buffer = RgbImage::new(grid_size.x, grid_size.y);

    //creating a variable for the Rng crate
    let rng = Rng::new();

    //creating a variable to generate a random for the noise generator
    let random_seed: i64 = rng.gen_value();

    //creating a random noise generator variable to be used in the loop
    let noise_generator = OpenSimplexNoise::new(Some(random_seed));

    let max_distance: f32 = island_size;

    let generated_island_centres = get_island_centres(island_count, size);

    //looping through the matrix and assigning noise values to each value in the matrix
    for (height, position) in terrain_map.iter_with_pos_mut() {
        // assert!(normalised_distance >= 0.0 && normalised_distance <= 1.0);

        let noise_value = layered_noise(&noise_generator, position, scale, layer_count);
        let inverted_distance =
            island_gradient(position, generated_island_centres.clone(), max_distance);

        *height = noise_value * inverted_distance;

        // *height = (1.0 - (distance / max_distance).min(1.0));

        // assert!(!height.is_nan());

        // assert!(
        //     *height >= 0.0 && *height <= 1.0,
        //     "height: {}, pos: {}, inv_dist: {}",
        //     *height,
        //     position,
        //     inverted_distance
        // );

        // if position == UVec2::new(0, 0) {
        //     dbg!(inverted_distance, square_root, &height);
        // }

        // if position == UVec2::new(500, 500) {
        //     dbg!(inverted_distance, square_root, &height);
        //     panic!()
        // }
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

    // println!(
    // "max distance {} \n centre point {}",
    // max_distance, matrix_centre
    // );
    println!("island centre {}", island_centre);
}

fn get_colour(height: f32) -> Rgb<u8> {
    if height <= 0.2 {
        Rgb([0, 0, 255]) // Water
    } else if height <= 0.22 {
        Rgb([190, 190, 0]) // Sand
    } else if height <= 0.3 {
        Rgb([230, 230, 0]) // 
    } else if height <= 0.5 {
        Rgb([0, 160, 0]) // 
    } else if height <= 0.8 {
        Rgb([150, 75, 0]) // 
    } else if height <= 0.9 {
        Rgb([255, 255, 255]) // 
    } else {
        panic!()
    }
}

fn island_gradient(pos: UVec2, island_centres: Vec<UVec2>, max_distance: f32) -> f32 {
    let mut total_inverted_distance = 0.0;
    let island_count: usize = island_centres.len();

    for centre in island_centres {
        let distance = centre.distance_euclidian(pos);
        let normalised_distance = (distance / max_distance).min(1.0);
        let inverted_distance = 1.0 - normalised_distance;

        total_inverted_distance += inverted_distance;
    }
    // let grid_centre = grid_size / 2;
    // let distance = grid_centre.distance_euclidian(pos);
    // let normalised_distance = (distance / max_distance).min(1.0);
    // let inverted_distance = 1.0 - normalised_distance;

    return total_inverted_distance / island_count as f32;
}

fn get_island_centres(island_count: u32, range_size: u32) -> Vec<UVec2> {
    let mut island_centres: Vec<UVec2> = Vec::new();
    let rng = Rng::new();

    for _ in 0..island_count {
        let island_centre = UVec2::new(
            rng.gen_range(range_size - 800..range_size - 200),
            rng.gen_range(range_size - 800..range_size - 200),
        );
        island_centres.push(island_centre);
    }

    return island_centres;
}
// println!("island centre {}", island_centre);

fn layered_noise(
    noise_generator: &OpenSimplexNoise,
    pos: UVec2,
    scale: f32,
    layer_count: u32,
) -> f32 {
    let mut total_noise_value = 0.0;
    let mut amp = 1.0;
    let mut noise_scale = scale;
    let mut total_amp = 0.0;

    for _ in 0..layer_count {
        let scaled_pos = pos.as_vec2() / noise_scale;
        total_noise_value +=
            noise_generator.eval_2d(scaled_pos.x as f64, scaled_pos.y as f64) as f32 * amp;

        total_amp += amp;
        amp /= 2.0;
        noise_scale /= 2.0;
    }

    // ((total_noise_value + 1.0) / 2.0) / total_amp
    ((total_noise_value / total_amp) + 1.0) / 2.0
}
