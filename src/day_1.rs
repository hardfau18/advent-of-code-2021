pub fn count_incrment(measurements: &[usize]) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(first, second)| second > first)
        .count()
}
pub fn count_incrment_windowed(measurements: &[usize], window_size: usize) -> usize {
    measurements
        .windows(window_size)
        .zip(measurements.windows(window_size).skip(1))
        .filter(|(first_w, second_w)| first_w[0] < second_w[window_size - 1]) // you just need to compare firsts and last element since middle once are same
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_1_a() {
        let readings = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_incrment(&readings), 7);
    }
    #[test]
    fn day_1_b() {
        let readings = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_incrment_windowed(&readings, 3), 5);
    }
}
