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
}

fn main() {
    let args = Args::parse();

    let distance = similars::get_image_distance_by_path(
        args.image_x_path,
        args.image_y_path,
        args.convert_width,
        args.convert_height,
    );
    println!("{}", distance);
}
