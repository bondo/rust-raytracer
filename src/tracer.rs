use rand::Rng;
use rayon::prelude::*;
use std::{io::Write, sync::Arc};

use crate::{
    vec3::{barycentric, unit_vector},
    Camera, DrawingMode, Material, Mesh, Ray, RayTracerConfig, Result, Vec3, World,
};

pub struct RayTracer {
    camera: Camera,
    config: RayTracerConfig,
    world: World,
}

impl Default for RayTracer {
    fn default() -> Self {
        Self::new(RayTracerConfig::default())
    }
}

impl RayTracer {
    pub(crate) fn new(config: RayTracerConfig) -> RayTracer {
        let aspect_ratio: f64 = (config.width as f64) / (config.height as f64);
        RayTracer {
            camera: Camera::with_aspect_ratio(aspect_ratio),
            config,
            world: World::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.world.add(mesh);
    }

    pub fn run_sequential(&self, output: &mut dyn Write) -> Result<()> {
        self.write_header(output)?;

        // Loop through our image
        for y in (0..self.config.height).rev() {
            for x in 0..self.config.width {
                let pixel = self.generate_pixel(x, y);
                self.write_color(output, pixel)?;
            }
        }

        Ok(())
    }

    pub fn run_parallel(&self, output: &mut dyn Write) -> Result<()> {
        self.write_header(output)?;

        let this = Arc::new(self);

        // Loop through our image
        let pixels: Vec<Vec3> = (0..this.config.height)
            .into_par_iter()
            .rev()
            .flat_map(|y| {
                let this = this.clone();
                (0..this.config.width)
                    .into_par_iter()
                    .map(move |x| this.generate_pixel(x, y))
            })
            .collect();

        for pixel in pixels {
            this.write_color(output, pixel)?;
        }

        Ok(())
    }

    fn generate_pixel(&self, x: u32, y: u32) -> Vec3 {
        match self.config.mode {
            DrawingMode::Colors | DrawingMode::Normals => {
                let u = x as f64 / (self.config.width - 1) as f64;
                let v = y as f64 / (self.config.height - 1) as f64;

                // Calculate the ray based on the pixel we are on
                let r = Ray::new(
                    self.camera.origin,
                    self.camera.lower_left_corner
                        + (self.camera.horizontal * u)
                        + (self.camera.vertical * v)
                        - self.camera.origin,
                );

                // Send over the ray and world and figure out the color we should draw for this pixel
                self.ray_color(r, self.config.max_depth)
            }
            DrawingMode::Samples(samples) => {
                let mut color = Vec3::new(0.0, 0.0, 0.0);

                // Loop for however many samples we want to take
                for _ in 0..samples {
                    // Need random number generator from 0-1
                    let mut rng = rand::thread_rng();

                    // Calculate u&v based on our random samples
                    let u: f64 = ((x) as f64 + rng.gen::<f64>()) / (self.config.width - 1) as f64;
                    let v: f64 = (y as f64 + rng.gen::<f64>()) / (self.config.height - 1) as f64;

                    let r = Ray::new(
                        self.camera.origin,
                        self.camera.lower_left_corner
                            + (self.camera.horizontal * u)
                            + (self.camera.vertical * v)
                            - self.camera.origin,
                    );

                    // Add to the color for each sample, essentially creating an average color
                    color = color + self.ray_color(r, self.config.max_depth);
                }
                color
            }
        }
    }

    /// Calculate color based on the ray and whatever it hits
    /// # Arguments
    /// * 'r' - Ray to cast
    /// * 'depth' - Number of bounces a ray can have
    /// # Returns
    /// * Vec3 which contains r,g,b values in the x,y,z position of the vector
    fn ray_color(&self, r: Ray, depth: u32) -> Vec3 {
        // Check if our ray hits any object
        // Hit will contain details about the object the ray hit
        let hit = self.world.hit(r);

        // Match the drawing mode
        match self.config.mode {
            DrawingMode::Colors => {
                // Hit.t will be > 0 if the ray actually hit something
                if hit.t > 0.0 {
                    // Simply return the color of what the ray hit
                    return hit.material.get_albedo();
                }
            }
            DrawingMode::Normals => {
                if hit.t > 0.0 {
                    // If the mesh is smooth shaded, we need to calculate the interpolated normal
                    let n = if hit.triangle.smooth {
                        // Calculate the barycentric coordinates
                        let bary = barycentric(hit.clone());

                        // Calculate the interpolated normal
                        unit_vector(
                            hit.triangle.normals[0] * bary.x
                                + hit.triangle.normals[1] * bary.y
                                + hit.triangle.normals[2] * bary.z,
                        )
                    } else {
                        // Mesh isn't smooth shaded, simply return its single normal
                        hit.triangle.normal
                    };

                    // Calculate color based on the normal
                    return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
                }
            }
            DrawingMode::Samples(_) => {
                // Samples mode recursively calls ray_color
                // Quit recursively calling if we've bounced our last bounce
                if depth == 0 {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
                if hit.t > 0.0 {
                    // Will store the new ray, i.e. we bounce off the object and have a new ray based on the bounce
                    let mut scattered =
                        Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

                    // Store the current color of whatever the ray bounces off
                    let mut attenuation = Vec3::new(0.0, 0.0, 0.0);

                    // Make sure we correctly scatter based on the objects material
                    if hit
                        .material
                        .scatter(r, hit.clone(), &mut attenuation, &mut scattered)
                    {
                        // Recursively call, multiplying the current color
                        return attenuation * self.ray_color(scattered, depth - 1);
                    }
                }
            }
        }

        // This code generates the blueish gradient background
        let n = r.direction;
        let t = (n.y + 1.0) * 0.5;

        // Typical interpolation
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + Vec3::new(0.5, 0.7, 1.0) * t
    }

    fn write_header(&self, output: &mut dyn Write) -> Result<()> {
        output.write_all(
            format!("P3\n{} {}\n255\n", self.config.width, self.config.height).as_bytes(),
        )?;

        Ok(())
    }

    /// Write a color to the output file
    /// # Arguments
    /// * 'output' - PPM file we write to
    /// * 'color' - Color which we wish to write
    fn write_color(&self, output: &mut dyn Write, color: Vec3) -> Result<()> {
        let r: u32;
        let g: u32;
        let b: u32;
        match self.config.mode {
            DrawingMode::Colors | DrawingMode::Normals => {
                // If we're drawing colors/normals, simply multiply by 255
                // Input color is 0-1, so multiply by 255 to make it in a range of 0-255
                r = (color.x * 255.0) as u32;
                g = (color.y * 255.0) as u32;
                b = (color.z * 255.0) as u32;
            }
            DrawingMode::Samples(samples) => {
                // Perform gamma correction
                r = ((color.x * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
                g = ((color.y * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
                b = ((color.z * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
            }
        }
        if r > 255 || g > 255 || b > 255 {
            panic!("Color value out of range");
        }

        output.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;

        Ok(())
    }
}
