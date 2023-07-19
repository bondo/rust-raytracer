use std::fs::File;

use anyhow::Context;
use rust_raytracer::{load_mesh, Diffuse, DrawingMode, MaterialEnum, Metal, RayTracerConfig, Vec3};

fn main() -> anyhow::Result<()> {
    let mut ray_tracer = RayTracerConfig::default()
        .width(1000)
        .height(1000)
        .mode(DrawingMode::Samples(5))
        .build();

    // Default scene
    // Floor object
    let mut floor = load_mesh("models/plane.obj", false).context("Failed to load plane mesh")?;
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.4, 0.4), 0.0));

    // Cube object
    let mut cube = load_mesh("models/cube.obj", false).context("Failed to load cube mesh")?;
    cube.scale(1.0);
    cube.rotate(Vec3::new(0.0, 10.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.4, -12.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    // Add objects to the world
    ray_tracer.add_mesh(floor);
    ray_tracer.add_mesh(cube);

    let mut output_file = File::create("output.ppm").context("Failed to create PPM file")?;
    ray_tracer
        .run_parallel(&mut output_file)
        .context("Failed to run ray tracer")?;

    Ok(())
}
