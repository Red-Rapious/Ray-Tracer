use lib_ray_tracer::render_image;


fn main() {
    let img = render_image();

    img.save("generated_images/hello_world.png").unwrap();
}
