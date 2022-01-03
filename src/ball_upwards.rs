fn max_ball(v0: i32) -> i32 {
    return ((v0 as f64 / 3.6) / 0.981).round() as i32;
}

fn testequal(v0: i32, exp: i32) -> () {
    assert_eq!(exp, max_ball(v0))
}

#[test]
fn basic_tests() {
    testequal(37, 10);
    testequal(45, 13);
}