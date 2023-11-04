use std::io::Read;

use clap::Parser;
use macroquad::prelude::*;

const HEIGHT: usize = 256;
const WIDTH: usize = 256;
const DEPTH: usize = 256;

type BitMap = Vec<Vec<Vec<u32>>>;

#[derive(Parser, Debug)]
struct Args {
    /// The path of the file to visualize.
    input_pathname: String,
}

#[macroquad::main("Binvis")]
async fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.input_pathname)
        .expect(format!("Could not read file {}", &args.input_pathname).as_str());

    let mut buffer: BitMap = vec![vec![vec![0; DEPTH]; WIDTH]; HEIGHT];
    populate_buffer(&mut buffer, &contents);

    let mut position = vec3(400., 400., 400.);

    loop {
        clear_background(BLACK);

        position.x += 1.;
        position.z -= 1.;
        set_camera(&Camera3D {
            position,
            up: Vec3::Y,
            target: Vec3::ZERO,
            ..Default::default()
        });

        draw_buffer(&buffer);

        next_frame().await;
    }
}

fn draw_buffer(buffer: &BitMap) {
    for z in 0..DEPTH {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if buffer[z][y][x] > 0 {
                    let brightness =
                        ((buffer[z][y][x] as f32).log(1.01) * 0.192).clamp(0.0, 255.0) as u8;

                    // image.get_pixel_mut(x as u32, y as u32).0 = [brightness, brightness, brightness];
                    // draw_cube(position, size, texture, color)
                    draw_cube(
                        vec3(x as f32, y as f32, z as f32),
                        Vec3::ONE,
                        None,
                        Color::from_rgba(255, 255, 255, brightness),
                    );
                }
            }
        }
    }
}

fn populate_buffer(buffer: &mut BitMap, source: &[u8]) {
    for pair in source.windows(3) {
        let x = pair[0] as usize;
        let y = pair[1] as usize;
        let z = pair[2] as usize;

        buffer[z][y][x] += 1;
    }
}

fn read_file_contents(pathname: &String) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(pathname)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}
