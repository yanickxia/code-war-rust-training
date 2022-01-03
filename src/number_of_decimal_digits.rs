fn digits(n: u64) -> usize {
    let mut u: usize = 0;
    let mut x = n;
    if x == 0 {
        return 1;
    }
    loop {
        if x == 0 {
            break;
        }

        x /= 10;
        u += 1;
    }
    u
}

#[test]
fn sample_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
