use screenshots::Screen;
use enigo::{Enigo, MouseControllable};

#[test]
fn test_mouse_color() {
    let mut enigo = Enigo::new();
    let (x, y) = enigo.mouse_location();
    println!("Mouse is at: {}, {}", x, y);
    
    let screens = Screen::all().unwrap();
    let primary = screens.first().unwrap();
    
    let image = primary.capture_area(x, y, 1, 1).unwrap();
    let raw = image.into_raw();
    if raw.len() >= 4 {
        println!("R={}, G={}, B={}, A={}", raw[0], raw[1], raw[2], raw[3]);
    }
}
