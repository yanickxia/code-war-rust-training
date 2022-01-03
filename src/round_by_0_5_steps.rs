#[cfg(test)]
mod tests {
    fn solution(n: f64) -> f64 {
        (2.0 * n).round() / 2.0
    }

    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}
