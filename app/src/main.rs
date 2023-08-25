use lib_ray_tracer::{Renderer, camera::Camera};
use nalgebra::Point3;

fn main() {
    let aspect_ratio = 16.0/9.0; // TODO: compute actual_ratio
    let image_width = 400;

    let renderer = Renderer::new(aspect_ratio, image_width, Camera::new(1.0, 2.0, aspect_ratio,Point3::from([0.0, 0.0, 0.0])));
    let img = renderer.render_image();

    img.save("generated_images/hello_world.png").unwrap();
}
