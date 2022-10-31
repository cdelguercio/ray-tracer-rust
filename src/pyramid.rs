use std::f64::consts;

use crate::{angle, bounds, quaternion, vector3};

pub struct Pyramid {
    origin: vector3::Vector3,
    direction: vector3::Vector3,
    vertical: vector3::Vector3,
    horizontal: vector3::Vector3,
    vertical_dot: f64,
    horizontal_dot: f64,
}

impl Pyramid {
    pub fn new(position: vector3::Vector3, rotation: quaternion::Quaternion, vertical_fov: angle::Angle, horizontal_fov: angle::Angle) -> Self {
        let vertical = rotation * vector3::Vector3::new(0.0, (vertical_fov.get_radians() / 2.0).sin(), 0.0);
        let horizontal = rotation * vector3::Vector3::new((horizontal_fov.get_radians() / 2.0).sin(), 0.0, 0.0);
        Pyramid {
            origin: position,
            direction: rotation * vector3::Vector3::new(0.0, 0.0, 1.0),
            vertical,
            horizontal,
            vertical_dot: vertical.norm_squared(),
            horizontal_dot: horizontal.norm_squared(),
        }
    }

    pub fn contains_point(&self, point: &vector3::Vector3) -> bool {
        let test = vector3::Vector3::normalized_sub(point, &self.origin);

        vector3::Vector3::dot(&test, &self.direction) > 0.0 &&
            vector3::Vector3::dot(&test, &self.vertical).abs() <= self.vertical_dot &&
            vector3::Vector3::dot(&test, &self.horizontal).abs() <= self.horizontal_dot
    }

    pub fn intersects_bounds(&self, bounds: &bounds::Bounds) -> bool {
        let min_test = vector3::Vector3::normalized_sub(&bounds.minimum(), &self.origin);
        let max_test = vector3::Vector3::normalized_sub(&bounds.maximum(), &self.origin);

        let vertical_norm = self.vertical.norm();
        let horizontal_norm = self.horizontal.norm();

        let min_vertical = vector3::Vector3::dot(&(min_test * vertical_norm), &self.vertical);
        let max_vertical = vector3::Vector3::dot(&(max_test * vertical_norm), &self.vertical);

        if (min_vertical > self.vertical_dot && max_vertical > self.vertical_dot) ||
            (min_vertical < -self.vertical_dot && max_vertical < -self.vertical_dot) {
            return false;
        }

        let min_horizontal = vector3::Vector3::dot(&(min_test * horizontal_norm), &self.horizontal);
        let max_horizontal = vector3::Vector3::dot(&(max_test * horizontal_norm), &self.horizontal);

        if (min_horizontal > self.horizontal_dot && max_horizontal > self.horizontal_dot) ||
            (min_horizontal < -self.horizontal_dot && max_horizontal < -self.horizontal_dot) {
            return false;
        }

        true
    }

