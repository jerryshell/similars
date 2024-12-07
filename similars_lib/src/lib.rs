use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimilarsError {
    #[error("Failed to read image: {path}")]
    Io { path: String },
    #[error("Failed to decode image: {path}")]
    Decode { path: String },
}

pub fn get_resize_gray_image_by_path(
    path: &str,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<image::GrayImage, SimilarsError> {
    let img_reader =
        image::ImageReader::open(path).map_err(|_e| SimilarsError::Io { path: path.into() })?;
    let img = img_reader
        .decode()
        .map_err(|_e| SimilarsError::Decode { path: path.into() })?;
    let img_resize = img.resize_exact(
        hamming_width,
        hamming_height,
        image::imageops::FilterType::Nearest,
    );
    let img_resize_gray = img_resize.into_luma8();
    Ok(img_resize_gray)
}

pub fn get_image_hash_by_path(
    path: &str,
    hamming_width: u32,
    hamming_height: u32,
) -> Result<Vec<u8>, SimilarsError> {
    let img_resize_gray = get_resize_gray_image_by_path(path, hamming_width, hamming_height)?;
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
) -> Result<usize, SimilarsError> {
    let img_x_hash = get_image_hash_by_path(image_x_path, hamming_width, hamming_height)?;
    tracing::debug!("img_x_hash {:?}", img_x_hash);
    let img_y_hash = get_image_hash_by_path(image_y_path, hamming_width, hamming_height)?;
    tracing::debug!("img_y_hash {:?}", img_y_hash);
    let img_distance = img_x_hash
        .iter()
        .enumerate()
        .filter(|(index, item)| match (*img_y_hash).get(*index) {
            Some(yp) => **item != *yp,
            None => false,
        })
        .count();
    tracing::debug!("img_distance {:?}", img_distance);
    Ok(img_distance)
}
