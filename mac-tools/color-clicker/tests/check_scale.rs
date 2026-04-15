use screenshots::Screen;

#[test]
fn test_scale() {
    let screens = Screen::all().unwrap();
    let primary = screens.first().unwrap();
    println!("Screen pos: {}, {} ({}x{}) scale: {}",
        primary.display_info.x,
        primary.display_info.y,
        primary.display_info.width,
        primary.display_info.height,
        primary.display_info.scale_factor
    );
}
