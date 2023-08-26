use camera::Camera;
use image::{DynamicImage, GenericImage};
use nalgebra::{Point3, Vector3};
use progress_bar::{
    finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color,
    Style,
};
use ray::Ray;

pub mod camera;
mod ray;

/// A structure encapsulating elements to render a scene.
pub struct Renderer {
    /// The given width of the image to render.
    image_width: u32,
    /// The height of the rendered image, computed using the width and ratio.
    image_height: u32,
    /// The camera used for rendering.
    camera: Camera,
    /// The world's coordinates of the upper left pixel of the viewport.
    upper_left_pixel: Point3<f64>,
    /// The vector representing the horizontal spacing between two centers of pixels.
    pixel_delta_u: Vector3<f64>,
    /// The vector representing the vertical spacing between two centers of pixels.
    pixel_delta_v: Vector3<f64>,
}

impl Renderer {
    /// Initialise a new renderer, given the ideal aspect ratio of the image, the image width, and a `Camera`.
    pub fn new(aspect_ratio: f64, image_width: u32, camera: Camera) -> Self {
        assert_ne!(aspect_ratio, 0.0);

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        assert!(image_height > 0);

        let (viewport_width, viewport_height) = (camera.viewport_width, camera.viewport_height);

        // Horizontal vector representing the width of the viewport.
        let viewport_u = Vector3::from([viewport_width, 0.0, 0.0]);
        // Vertical descending vector representing the height of the viewport.
        let viewport_v = Vector3::from([0.0, -viewport_height, 0.0]);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera.center()
            - Vector3::from([0.0, 0.0, camera.focal_length])
            - 0.5 * (viewport_u + viewport_v);

        let upper_left_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            camera,
            upper_left_pixel,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    /// Render the image.
    pub fn render_image(&self) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.image_width, self.image_height);

        init_progress_bar(self.image_height as usize);
        set_progress_bar_action("Rendering", Color::Blue, Style::Bold);
        for y in 0..img.height() {
            inc_progress_bar();
            for x in 0..img.width() {
                let pixel_center = self.upper_left_pixel
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera.center();

                let ray = Ray::new(*self.camera.center(), ray_direction);

                let pixel_color = ray.color();
                img.put_pixel(x, y, pixel_color);
            }
        }
        finalize_progress_bar();

        img
    }
}
