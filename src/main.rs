use std::fs::File;

use rust_raytracer::{load_mesh, Diffuse, MaterialEnum, Metal, RayTracerConfig, Vec3};

fn main() {
    let mut output_file = File::create("output.ppm").expect("Failed to create PPM file");
    let mut ray_tracer = RayTracerConfig::default().build(&mut output_file);

    // Default scene
    // Floor object
    let mut floor = load_mesh("models/plane.obj", false);
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.4, 0.4), 0.0));

    // Cube object
    let mut cube = load_mesh("models/cube.obj", false);
    cube.scale(1.0);
    cube.rotate(Vec3::new(0.0, 10.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.4, -12.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    // Add objects to the world
    ray_tracer.add_mesh(floor);
    ray_tracer.add_mesh(cube);

    ray_tracer.run();
}
