fn alphabet_position(text: &str) -> String {
    let mut rs = String::new();

    let string = text.to_lowercase();

    let mut chars = string.chars();

    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }

        let x = c.unwrap();
        if x.is_ascii_alphabetic() {
            let i = (x as i32 - 'a' as i32 + 1).to_string();

            println!("{:?}, {:?}, diff {:?}", x, x as i32, (x as i32 - 'a' as i32 + 1));

            rs += i.as_str();
            rs += " ";
        }
    }

    if rs.len() == 0 {
        return rs;
    }

    rs[0..rs.len() - 1].to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
        );
        assert_eq!(
            alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
        );
    }

    #[test]
    fn test_symbols() {
        assert_eq!(
            alphabet_position(r#"-=!@#$%^&*()_+[];,./\{}:|<>? "#),
            "",
        );
    }
}
