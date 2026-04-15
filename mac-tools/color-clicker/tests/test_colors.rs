use screenshots::Screen;

#[test]
fn test_screenshot_colors() {
    let screens = Screen::all().unwrap();
    let image = screens[0].capture_area(0, 0, 100, 100).unwrap();
    let width = image.width();
    let height = image.height();
    let raw = image.into_raw();
    println!("Size: {}x{}, raw len: {}", width, height, raw.len());
    
    // Pick 5 random pixels and print them
    for i in 0..5 {
        let idx = i * 4 * width as usize + (i * 4);
        if idx + 3 < raw.len() {
            println!("Pixel {}: [{}, {}, {}, {}]", i, raw[idx], raw[idx+1], raw[idx+2], raw[idx+3]);
        }
    }
}
