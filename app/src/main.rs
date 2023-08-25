use image::{Rgba, DynamicImage, GenericImage};
use progress_bar::{init_progress_bar, set_progress_bar_action, Color, Style, inc_progress_bar, finalize_progress_bar};

fn main() {
    // Construct a new by repeated calls to the supplied closure.
    let mut img = DynamicImage::new_rgb8(256, 256);
        
    init_progress_bar(256);
    set_progress_bar_action("Rendering", Color::Blue, Style::Bold);
    for y in 0..img.height() {
        inc_progress_bar();
        for x in 0..img.width() {
            img.put_pixel(x, y, Rgba([x as u8, y as u8, 0, 1]));
        }
    }
    finalize_progress_bar();

    img.save("generated_images/hello_world.png").unwrap();
}
