use std::ops;

#[derive(Clone)]
pub struct Limits {
    pub min: f64,
    pub max: f64,
}

impl Limits {
    pub fn new(min: f64, max: f64) -> Self {
        Limits {
            min: min.min(max),
            max: max.max(min),
        }
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn intersects(&self, other: &Limits) -> bool {
        let highest_min = self.min.max(other.min);
        let lowest_max = self.max.min(other.max);
        highest_min <= lowest_max
    }
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            min: 0.0,
            max: 0.0,
        }
    }
}

impl ops::AddAssign for Limits {
    fn add_assign(&mut self, rhs: Self) {
        self.min = self.min.min(rhs.min);
        self.max = self.max.max(rhs.max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let limits = Limits::new(0.0, 1.0);

        assert!(limits.contains(0.0));
        assert!(limits.contains(0.5));
        assert!(limits.contains(1.0));
        assert!(!limits.contains(-0.1));
        assert!(!limits.contains(1.1));
    }

    #[test]
    fn intersects() {
        let limits = Limits::new(0.0, 1.0);

        let other = Limits::new(0.25, 0.75);
        assert!(limits.intersects(&other));
        assert!(other.intersects(&limits));

        let other = Limits::new(-0.5, 1.5);
        assert!(limits.intersects(&other));
        assert!(other.intersects(&limits));

        let other = Limits::new(0.5, 1.5);
        assert!(limits.intersects(&other));
        assert!(other.intersects(&limits));

        let other = Limits::new(-0.5, 0.5);
        assert!(limits.intersects(&other));
        assert!(other.intersects(&limits));

        let other = Limits::new(1.5, 2.0);
        assert!(!limits.intersects(&other));
        assert!(!other.intersects(&limits));
    }
}
