pub fn get_resize_gray_image_by_path(
    path: &str,
    width: u32,
    height: u32,
) -> Result<image::GrayImage, image::ImageError> {
    let img = image::open(path)?;
    let img_resize = img.resize_exact(width, height, image::imageops::FilterType::Nearest);
    let img_resize_gray = img_resize.into_luma8();
    Ok(img_resize_gray)
}

pub fn get_image_hash_by_path(
    path: &str,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<Vec<u8>, image::ImageError> {
    let img_resize_gray = get_resize_gray_image_by_path(path, hamming_width, hamming_height)?;
    let img_avg =
        img_resize_gray.iter().map(|&p| p as u32).sum::<u32>() / (hamming_width * hamming_height);
    let img_hash = img_resize_gray
        .iter()
        .map(|&p| if p as u32 >= img_avg { 1 } else { 0 })
        .collect();
    Ok(img_hash)
}

pub fn get_image_distance_by_path(
    image_x_path: &str,
    image_y_path: &str,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<usize, image::ImageError> {
    let img_x_hash = get_image_hash_by_path(image_x_path, hamming_width, hamming_height)?;
    let img_y_hash = get_image_hash_by_path(image_y_path, hamming_width, hamming_height)?;
    let img_distance = img_x_hash
        .iter()
        .zip(img_y_hash.iter())
        .filter(|(x, y)| x != y)
        .count();
    Ok(img_distance)
}
