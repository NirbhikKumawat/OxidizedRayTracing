use std::env;
use std::fs::File;
use std::io::{BufWriter,Write};
use OxidisedRayTracing::color::{print_color, Color};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    let image_width = 256;
    let image_height = 256;

    writeln!(writer,"P3\n{} {}\n255", image_width, image_height)?;
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0f64;
            print_color(&Color::new(r, g, b), &mut writer);
        }
    }
    writer.flush()?;
    Ok(())
}
