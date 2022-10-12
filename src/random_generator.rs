use rand::Rng;
use rand::rngs;

pub struct RandomGenerator {
    rng: rngs::ThreadRng,
}

impl RandomGenerator {
    fn new() -> RandomGenerator {
        RandomGenerator {
            rng: rand::thread_rng(),
        }
    }
    fn value(&mut self, scale: f64) -> f64 {
        self.rng.gen_range(0.0..scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_number() {
        let mut g = RandomGenerator::new();
        let v = g.value(1.0);
        assert!(v >= 0.0 && v < 1.0);
    }
}
