fn get_middle(s: &str) -> &str {
    let s_len = s.len();
    let s_mid_index = s_len / 2;
    let rs;
    if s_len % 2 == 0 {
        rs = &s[s_mid_index - 1..s_mid_index + 1];
    } else {
        rs = &s[s_mid_index..s_mid_index + 1];
    }
    println!("{}", rs);
    rs
}

#[test]
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}