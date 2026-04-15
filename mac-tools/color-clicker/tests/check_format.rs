use screenshots::Screen;

#[test]
fn test_screenshot() {
    let screens = Screen::all().unwrap();
    let image = screens[0].capture().unwrap();
    println!("Width: {}, Height: {}", image.width(), image.height());
    // In screenshots 0.8, the image type is image::RgbaImage returned by rgba() or similar. We can check if rgba() exists.
    let rgba = image.rgba();
    println!("First pixel rgba: {:?}", &rgba[0..4]);
}
