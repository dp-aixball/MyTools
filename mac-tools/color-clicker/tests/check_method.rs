use screenshots::Screen;

#[test]
fn test_screenshot() {
    let screens = Screen::all().unwrap();
    let image = screens[0].capture_area(0, 0, 10, 10).unwrap();
    println!("Width: {}, Height: {}", image.width(), image.height());
}
