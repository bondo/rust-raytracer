use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use thiserror::Error;

use crate::{
    material::{Diffuse, MaterialEnum},
    vec3::unit_vector,
    Hit, Ray, Triangle, Vec3,
};

/// Mesh struct
#[derive(Clone, Debug)]
pub struct Mesh {
    /// All of the triangles in a mesh
    pub triangles: Vec<Triangle>,
    /// The mesh's material
    pub material: MaterialEnum,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Mesh {
    /// Create a new empty mesh
    /// # Default Values
    /// * 'triangles' - Empty Rust vec
    /// * 'material' - White diffuse material
    pub fn new() -> Mesh {
        Mesh {
            triangles: Vec::new(),
            material: MaterialEnum::Diffuse(Diffuse::new(Vec3::new(1.0, 1.0, 1.0))),
        }
    }

    /// Create a mesh with an already established Vec of triangles
    /// # Arguments
    /// * 'trigs' - Rust vec of triangles
    pub fn new_mesh(trigs: Vec<Triangle>) -> Mesh {
        Mesh {
            triangles: trigs,
            material: MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.5, 0.5, 0.5))),
        }
    }

    /// Create add triangles to a mesh
    /// # Arguments
    /// * 'trig' - Single triangle to add
    pub fn add(&mut self, trig: Triangle) {
        self.triangles.push(trig);
    }

    /// Translate a mesh
    /// # Arguments
    /// * 'd' - Vec3 which contains the x,y,z directions to translate
    pub fn translate(&mut self, d: Vec3) {
        // Loop over each triangle in the mesh and simply add x,y,z to the points to translate
        for trig in self.triangles.iter_mut() {
            for point in trig.points.iter_mut() {
                *point = Vec3::new(point.x + d.x, point.y + d.y, point.z + d.z);
            }
        }
    }

    /// Scale a mesh
    /// # Arguments
    /// * 'c' - Amount to scale
    pub fn scale(&mut self, c: f64) {
        // Loop over each triangle in the mesh and scale the points
        for trig in self.triangles.iter_mut() {
            for point in trig.points.iter_mut() {
                *point = Vec3::new(point.x * c, point.y * c, point.z * c);
            }
        }
    }

    /// Rotate a mesh
    /// # Arguments
    /// * 'r' - Vec3 in degrees NOT radians
    pub fn rotate(&mut self, r: Vec3) {
        // Must convert to radians
        let theta_x = r.x.to_radians();
        let theta_y = r.y.to_radians();
        let theta_z = r.z.to_radians();

        // Need to rotate each triangle in the mesh
        for trig in self.triangles.iter_mut() {
            // First, let's rotate the normals
            // Rotate on x
            trig.normal = Vec3::new(
                trig.normal.x,
                trig.normal.y * theta_x.cos() - trig.normal.z * theta_x.sin(),
                trig.normal.y * theta_x.sin() + trig.normal.z * theta_x.cos(),
            );

            // If the triangle is smoothly shaded, it will have normals for each vertex
            // Rotate each of the normals. If not smooth shaded, this doesn't do much
            trig.normals[0] = Vec3::new(
                trig.normals[0].x,
                trig.normals[0].y * theta_x.cos() - trig.normals[0].z * theta_x.sin(),
                trig.normals[0].y * theta_x.sin() + trig.normals[0].z * theta_x.cos(),
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x,
                trig.normals[1].y * theta_x.cos() - trig.normals[1].z * theta_x.sin(),
                trig.normals[1].y * theta_x.sin() + trig.normals[1].z * theta_x.cos(),
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x,
                trig.normals[2].y * theta_x.cos() - trig.normals[2].z * theta_x.sin(),
                trig.normals[2].y * theta_x.sin() + trig.normals[2].z * theta_x.cos(),
            );

            // EACH time we rotate, we must normalize
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

            // Rotate on y
            trig.normal = Vec3::new(
                trig.normal.x * theta_y.cos() + trig.normal.z * theta_y.sin(),
                trig.normal.y,
                -trig.normal.x * theta_y.sin() + trig.normal.z * theta_y.cos(),
            );
            trig.normals[0] = Vec3::new(
                trig.normals[0].x * theta_y.cos() + trig.normals[0].z * theta_y.sin(),
                trig.normals[0].y,
                -trig.normals[0].x * theta_y.sin() + trig.normals[0].z * theta_y.cos(),
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x * theta_y.cos() + trig.normals[1].z * theta_y.sin(),
                trig.normals[1].y,
                -trig.normals[1].x * theta_y.sin() + trig.normals[1].z * theta_y.cos(),
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x * theta_y.cos() + trig.normals[2].z * theta_y.sin(),
                trig.normals[2].y,
                -trig.normals[2].x * theta_y.sin() + trig.normals[2].z * theta_y.cos(),
            );
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

            // Rotate on z
            trig.normal = Vec3::new(
                trig.normal.x * theta_z.cos() - trig.normal.y * theta_z.sin(),
                trig.normal.x * theta_z.sin() + trig.normal.y * theta_z.cos(),
                trig.normal.z,
            );
            trig.normals[0] = Vec3::new(
                trig.normals[0].x * theta_z.cos() - trig.normals[0].y * theta_z.sin(),
                trig.normals[0].x * theta_z.sin() + trig.normals[0].y * theta_z.cos(),
                trig.normals[0].z,
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x * theta_z.cos() - trig.normals[1].y * theta_z.sin(),
                trig.normals[1].x * theta_z.sin() + trig.normals[1].y * theta_z.cos(),
                trig.normals[1].z,
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x * theta_z.cos() - trig.normals[2].y * theta_z.sin(),
                trig.normals[2].x * theta_z.sin() + trig.normals[2].y * theta_z.cos(),
                trig.normals[2].z,
            );
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

            // Now rotate the individual points
            for point in trig.points.iter_mut() {
                // Rotate on x.
                *point = Vec3::new(
                    point.x,
                    point.y * theta_x.cos() - point.z * theta_x.sin(),
                    point.y * theta_x.sin() + point.z * theta_x.cos(),
                );

                // Rotate on y.
                *point = Vec3::new(
                    point.x * theta_y.cos() + point.z * theta_y.sin(),
                    point.y,
                    -point.x * theta_y.sin() + point.z * theta_y.cos(),
                );

                // Rotate on z.
                *point = Vec3::new(
                    point.x * theta_z.cos() - point.y * theta_z.sin(),
                    point.x * theta_z.sin() + point.y * theta_z.cos(),
                    point.z,
                );
            }
        }
    }
}

