use lib_ray_tracer::{
    bvh::BVHNode,
    camera::{self, Camera},
    geometry::Sphere,
    material::Material,
    texture::Texture,
    world::World,
    Renderer,
};
use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};
use image::{ImageBuffer, Rgba};

pub fn render() {
    let img = match 2 {
        0 => random_spheres(),
        1 => two_spheres(),
        //2 => earth(),
        _ => panic!()
    };
    img.save("generated_images/24_two_spheres.png").unwrap();
}

fn random_spheres() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let camera = Camera::new(
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        camera::Gamma::Gamma2,
        0.6,
        10.0,
    );

    let mut world = World::empty();
    let mut rng = thread_rng();

    //let ground_mat = Material::Lambertian(Vector3::new(0.5, 0.5, 0.5));
    static EVEN: Texture = Texture::SolidColor(Vector3::new(0.2, 0.3, 0.1));
    static ODD: Texture = Texture::SolidColor(Vector3::new(0.9, 0.9, 0.9));
    let checker = Material::TexturedLambertian(Texture::Checker(3.0, &EVEN, &ODD));
    world.add(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let choose_mat: f64 = rng.gen();

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vector3::<f64>::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );
                    let sphere_material = Material::Lambertian(albedo);
                    let center2 = center + Vector3::new(0.0, rng.gen_range(0.0..=0.5), 0.0);
                    world.add(Sphere::moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector3::new(
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                        rng.gen_range(0.5..=1.0),
                    );
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let sphere_material = Material::Metal(albedo, fuzz);
                    world.add(Sphere::stationary(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric(1.5);
                    world.add(Sphere::stationary(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = Material::Lambertian(Vector3::new(0.4, 0.2, 0.1));
    world.add(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Material::Metal(Vector3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    // Create a new world made of only one object, a `BVHNode`
    let mut world2 = World::empty();
    let l = world.objects().len();
    let mut objects = world.objects().drain(0..l).map(Some).collect();
    world2.add(BVHNode::new(&mut objects, 0, l));

    // Render the world that uses BVH
    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    renderer.render_parallel_image(&world2)
}

fn two_spheres() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let camera = Camera::new(
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        camera::Gamma::Gamma2,
        0.6,
        10.0,
    );

    let mut world = World::empty();

    static EVEN: Texture = Texture::SolidColor(Vector3::new(0.2, 0.3, 0.1));
    static ODD: Texture = Texture::SolidColor(Vector3::new(0.9, 0.9, 0.9));
    let checker = Material::TexturedLambertian(Texture::Checker(3.0, &EVEN, &ODD));
    world.add(Sphere::stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker,
    ));

    world.add(Sphere::stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    ));

    // Create a new world made of only one object, a `BVHNode`
    let mut world2 = World::empty();
    let l = world.objects().len();
    let mut objects = world.objects().drain(0..l).map(Some).collect();
    world2.add(BVHNode::new(&mut objects, 0, l));

    // Render the world that uses BVH
    let renderer = Renderer::new(aspect_ratio, image_width, camera);
    renderer.render_parallel_image(&world2)
}

/*fn earth() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let camera = Camera::new(
        100,
        50,
        20.0,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        camera::Gamma::Gamma2,
        0.0,
        10.0,
    );

    let mut world = World::empty();
    let mut rng = thread_rng();

    let earth_image: DynamicImage = image::open("./assets/earthmap.jpg").unwrap();
    let earth_surface = Material::TexturedLambertian(Texture::Image(&earth_image));
    world.add(Sphere::stationary(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));


    // Create a new world made of only one object, a `BVHNode`
    let mut world2 = World::empty();
    let l = world.objects().len();
    let mut objects = world.objects().drain(0..l).map(Some).collect();
    world2.add(BVHNode::new(&mut objects, 0, l));

    // Render the world that uses BVH
    let renderer = Renderer::new(16.0/9.0, 400, camera);
    renderer.render_parallel_image(&world2)
}*/