use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

/// Sum the two given numbers
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Sum two random numbers
pub fn add_two_random() -> usize {
    let mut generator: SmallRng = SmallRng::from_entropy();
    add(generator.gen_range(1..500), generator.gen_range(1..500))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
