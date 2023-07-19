mod camera;
mod config;
mod error;
mod hit;
mod material;
mod mesh;
mod ray;
mod tracer;
mod triangle;
mod vec3;
mod world;

pub use config::{DrawingMode, RayTracerConfig};
pub use error::Error;
pub use material::{Diffuse, Material, MaterialEnum, Metal};
pub use mesh::load_mesh;
pub use tracer::RayTracer;
pub use vec3::Vec3;

use camera::Camera;
use error::Result;
use hit::Hit;
use mesh::Mesh;
use ray::Ray;
use triangle::Triangle;
use world::World;
