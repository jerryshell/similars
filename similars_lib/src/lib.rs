use anyhow::Result;
use image::GenericImageView;

pub fn get_resize_gray_image_by_path(
    path: &str,
    hamming_width: u32,
    hamming_height: u32,
    debug_flag: bool,
) -> Result<image::GrayImage> {
    let img = image::io::Reader::open(path)?.decode()?;

    let img_resize = img.resize_exact(
        hamming_width,
        hamming_height,
        image::imageops::FilterType::Nearest,
    );
    if debug_flag {
        println!("img_resize.dimensions {:?}", img_resize.dimensions());
    }

    let img_resize_gray = img_resize.into_luma8();

    Ok(img_resize_gray)
}

pub fn get_image_hash_by_path(
    path: &str,
    hamming_width: u32,
    hamming_height: u32,
    debug_flag: bool,
) -> Result<Vec<u8>> {
    let img_resize_gray =
        get_resize_gray_image_by_path(path, hamming_width, hamming_height, debug_flag)?;
    if debug_flag {
        if let Some(filename) = path.split('/').last() {
            println!("filename {}", filename);
            let debug_filename = format!("debug.{}", filename);
            println!("debug_filename {}", debug_filename);
            img_resize_gray.save(debug_filename)?;
        }
    }

    let img_resize_gray_sum = img_resize_gray.iter().map(|p| *p as u32).sum::<u32>();
    let img_resize_gray_avg = img_resize_gray_sum / (hamming_width * hamming_height);
    let image_hash = img_resize_gray
        .iter()
        .map(|p| match p {
            p if img_resize_gray_avg >= *p as u32 => 1,
            _ => 0,
        })
        .collect::<Vec<u8>>();

    Ok(image_hash)
}

pub fn get_image_distance_by_path(
    image_x_path: &str,
    image_y_path: &str,
    hamming_width: u32,
    hamming_height: u32,
    debug_flag: bool,
) -> Result<usize> {
    let img_x_hash =
        get_image_hash_by_path(image_x_path, hamming_width, hamming_height, debug_flag)?;
    if debug_flag {
        println!("img_x_hash {:?}", img_x_hash);
    }

    let img_y_hash =
        get_image_hash_by_path(image_y_path, hamming_width, hamming_height, debug_flag)?;
    if debug_flag {
        println!("img_y_hash {:?}", img_y_hash);
    }

    let img_distance = img_x_hash
        .iter()
        .enumerate()
        .filter(|(index, item)| match (*img_y_hash).get(*index) {
            Some(yp) => **item != *yp,
            None => false,
        })
        .count();
    if debug_flag {
        println!("img_distance {}", img_distance);
    }

    Ok(img_distance)
}
