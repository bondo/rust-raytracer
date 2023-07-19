use std::io::Write;

use crate::RayTracer;

/// Determine which drawing mode to use
/// * 'Colors' - Draw only the colors of the objects
/// * 'Normals' - Draw only the normals of the objects
/// * 'Samples' - Draw the final image with sampling
#[derive(Copy, Clone)]
pub enum DrawingMode {
    Colors,
    Normals,
    Samples(u32),
}

pub struct RayTracerConfig {
    pub(crate) mode: DrawingMode,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) max_depth: u32,
}

impl Default for RayTracerConfig {
    fn default() -> Self {
        Self {
            mode: DrawingMode::Samples(3),
            width: 480,
            height: 270,
            max_depth: 5,
        }
    }
}

impl RayTracerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(mut self, mode: DrawingMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn build(self, output: &mut dyn Write) -> RayTracer {
        RayTracer::new(self, output)
    }
}
