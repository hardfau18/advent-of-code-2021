pub fn count_incrment(measurements: &[usize]) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(first, second)| second > first)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn must_be_7() {
        let readings = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_incrment(&readings), 7);
    }
}
