use std::io::Read;

use clap::Parser;
use raylib::prelude::*;

/// Pres `C` while running to capture/release the cursor.
#[derive(Parser, Debug)]
struct Args {
    /// The path of the file to visualize.
    input_pathname: String,

    /// The minimum "brightness" of a location in the input file to qualify for
    /// being drawn. A higher number will make the visualizion show fewer points
    /// and thus be dimmer.
    #[arg(short = 't', long, default_value_t = 15)]
    draw_threshold: u8,
}

struct Visualization {
    points: Vec<(f32, f32, f32)>,
}

impl Visualization {
    const HEIGHT: usize = 256;
    const WIDTH: usize = 256;
    const DEPTH: usize = 256;

    fn new_from_bytes(bytes: &[u8], draw_threshold: u8) -> Self {
        let mut buffer = vec![vec![vec![0; Self::DEPTH]; Self::WIDTH]; Self::HEIGHT];

        for pair in bytes.windows(3) {
            let x = pair[0] as usize;
            let y = pair[1] as usize;
            let z = pair[2] as usize;

            buffer[z][y][x] += 1;
        }

        let mut points = vec![];

        // Only want to draw sufficiently "bright" points
        #[allow(clippy::needless_range_loop)]
        for z in 0..Self::DEPTH {
            for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    let brightness =
                        ((buffer[z][y][x] as f32).log(1.01) * 0.192).clamp(0.0, 255.0) as u8;
                    if brightness >= draw_threshold {
                        points.push((x as f32 - 128., y as f32 - 128., z as f32 - 128.));
                    }
                }
            }
        }

        Self { points }
    }

    fn draw(&self, d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>) {
        for &(x, y, z) in &self.points {
            d.draw_point3D(Vector3::new(x, y, z), Color::WHITE);
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.input_pathname)
        .unwrap_or_else(|_| panic!("Could not read file {}", &args.input_pathname));

    let vis = Visualization::new_from_bytes(&contents, args.draw_threshold);

    let (mut rl, thread) = raylib::init().size(1920, 1080).build();
    let mut camera = Camera3D::perspective(
        Vector3::new(200., 200., 200.),
        Vector3::new(0.0, 1.8, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    rl.set_target_fps(60);
    rl.disable_cursor();

    while !rl.window_should_close() {
        if matches!(rl.get_key_pressed(), Some(KeyboardKey::KEY_C)) {
            if rl.is_cursor_hidden() {
                rl.show_cursor()
            } else {
                rl.disable_cursor();
            }
        }

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
