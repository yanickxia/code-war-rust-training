#[cfg(test)]
mod tests {
    fn camel_case(str: &str) -> String {
        let chars: Vec<char> = str.chars().collect();
        let mut new_chars: Vec<char> = vec![];
        let mut is_to_upper = true;
        for c in chars {
            if c == ' ' {
                is_to_upper = true;
            } else {
                if is_to_upper {
                    new_chars.push(c.to_ascii_uppercase());
                } else {
                    new_chars.push(c);
                }
                is_to_upper = false;
            }
        }
        let rs = new_chars.into_iter().collect();
        println!("{}", rs);

        return rs;
    }

    fn camel_case_2(str: &str) -> String {
        return str.split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|&s| [&s[..1].to_uppercase(), &s[1..]].join(""))
            .collect::<Vec<_>>()
            .join("") as String;
    }

    #[test]
    fn sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(camel_case_2("test case"), "TestCase");
        assert_eq!(camel_case_2("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case_2("say hello "), "SayHello");
        assert_eq!(camel_case_2(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case_2(""), "");
    }
}
