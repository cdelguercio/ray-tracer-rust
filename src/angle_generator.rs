use std::f64::consts;

use crate::{quaternion, random_generator, vector3};

pub struct AngleGenerator {
    max_angle: f64,
    linearity: f64,
}

impl AngleGenerator {
    pub fn new(max_angle: f64, linearity: f64) -> Self {
        AngleGenerator {
            max_angle,
            linearity,
        }
    }

    // TODO(cdelguercio): have these functions return angle::Angle
    fn map(&self, value: f64) -> f64 {
        let mut mapped_value = value.max(-1.0).min(1.0);
        let value_result = value.powf(self.linearity) * self.max_angle;

        value_result
    }

    fn generate(&self, random_generator: &mut random_generator::RandomGenerator) -> f64 {
        let value = -1.0 + (random_generator.value(1.0) * 2.0);

        self.map(value)
    }

    pub fn generate_offset_vector(&self, center: &vector3::Vector3, random_generator: &mut random_generator::RandomGenerator) -> vector3::Vector3 {
        let theta = random_generator.value(consts::PI * 2.0);
        let angle = self.map(random_generator.value(1.0));
        let use_z = vector3::Vector3::dot(center, &vector3::UNIT_Y).abs() > 0.9;

        let cross_axis = if use_z {
            vector3::UNIT_Z
        } else {
            vector3::UNIT_Y
        };

        let offset = vector3::Vector3::cross(center, &cross_axis) * angle.sin();
        let rotation = quaternion::Quaternion::from_axis_angle(center, theta);
        let offset = rotation * offset;

        (*center * angle.cos()) + offset
    }
}

impl Default for AngleGenerator {
    fn default() -> Self {
        AngleGenerator {
            max_angle: consts::PI,
            linearity: 1.0,
        }
    }
}
