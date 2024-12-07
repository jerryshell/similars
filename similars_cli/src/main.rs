use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'x', long)]
    image_x_path: String,
    #[clap(short = 'y', long)]
    image_y_path: String,
    #[clap(long = "width", default_value_t = 8)]
    hamming_width: u32,
    #[clap(long = "height", default_value_t = 8)]
    hamming_height: u32,
    #[clap(
        short = 'd',
        long,
        help = "By default, the output is the percentile of similarity, or the Hamming distance if --show-distance is explicitly specified."
    )]
    show_distance: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let distance = similars_lib::get_image_distance_by_path(
        &args.image_x_path,
        &args.image_y_path,
        args.hamming_width,
        args.hamming_height,
    )?;
    if args.show_distance {
        println!("{}", distance);
        return Ok(());
    }
    let percentile =
        (1f32 - distance as f32 / (args.hamming_width * args.hamming_height) as f32) * 100f32;
    println!("{}", percentile);
    Ok(())
}
