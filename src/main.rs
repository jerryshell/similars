use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'x', long)]
    image_x_path: String,
    #[clap(short = 'y', long)]
    image_y_path: String,
    #[clap(short = 'w', long, default_value_t = 8)]
    convert_width: u32,
    #[clap(short = 'h', long, default_value_t = 8)]
    convert_height: u32,
    #[clap(
        short = 'd',
        long,
        help = "By default, the output is a percentile of similarity, and if this parameter is explicitly specified, the output is hamming distance."
    )]
    distance_flag: bool,
}

fn main() {
    let args = Args::parse();

    let distance = similars::get_image_distance_by_path(
        args.image_x_path,
        args.image_y_path,
        args.convert_width,
        args.convert_height,
    );

    if args.distance_flag {
        println!("{}", distance);
        return;
    }

    let percentile =
        (1.0f32 - distance as f32 / (args.convert_width * args.convert_height) as f32) * 100.0f32;

    println!("{}", percentile);
}