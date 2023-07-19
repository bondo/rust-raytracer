use criterion::{criterion_group, criterion_main, Criterion};
use rust_raytracer::{load_mesh, Diffuse, MaterialEnum, Metal, RayTracer, Vec3};

fn setup() -> anyhow::Result<RayTracer> {
    let mut ray_tracer = RayTracer::default();

    // Floor object
    let mut floor = load_mesh("models/plane.obj", false)?;
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.4, 0.4), 0.0));

    // Cube object
    let mut cube = load_mesh("models/cube.obj", false)?;
    cube.scale(1.0);
    cube.rotate(Vec3::new(0.0, 10.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.4, -12.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    // Add objects to the world
    ray_tracer.add_mesh(floor);
    ray_tracer.add_mesh(cube);

    Ok(ray_tracer)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sequential", |b| {
        b.iter(|| {
            let mut output: Vec<u8> = vec![];
            let ray_tracer = setup().unwrap();
            ray_tracer.run_sequential(&mut output).unwrap();
            output
        })
    });

    c.bench_function("parallel", |b| {
        b.iter(|| {
            let mut output: Vec<u8> = vec![];
            let ray_tracer = setup().unwrap();
            ray_tracer.run_parallel(&mut output).unwrap();
            output
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
