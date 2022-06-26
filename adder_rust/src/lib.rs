/// Sum the two given numbers
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Sum two random numbers
pub fn add_two_random() -> usize {
    add(rand::random(), rand::random())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
