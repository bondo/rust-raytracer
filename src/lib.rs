mod config;
mod hit;
mod material;
mod mesh;
mod ray;
mod tracer;
mod triangle;
mod vec3;
mod world;

pub use config::{DrawingMode, RayTracerConfig};
pub use material::{Diffuse, Material, MaterialEnum, Metal};
pub use mesh::load_mesh;
use mesh::Mesh;
use ray::Ray;
pub use tracer::RayTracer;
pub use vec3::Vec3;
use vec3::{barycentric, unit_vector};
use world::World;
