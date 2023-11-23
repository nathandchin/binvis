use std::io::Read;

use clap::Parser;
use macroquad::prelude::*;

type BitMap = Vec<Vec<Vec<u32>>>;

#[derive(Parser, Debug)]
struct Args {
    /// The path of the file to visualize.
    input_pathname: String,
}

struct Visualization {
    buffer: BitMap,
}

impl Visualization {
    const HEIGHT: usize = 256;
    const WIDTH: usize = 256;
    const DEPTH: usize = 256;

    fn new_from_bytes(bytes: &[u8]) -> Self {
        let mut buffer: BitMap = vec![vec![vec![0; Self::DEPTH]; Self::WIDTH]; Self::HEIGHT];

        for pair in bytes.windows(3) {
            let x = pair[0] as usize;
            let y = pair[1] as usize;
            let z = pair[2] as usize;

            buffer[z][y][x] += 1;
        }

        Self { buffer }
    }

    fn draw(&self) {
        for z in 0..Self::DEPTH {
            for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    if self.buffer[z][y][x] > 0 {
                        let brightness = ((self.buffer[z][y][x] as f32).log(1.01) * 0.192)
                            .clamp(0.0, 255.0) as u8;

                        if brightness < 15 {
                            continue;
                        }

                        draw_cube(
                            vec3(x as f32 - 128., y as f32 - 128., z as f32 - 128.),
                            Vec3::ONE,
                            None,
                            Color::from_rgba(255, 255, 255, brightness),
                        );
                    }
                }
            }
        }
    }
}

#[macroquad::main("Binvis")]
async fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.input_pathname)
        .expect(format!("Could not read file {}", &args.input_pathname).as_str());

    let vis = Visualization::new_from_bytes(&contents);

    let mut position = vec3(275., 275., 275.);

    loop {
        clear_background(BLACK);

        position = Mat3::from_rotation_y(0.01) * position;

        set_camera(&Camera3D {
            position,
            up: Vec3::Y,
            target: Vec3::ZERO,
            ..Default::default()
        });

        vis.draw();

        next_frame().await;
    }
}

fn read_file_contents(pathname: &String) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(pathname)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}
