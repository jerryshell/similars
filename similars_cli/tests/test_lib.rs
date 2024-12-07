#[test]
fn test_image_distance1() {
    let distance =
        similars_lib::image_distance("example_img1.jpg", "example_img2.jpg", 8, 8).unwrap();
    assert_eq!(distance, 3)
}

#[test]
fn test_image_distance2() {
    let distance =
        similars_lib::image_distance("example_img1.jpg", "example_img1.jpg", 8, 8).unwrap();
    assert_eq!(distance, 0)
}
