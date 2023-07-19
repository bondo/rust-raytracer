use crate::Vec3;

pub(crate) struct Camera {
    pub(crate) origin: Vec3,
    pub(crate) lower_left_corner: Vec3,
    pub(crate) horizontal: Vec3,
    pub(crate) vertical: Vec3,
}

impl Camera {
    pub(crate) fn with_aspect_ratio(viewport_aspect_ratio: f64) -> Self {
        // Viewport properties
        let viewport_height = 2.0;
        let viewport_width = viewport_aspect_ratio * viewport_height;

        // Camera properties
        let focal_length = 5.0;
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
