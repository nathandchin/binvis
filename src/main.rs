use std::io::Read;

use clap::Parser;
use raylib::prelude::*;

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

    fn draw(&self, d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        for z in 0..Self::DEPTH {
            for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    if self.buffer[z][y][x] > 0 {
                        let brightness = ((self.buffer[z][y][x] as f32).log(1.01) * 0.192)
                            .clamp(0.0, 255.0) as u8;

                        if brightness < 15 {
                            continue;
                        }

                        d.draw_cube(
                            Vector3::new(x as f32 - 128., y as f32 - 128., z as f32 - 128.),
                            0.8,
                            0.8,
                            0.8,
                            Color::from_hex("F0F0F0").unwrap(),
                        );
                    }
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.input_pathname)
        .expect(format!("Could not read file {}", &args.input_pathname).as_str());

    let vis = Visualization::new_from_bytes(&contents);

    let (mut rl, thread) = raylib::init().size(1920, 1080).build();
    let mut camera = Camera3D::perspective(
        Vector3::new(275., 275., 275.),
        Vector3::new(0.0, 1.8, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_THIRD_PERSON);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        let mut d2 = d.begin_mode3D(camera);
        vis.draw(&mut d2);
    }
}

fn read_file_contents(pathname: &String) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(pathname)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}
