use std::cmp;

use crate::{angle, object, pixel_coords, pyramid, vector3};

pub struct Camera {
    m_width: usize,
    m_height: usize,
    m_aspect_ratio: f64,
    m_vertical_fov: angle::Angle,
    m_horizontal_fov: angle::Angle,
    object: object::Object,
}

impl Camera {
    pub fn new(width: usize, height: usize, vertical_fov: &angle::Angle) -> Self {
        let aspect_ratio = width as f64 / height as f64;
        let horizontal_fov = *vertical_fov * aspect_ratio;

        return Camera {
            m_width: width,
            m_height: height,
            m_aspect_ratio: aspect_ratio,
            m_vertical_fov: *vertical_fov,
            m_horizontal_fov: horizontal_fov,
            object: object::Object::new(),
        };
    }

    pub fn width(&self) -> usize {
        return self.m_width;
    }

    pub fn height(&self) -> usize {
        return self.m_height;
    }

    fn aspect_ratio(&self) -> f64 {
        return self.m_aspect_ratio;
    }

    fn vertical_fov(&self) -> angle::Angle {
        return self.m_vertical_fov;
    }

    fn horizontal_fov(&self) -> angle::Angle {
        return self.m_horizontal_fov;
    }

    fn set_vertical_fov(&mut self, vertical_fov: &angle::Angle) {
        self.m_vertical_fov = *vertical_fov;
        self.m_horizontal_fov = *vertical_fov * self.m_aspect_ratio;
    }

    fn set_from_render_configuration(&mut self, width: usize, height: usize) {
        if width == 0 || height == 0 {
            panic!("Cannot configure Camera with 0 width or 0 height");
        }

        self.m_width = width;
        self.m_height = height;
        self.m_aspect_ratio = width as f64 / height as f64;
        self.m_horizontal_fov = self.m_vertical_fov * self.m_aspect_ratio;
    }

    pub fn coord_for_point(&self, point: &vector3::Vector3) -> Option<pixel_coords::PixelCoords> {
        let frustum = pyramid::Pyramid::new(
            self.object.position(),
            self.object.rotation(),
            self.m_vertical_fov,
            self.m_horizontal_fov,
        );
        let position = frustum.relative_position_in_frustum(point);

        return if position.get_z() <= 0.0 || position.get_x() < 0.0 || position.get_x() > 1.0 || position.get_y() < 0.0 || position.get_y() > 1.0 { // TODO(cdelguercio): write a comment explaining this
            None
        } else {
            Some(
                pixel_coords::PixelCoords::new(
                    cmp::min(self.m_width - 1, (position.get_x() * self.m_width as f64).round() as usize),
                    cmp::min(self.m_height - 1, (position.get_y() * self.m_height as f64).round() as usize),
                )
            )
        }
    }

    pub fn pixel_direction(&self, pixel_coords: &pixel_coords::PixelCoords) -> vector3::Vector3 {
        let horizontal = pixel_coords.x as f64 / self.m_width as f64;
        let vertical = pixel_coords.y as f64 / self.m_height as f64;
        let horizontal_angle = self.m_horizontal_fov.get_radians().powi(2) * (horizontal - 0.5);
        let vertical_angle = self.m_vertical_fov.get_radians().powi(2) * (vertical - 0.5);

        let direction = vector3::Vector3::new(
            horizontal_angle.sin(),
            vertical_angle.sin(),
            horizontal_angle.cos() * vertical_angle.cos(),
        );

        self.object.rotation() * direction
    }

    pub fn position(&self) -> vector3::Vector3 {
        self.object.position()
    }

    pub fn forward(&self) -> vector3::Vector3 {
        self.object.forward()
    }
}

#[cfg(test)]
mod tests {
    use crate::quaternion;
    use super::*;

    #[test]
    fn test_coord_for_point() {
        let mut camera = Camera::new(100, 100, &angle::Angle::from_degrees(90.0));
        camera.object.transform.position = vector3::Vector3::new(0.0, 0.0, 0.0);
        camera.object.transform.rotation = quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0);

        // TODO(cdelguercio): write tests
    }

    #[test]
    fn test_pixel_direction() {
        let mut camera = Camera::new(100, 100, &angle::Angle::from_degrees(90.0));
        camera.object.transform.position = vector3::Vector3::new(0.0, 0.0, 0.0);
        camera.object.transform.rotation = quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0);

        // TODO(cdelguercio): write tests
    }
}
