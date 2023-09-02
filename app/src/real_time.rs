use lib_ray_tracer::{
    camera::{self, Camera},
    geometry::Sphere,
    material::Material,
    world::World,
    Renderer,
};
use nalgebra::{Point3, Vector3};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
//use std::{thread, time};

/// Minimum time between two frames of the simulation, in milliseconds
//const DELTA: u64 = 30;

const UPSCALE: u32 = 5;

pub fn run() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 640;

    let camera = Camera::new(
        5,
        10,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        camera::Gamma::Gamma2,
        0.6,
        10.0,
    );

    let mut world = World::empty();

    let ground_mat = Material::Lambertian(Vector3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Vector3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Vector3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let window = Window::new_centered(
        "Ray Tracer",
        (
            image_width * UPSCALE,
            (image_width as f64 / aspect_ratio) as u32 * UPSCALE,
        ),
    )
    .unwrap();
    window.run_loop(RealTimeRenderer::new(
        aspect_ratio,
        image_width,
        camera,
        world,
    ));
}

struct RealTimeRenderer {
    size: (u32, u32),
    renderer: Renderer,
    mouse_position: Vec2,
    world: World,
    last_frame: std::time::Instant,
}

impl RealTimeRenderer {
    pub fn new(aspect_ratio: f64, image_width: u32, camera: Camera, world: World) -> Self {
        Self {
            size: (image_width, (image_width as f64 / aspect_ratio) as u32),
            renderer: Renderer::new(aspect_ratio, image_width, camera),
            mouse_position: Vec2::new(0.0, 0.0),
            world,
            last_frame: std::time::Instant::now(),
        }
    }
}

impl WindowHandler for RealTimeRenderer {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        helper.set_resizable(false);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);
        let data = self.renderer.render_parallel_image_data(&self.world);
        let handle = graphics
            .create_image_from_raw_pixels(
                speedy2d::image::ImageDataType::RGBA,
                speedy2d::image::ImageSmoothingMode::Linear,
                self.size,
                &data,
            )
            .unwrap();

        graphics.draw_rectangle_image(
            Rectangle::new(
                Vec2::new(0.0, 0.0),
                Vec2::new((self.size.0 * UPSCALE) as f32, (self.size.1 * UPSCALE) as f32),
            ),
            &handle,
        );
        //graphics.draw_text((0.0, 0.0), Color::BLACK, format!("{:?}", self.last_frame.elapsed()));
        //println!("{:?}", self.last_frame.elapsed());
        self.last_frame = std::time::Instant::now();

        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _: speedy2d::window::KeyScancode,
    ) {
        if virtual_key_code == Some(VirtualKeyCode::Space) {}
    }

    fn on_mouse_move(&mut self, _: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_position = position;
    }
}