    pub fn relative_position_in_frustum(&self, point: &vector3::Vector3) -> vector3::Vector3 {
        let test = vector3::Vector3::normalized_sub(point, &self.origin);

        vector3::Vector3::new(
            ((vector3::Vector3::dot(&test, &self.horizontal) / self.horizontal_dot) + 1.0) / 2.0,
            ((vector3::Vector3::dot(&test, &self.vertical) / self.vertical_dot) + 1.0) / 2.0,
            vector3::Vector3::dot(&test, &self.direction),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use rstest::*;

    #[test]
    fn test_new() {
        let position = vector3::Vector3::new(0.0, 0.0, 0.0);
        let rotation = quaternion::Quaternion::new(0.0, 0.0, 0.0, 1.0);
        let vertical_fov = angle::Angle::from_degrees(90.0);
        let horizontal_fov = angle::Angle::from_degrees(90.0);
        let pyramid = Pyramid::new(position, rotation, vertical_fov, horizontal_fov);
        assert_eq!(pyramid.origin, position);
        assert_eq!(pyramid.direction, vector3::Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(pyramid.vertical, vector3::Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(pyramid.horizontal, vector3::Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(pyramid.vertical_dot, 1.0);
        assert_eq!(pyramid.horizontal_dot, 1.0);
    }

    #[test]
    fn contains_point() {
        let pyramid = Pyramid::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0),
            angle::Angle::from_degrees(90.0),
            angle::Angle::from_degrees(90.0),
        );

        println!("{:?}", pyramid.direction);

        assert!(pyramid.contains_point(&vector3::Vector3::new(0.0, 0.0, 1.0)));
        assert!(pyramid.contains_point(&vector3::Vector3::new(0.0, 0.9, 1.0)));
        assert!(pyramid.contains_point(&vector3::Vector3::new(0.9, 0.0, 1.0)));
        assert!(pyramid.contains_point(&vector3::Vector3::new(0.9, 0.9, 1.0)));
        assert!(pyramid.contains_point(&vector3::Vector3::new(0.0, 0.0, 10.0)));
        assert!(pyramid.contains_point(&vector3::Vector3::new(9.0, 9.0, 10.0)));

        assert!(!pyramid.contains_point(&vector3::Vector3::new(0.0, 1.1, 1.0)));
        assert!(!pyramid.contains_point(&vector3::Vector3::new(1.1, 0.0, 1.0)));
        assert!(!pyramid.contains_point(&vector3::Vector3::new(0.0, 11.0, 10.0)));
        assert!(!pyramid.contains_point(&vector3::Vector3::new(11.0, 0.0, 10.0)));
        assert!(!pyramid.contains_point(&vector3::Vector3::new(0.0, 0.0, -0.1)));
        assert!(!pyramid.contains_point(&vector3::Vector3::new(11.0, 11.0, 10.0))); // TODO: this one should pass
    }

    #[test]
    fn intersects_bounds() {
        let pyramid = Pyramid::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0),
            angle::Angle::from_degrees(90.0),
            angle::Angle::from_degrees(90.0),
        );

        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(1.0, 1.0, 1.0))));
        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(-1.0, 1.0, 1.0))));
        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.9, 0.9, 0.9), vector3::Vector3::new(1.1, 1.1, 1.1))));
        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(-0.9, -0.9, -0.9), vector3::Vector3::new(-1.1, -1.1, -1.1))));
        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(1.0, 0.0, 0.0))));
        assert!(pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(-1.0, 0.0, 0.0))));

        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, -0.1), vector3::Vector3::new(0.0, 0.0, -0.2))));// TODO: this one onwards
        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(0.0, 0.0, -2.0))));
        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(0.0, 2.0, 0.0))));
        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(0.0, -2.0, 0.0))));
        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(2.0, 0.0, 0.0))));
        assert!(!pyramid.intersects_bounds(&bounds::Bounds::from_vectors(vector3::Vector3::new(0.0, 0.0, 0.0), vector3::Vector3::new(-2.0, 0.0, 0.0))));
    }

    #[fixture]
    pub fn fixture() -> Pyramid {
        Pyramid::new(
            vector3::Vector3::new(0.0, 0.0, 0.0),
            quaternion::Quaternion::from_roll_pitch_yaw(0.0, 0.0, 0.0),
            angle::Angle::from_degrees(90.0),
            angle::Angle::from_degrees(90.0),
        )
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), (0.5, 0.5, 0.0))]
    #[case((0.0, 0.0, 0.5), (0.5, 0.5, 0.0))]
    #[case((0.0, 0.0, 1.0), (0.5, 0.5, 0.0))]
    #[case((1.0, 0.0, 0.0), (0.0, 0.5, 0.0))]
    #[case((1.0, 0.0, 1.0), (0.0, 0.5, 0.0))]
    #[case((0.0, 1.0, 0.0), (0.5, 0.0, 0.0))]
    #[case((0.0, 1.0, 1.0), (0.5, 0.0, 0.0))]
    #[case((1.0, 1.0, 0.0), (0.0, 0.0, 0.0))]
    #[case((1.0, 1.0, 1.0), (0.0, 0.0, 0.0))]
    #[case((10.0, 10.0, 10.0), (0.0, 0.0, 0.0))]
    fn relative_position_in_frustum(fixture: Pyramid, #[case] position: (f64, f64, f64), #[case] expected: (f64, f64, f64)) {
        let test_position = fixture.relative_position_in_frustum(&vector3::Vector3::new(position.0, position.1, position.2));
        assert_approx_eq!(test_position.get_x(), expected.0, 1e-3);
        assert_approx_eq!(test_position.get_y(), expected.1, 1e-3);
        assert_approx_eq!(test_position.get_z(), expected.2, 1e-3);
    }
}
