use image::{DynamicImage, GenericImage};
use nalgebra::{Point3, Vector3};
use progress_bar::*;
use rand::{thread_rng, Rng, RngCore};
use rayon::prelude::*;
use std::time::Instant;

use camera::Camera;
use ray::Ray;
use world::World;

mod aabb;
pub mod bvh;
pub mod camera;
pub mod geometry;
pub mod material;
mod ray;
pub mod texture;
mod utility;
pub mod world;

/// A structure encapsulating elements to render a scene.
pub struct Renderer {
    /// The given width of the image to render.
    image_width: u32,
    /// The height of the rendered image, computed using the width and ratio.
    image_height: u32,
    /// The camera used for rendering.
    pub camera: Camera,
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

        //let (viewport_width, viewport_height) = (camera.viewport_width, camera.viewport_height);
        let theta = camera.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera.focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        // Horizontal vector representing the width of the viewport.
        let viewport_u = viewport_width * camera.frame_basis.u;
        // Vertical descending vector representing the height of the viewport.
        let viewport_v = -viewport_height * camera.frame_basis.v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera.center
            - (camera.focus_distance * camera.frame_basis.w)
            - viewport_u / 2.0
            - viewport_v / 2.0;

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

    /// Renders the image.
    pub fn render_image(&self, world: &World) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.image_width, self.image_height);
        let mut rng = thread_rng();

        init_progress_bar(self.image_height as usize);
        set_progress_bar_action("Rendering", Color::Blue, Style::Bold);
        let start_time = Instant::now();

        for y in 0..img.height() {
            inc_progress_bar();
            for x in 0..img.width() {
                let pixel_color = self.render_pixel(x, y, &mut rng, world);

                // Add the pixel to the image, after converting integers to `u8`.
                img.put_pixel(x, y, self.camera.color_to_pixel(pixel_color));
            }
        }

        print_progress_bar_final_info(
            "Rendered",
            format!("in {:?}", start_time.elapsed()).as_str(),
            Color::Green,
            Style::Bold,
        );
        finalize_progress_bar();

        img
    }

    /// Renders the image using multiple threads. Usually much faster than `render_image`.
    pub fn render_parallel_image(
        &self,
        world: &World,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        init_progress_bar_with_eta(self.image_height as usize);
        set_progress_bar_action("Rendering", Color::Blue, Style::Bold);
        let start_time = Instant::now();

        let vec_image = (0..self.image_height)
            .into_par_iter() // use rayon to enable multithreading
            .map(|y| {
                inc_progress_bar();

                (0..self.image_width)
                    .into_par_iter()
                    .map_init(thread_rng, |rng, x| {
                        let pixel_color = self.render_pixel(x, y, rng, world);

                        self.camera.color_to_pixel(pixel_color)
                    })
                    .collect::<Vec<image::Rgba<u8>>>()
            })
            .collect::<Vec<Vec<image::Rgba<u8>>>>();

        let img = image::ImageBuffer::from_fn(self.image_width, self.image_height, |x, y| {
            vec_image[y as usize][x as usize]
        });

        print_progress_bar_final_info(
            "Rendered",
            format!("in {:?}", start_time.elapsed()).as_str(),
            Color::Green,
            Style::Bold,
        );
        finalize_progress_bar();

        img
    }

    /// Renders the image using multiple threads for real-time use.
    pub fn render_parallel_image_data(&self, world: &World) -> Vec<u8> {
        (0..self.image_height)
            .into_par_iter() // use rayon to enable multithreading
            .map(|y| {
                (0..self.image_width)
                    .into_par_iter()
                    .map_init(thread_rng, |rng, x| {
                        let pixel_color = self.render_pixel(x, y, rng, world);

                        vec![
                            (pixel_color.x.sqrt() * 255.0) as u8,
                            (pixel_color.y.sqrt() * 255.0) as u8,
                            (pixel_color.z.sqrt() * 255.0) as u8,
                            255,
                        ]
                    })
                    .flatten()
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect::<Vec<u8>>()
    }

    fn render_pixel(&self, x: u32, y: u32, rng: &mut dyn RngCore, world: &World) -> Vector3<f64> {
        let mut pixel_color = Vector3::from([0.0, 0.0, 0.0]);

        // Send a given number of random rays in the same overall direction.
        for _ in 0..self.camera.samples_per_pixel {
            let ray = self.random_ray(x, y, rng);
            pixel_color += ray.color(self.camera.max_depth, world, rng);
        }

        // Take the mean of the colors retrieved by the random rays.
        pixel_color /= self.camera.samples_per_pixel as f64;
        pixel_color
    }

    /// Generates a ray corresponding to the given pixel `(x, y)`.
    /// To the standard direction of the ray is added some random noise to have different samples.
    /// The ray is originating from the camera defocus disk.
    fn random_ray(&self, x: u32, y: u32, rng: &mut dyn RngCore) -> Ray {
        // The center of the square pixel.
        let pixel_center = self.upper_left_pixel
            + (x as f64 * self.pixel_delta_u)
            + (y as f64 * self.pixel_delta_v);
        // A random point of the square pixel.
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);
        // Vector pointing from the camera towards the random point of the pixel.
        let origin = self.camera.defocus_disk_sample(rng);
        let ray_direction = pixel_sample - origin;

        let time = rng.gen();

        Ray::new(origin, ray_direction, time)
    }

    /// Generates a vector from the center of the pixel to a random point of the square pixel.
    fn pixel_sample_square(&self, rng: &mut dyn RngCore) -> Vector3<f64> {
        let dx = -0.5 + rng.gen::<f64>();
        let dy = -0.5 + rng.gen::<f64>();
        dx * self.pixel_delta_u + dy * self.pixel_delta_v
    }
}
