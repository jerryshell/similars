pub fn get_resize_gray_image_by_path(
    path: String,
    convert_width: u32,
    convert_height: u32,
) -> image::GrayImage {
    let img = image::io::Reader::open(path).unwrap().decode().unwrap();
    let img_resize = img.resize(
        convert_width,
        convert_height,
        image::imageops::FilterType::Nearest,
    );
    img_resize.into_luma8()
}

pub fn get_image_hash_by_path(path: String, convert_width: u32, convert_height: u32) -> Vec<u8> {
    let img_resize_gray = get_resize_gray_image_by_path(path, convert_width, convert_height);
    let img_resize_gray_sum = img_resize_gray.iter().map(|p| *p as u32).sum::<u32>();
    let img_resize_gray_avg = img_resize_gray_sum / (convert_width * convert_height);
    img_resize_gray
        .iter()
        .map(|p| {
            if img_resize_gray_avg > *p as u32 {
                1
            } else {
                0
            }
        })
        .collect::<Vec<u8>>()
}

pub fn get_image_distance_by_path(
    image_x_path: String,
    image_y_path: String,
    convert_width: u32,
    convert_height: u32,
) -> usize {
    let img_x_hash = get_image_hash_by_path(image_x_path, convert_width, convert_height);
    let img_y_hash = get_image_hash_by_path(image_y_path, convert_width, convert_height);
    img_x_hash
        .iter()
        .enumerate()
        .filter(|(index, item)| **item != *img_y_hash.get(*index).unwrap())
        .count()
}
