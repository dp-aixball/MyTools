use xcap::Monitor;
use xcap::image::GenericImageView;

#[test]
fn test_screenshot_colors() {
    let monitors = Monitor::all().unwrap();
    let monitor = &monitors[0];
    let image = monitor.capture_image().unwrap();
    let width = image.width();
    let height = image.height();
    println!("Size: {}x{}", width, height);
    
    // Pick 5 random pixels and print them
    for i in 0..5 {
        let x = i * 20;
        let y = i * 20;
        if x < width && y < height {
            let pixel = image.get_pixel(x, y);
            println!("Pixel {}: [{}, {}, {}, {}]", i, pixel[0], pixel[1], pixel[2], pixel[3]);
        }
    }
}
