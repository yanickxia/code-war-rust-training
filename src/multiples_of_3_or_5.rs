fn solution(num: i32) -> i32 {
    let mut x = 0;
    for i in 1..(num) {
        if i % 3 == 0 {
            x += i;
        }
        if i % 5 == 0 {
            x += i;
        }
        if i % 15 == 0 {
            x -= i;
        }
    }
    x
}

#[test]
fn returns_expected() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(11), 33);
    assert_eq!(solution(6), 8);
}