#[test]
fn test_get_image_distance_by_path1() {
    let distance = similars_lib::get_image_distance_by_path(
        "example_img1.jpg",
        "example_img2.jpg",
        8,
        8,
        false,
        false,
    )
    .unwrap();
    assert_eq!(distance, 3)
}

#[test]
fn test_get_image_distance_by_path2() {
    let distance = similars_lib::get_image_distance_by_path(
        "example_img1.jpg",
        "example_img1.jpg",
        8,
        8,
        false,
        false,
    )
    .unwrap();
    assert_eq!(distance, 0)
}
