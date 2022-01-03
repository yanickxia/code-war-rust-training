use std::collections::HashSet;

fn trotter(n: i32) -> i32 {
    if n == 0 { return -1; }
    let (mut m, mut d, mut done) = (0, HashSet::new(), false);
    while !done {
        m = m + n;
        let mut i = m;
        while i > 0 {
            let seen = i % 10;
            d.insert(seen);
            i = i / 10;
        }
        if d.len() == 10 { done = true; }
    }
    return m;
}

#[test]
fn returns_expected() {
    assert_eq!(trotter(1692), 5076);
    assert_eq!(trotter(2), 90);
    assert_eq!(trotter(7), 70);
}