impl Mesh {
    /// Check if our mesh has been hit by a ray
    /// # Arguments
    /// * 'r' - The incoming ray
    /// # Returns
    /// * A hit struct containing the closest hit triangle and its properties
    pub fn hit(&self, r: Ray) -> Hit {
        // We want to store the closest hit triangle so we only draw those
        let mut closest_hit = Hit::new();

        // Loop through every triangle within the mesh
        for trig in self.triangles.iter() {
            // Check if the ray has hit any of the triangles within the mesh
            let hit: Hit = trig.hit(r);
            if hit.t > 0.0 {
                // Check if the hit triangle is closer than the current closest
                if hit.at.z > closest_hit.at.z {
                    closest_hit = hit;
                    closest_hit.material = self.material.clone();
                }
            }
        }
        closest_hit
    }
}

#[derive(Debug, Error)]
pub enum MeshError {
    #[error("Failed to open mesh file: {0}")]
    OpenFile(#[from] std::io::Error),

    #[error("Failed to parse mesh data: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Failed to parse mesh data: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
}

/// Load an OBJ mesh
/// # Arguments
/// * 'path' - Path of an OBJ file
/// * 'smooth' - Boolean which states if the mesh is smooth shaded
/// # Returns
/// * A mesh and all of its triangles, including a default material
pub fn load_mesh(path: &str, smooth: bool) -> Result<Mesh, MeshError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Will store all vertices, normals, and triangles
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[f64; 3]> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    // For each line in the obj file
    for line in reader.lines() {
        let line = line?;

        // Split by white space
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.is_empty() {
            continue;
        }

        // If the first word is v, this means vertex
        if words[0] == "v" {
            // Push the vertex into the vertices vec
            vertices.push([words[1].parse()?, words[2].parse()?, words[3].parse()?]);

        // If it's a vertex normal
        } else if words[0] == "vn" {
            normals.push([words[1].parse()?, words[2].parse()?, words[3].parse()?]);

        // If it's a face
        } else if words[0] == "f" {
            // Split by '/'
            let v1: Vec<&str> = words[1].split('/').collect();
            let v2: Vec<&str> = words[2].split('/').collect();
            let v3: Vec<&str> = words[3].split('/').collect();

            // Match the points and the normals
            let p1: usize = v1[0].parse()?;
            let n1: usize = v1[2].parse()?;

            let p2: usize = v2[0].parse()?;
            let n2: usize = v2[2].parse()?;

            let p3: usize = v3[0].parse()?;
            let n3: usize = v3[2].parse()?;

            // Create a new triangle
            let mut trig = Triangle::new(
                Vec3::new(
                    vertices[p1 - 1][0],
                    vertices[p1 - 1][1],
                    vertices[p1 - 1][2],
                ),
                Vec3::new(
                    vertices[p2 - 1][0],
                    vertices[p2 - 1][1],
                    vertices[p2 - 1][2],
                ),
                Vec3::new(
                    vertices[p3 - 1][0],
                    vertices[p3 - 1][1],
                    vertices[p3 - 1][2],
                ),
                Vec3::new(normals[n1 - 1][0], normals[n1 - 1][1], normals[n1 - 1][2]),
            );

            // If it's a smoothly shaded mesh, add to the triangle normals
            if smooth {
                trig.smooth = true;

                trig.normals = [
                    Vec3::new(normals[n1 - 1][0], normals[n1 - 1][1], normals[n1 - 1][2]),
                    Vec3::new(normals[n2 - 1][0], normals[n2 - 1][1], normals[n2 - 1][2]),
                    Vec3::new(normals[n3 - 1][0], normals[n3 - 1][1], normals[n3 - 1][2]),
                ];
            }

            // Push the triangle to the vec
            triangles.push(trig);
        }
    }

    // Return the new mesh based on the triangles
    Ok(Mesh::new_mesh(triangles))
}
