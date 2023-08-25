use camera::Camera;
use image::{Rgba, DynamicImage, GenericImage};
use nalgebra::{Vector3, Point3};
use progress_bar::{init_progress_bar, set_progress_bar_action, Color, Style, inc_progress_bar, finalize_progress_bar};

mod ray;
pub mod camera;

pub struct Renderer {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    camera: Camera,
    viewport_u: Vector3<f64>,
    viewport_v: Vector3<f64>,
    upper_left_pixel: Point3<f64>
}

impl Renderer {
    pub fn new(aspect_ratio: f64, image_width: usize, camera: Camera) -> Self {
        assert_ne!(aspect_ratio, 0.0);

        let image_height = (image_width as f64 / aspect_ratio) as usize;
        assert!(image_height > 0);

        let (viewport_width, viewport_height) = (camera.viewport_width, camera.viewport_height);
        let viewport_u = Vector3::from([viewport_width, 0.0, 0.0]);
        let viewport_v = Vector3::from([0.0, -viewport_height, 0.0]);


        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera.center() - Vector3::from([0.0, 0.0, camera.focal_length]) - 0.5 * (viewport_u + viewport_v);

        let upper_left_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            camera,
            viewport_u: Vector3::from([viewport_width, 0.0, 0.0]),
            viewport_v: Vector3::from([0.0, -viewport_height, 0.0]),
            upper_left_pixel
        }
    }

    pub fn render_image(&self) -> DynamicImage {
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
        img
    }
}