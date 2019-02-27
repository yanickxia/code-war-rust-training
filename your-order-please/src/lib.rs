#[cfg(test)]
mod tests {
    fn order(sentence: &str) -> String {
        let mut spited_string = sentence
            .split_whitespace()
            .collect::<Vec<&str>>();

        spited_string.sort_by(|a, b| find_the_number_in_string(a).cmp(&find_the_number_in_string(b)));
        spited_string.join(" ")
    }

    fn find_the_number_in_string(str: &str) -> u32 {
        return str.chars().find(|c| c.is_numeric()).unwrap().to_digit(10).unwrap_or(0);
    }


    #[test]
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
