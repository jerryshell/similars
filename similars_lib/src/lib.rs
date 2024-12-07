pub fn load_image_resize_gray<P>(
    path: P,
    width: u32,
    height: u32,
) -> Result<image::GrayImage, image::ImageError>
where
    P: AsRef<std::path::Path>,
{
    let img = image::open(path)?;
    let img_resize = img.resize_exact(width, height, image::imageops::FilterType::Nearest);
    let img_resize_gray = img_resize.into_luma8();
    Ok(img_resize_gray)
}

pub fn image_hash<P>(
    path: P,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<Vec<u8>, image::ImageError>
where
    P: AsRef<std::path::Path>,
{
    let img_resize_gray = load_image_resize_gray(path, hamming_width, hamming_height)?;
    let img_avg =
        img_resize_gray.iter().map(|&p| p as u32).sum::<u32>() / (hamming_width * hamming_height);
    let img_hash = img_resize_gray
        .iter()
        .map(|&p| if p as u32 >= img_avg { 1 } else { 0 })
        .collect();
    Ok(img_hash)
}

pub fn image_distance<P>(
    image_x_path: P,
    image_y_path: P,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<usize, image::ImageError>
where
    P: AsRef<std::path::Path>,
{
    let img_x_hash = image_hash(image_x_path, hamming_width, hamming_height)?;
    let img_y_hash = image_hash(image_y_path, hamming_width, hamming_height)?;
    let img_distance = img_x_hash
        .iter()
        .zip(img_y_hash.iter())
        .filter(|(x, y)| x != y)
        .count();
    Ok(img_distance)
}
