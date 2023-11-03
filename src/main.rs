use std::io::Read;

use clap::Parser;
use image::RgbImage;

const HEIGHT: usize = 256;
const WIDTH: usize = 256;

type BitMap = Vec<Vec<u32>>;

#[derive(Parser, Debug)]
struct Args {
    /// The path of the file to visualize.
    input_pathname: String,

    /// The path to write to.
    output_pathname: String,
}
fn main() {
    let args = Args::parse();
    let contents = read_file_contents(&args.input_pathname)
        .expect(format!("Could not read file {}", &args.input_pathname).as_str());

    let mut buffer: BitMap = vec![vec![0; WIDTH]; HEIGHT];
    populate_buffer(&mut buffer, &contents);

    draw_buffer(&buffer, &args.output_pathname);
}

fn draw_buffer(buffer: &BitMap, pathname: &String) {
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if buffer[y][x] > 0 {
                let brightness = ((buffer[y][x] as f32).log10() * 106.0).clamp(0.0, 255.0) as u8;

                image.get_pixel_mut(x as u32, y as u32).0 = [brightness, brightness, brightness];
            }
        }
    }

    image.save(pathname).unwrap();
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
