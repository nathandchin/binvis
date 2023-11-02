use macroquad::prelude::*;
use std::io::Read;

use clap::Parser;

const HEIGHT: usize = 256;
const WIDTH: usize = 256;
const SCALE: f32 = 2.0;

type BitMap = Vec<Vec<u32>>;

#[derive(Parser, Debug)]
struct Args {
    pathname: String,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "BinVis".to_string(),
        // window_height: HEIGHT as i32,
        // window_width: WIDTH as i32,
        window_height: (HEIGHT * SCALE as usize) as i32,
        window_width: (WIDTH * SCALE as usize) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.pathname)
        .expect(format!("Could not read file {}", &args.pathname).as_str());

    let mut buffer: BitMap = vec![vec![0; WIDTH]; HEIGHT];
    populate_buffer(&mut buffer, &contents);

    loop {
        clear_background(BLACK);

        draw_buffer(&buffer);

        next_frame().await
    }
}

fn draw_buffer(buffer: &BitMap) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if buffer[y][x] > 0 {
            let brightness: u32 = buffer[y][x];

            draw_rectangle(
                x as f32 * SCALE,
                y as f32 * SCALE,
                SCALE,
                SCALE,
                Color::from_rgba(255, 255, 255, brightness.clamp(0, 255).try_into().unwrap()),
            );
            }
        }
    }
}

fn populate_buffer(buffer: &mut BitMap, source: &[u8]) {
    for pair in source.windows(2) {
        let x = pair[0] as usize;
        let y = pair[1] as usize;

        buffer[y][x] += 1;
    }
}

fn read_file_contents(pathname: &String) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(pathname)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}
