use image::*;
use opensimplex_noise_rs::*;
use rng::*;
use spatial2d::*;

fn main() {
    //creating a variable for the Rng crate
    let rng = Rng::new();

    let image_size = 2000;
    let scale: f32 = 160.0;
    let layer_count: u32 = 8;
    // let island_circumference = rng.gen_range(200..800) as f32;
    // let island_centre_x = rng.gen_range(500..1500);
    // let island_centre_y = rng.gen_range(500..1500);

    //setting the image_size of the y and x axis
    let grid_size = UVec2::splat(image_size);

    //creating a matrix based on x and y variable sizes
    let mut terrain_map: Matrix<f32> = Matrix::new(grid_size);

    //creating a an image buffer based on the x and y variable sizes
    let mut image_buffer = RgbImage::new(grid_size.x, grid_size.y);

    //creating a random noise generator variable to be used in the loop
    let noise_generator = OpenSimplexNoise::new(Some(rng.gen_value()));

    //looping through the matrix and assigning noise values to each value in the matrix
    for (height, position) in terrain_map.iter_with_pos_mut() {
        // assert!(normalised_radius >= 0.0 && normalised_radius <= 1.0);

        let noise_value = layered_noise(&noise_generator, position, scale, layer_count);
        let inverted_radius = island_gradient(position, 1.0, 1.0, 2);

        *height = noise_value * inverted_radius;

        // *height = (1.0 - (island_radius / max_distance).min(1.0));

        // assert!(!height.is_nan());

        // assert!(
        //     *height >= 0.0 && *height <= 1.0,
        //     "height: {}, pos: {}, inv_dist: {}",
        //     *height,
        //     position,
        //     inverted_radius
        // );

        // if position == UVec2::new(0, 0) {
        //     dbg!(inverted_radius, square_root, &height);
        // }

        // if position == UVec2::new(500, 500) {
        //     dbg!(inverted_radius, square_root, &height);
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
    // "max island_radius {} \n centre point {}",
    // max_distance, matrix_centre
    // );
}

fn get_colour(height: f32) -> Rgb<u8> {
    if height <= 0.1 {
        // water
        Rgb([0, 0, 255])
    } else if height <= 0.15 {
        // dark sand
        Rgb([190, 190, 0])
    } else if height <= 0.2 {
        // light sand
        Rgb([230, 230, 0])
    } else if height <= 0.38 {
        // grass
        Rgb([0, 160, 0])
    } else if height <= 0.8 {
        // mountains
        Rgb([150, 75, 0])
    } else if height <= 0.9 {
        // snow
        Rgb([255, 255, 255])
    } else {
        panic!()
    }
}

fn island_gradient(
    pos: UVec2,
    // grid_size: UVec2,
    // island_circumference: f32,
    x_scale: f32,
    y_scale: f32,
    // island_centre_x: u32,
    // island_centre_y: u32,
    island_count: u32,
) -> f32 {
    let mut inverted_radius: f32 = 0.0;
    // let mut amp = 1.0;
    // let mut total_amp = 0.0;

    for _ in 0..island_count {
        let rng = Rng::new();
        let mut pos_f = pos.as_vec2();
        pos_f.x *= x_scale;
        pos_f.y *= y_scale;
        let island_centre = UVec2::new(rng.gen_range(500..1500), rng.gen_range(500..1500));
        let island_radius = island_centre.as_vec2().distance(pos_f);
        let normalised_radius = (island_radius / rng.gen_range(200.0..800.0)).min(1.0);
        inverted_radius += 1.0 - normalised_radius;

        // total_amp += amp;
        // amp /= 2.0;
    }
    inverted_radius / island_count as f32
}

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
