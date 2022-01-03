#[cfg(test)]
mod tests {
    fn count_bits(n: i64) -> u32 {
        let mut mn = n;
        let mut count = 0 as u32;
        while mn != 0 {
            if mn % 2 != 0 {
                count += 1
            }

            mn = mn >> 1
        }

        count
    }


    #[test]
    fn returns_expected() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
    }
}
