fn testequal(n: i32, exp: i64) -> () {
    assert_eq!(exp, fcn(n))
}

#[test]
fn basics() {
    testequal(17, 131072);
    testequal(21, 2097152);
}


fn fcn(n: i32) -> i64 {
    1i64 << n
}

fn element(n: i32, cache: &mut Vec<i64>) -> i64 {
    for i in 2..(n + 1) {
        let x = i as usize;
        let y = (6 * cache[x - 2] * cache[x - 1]) / (5 * cache[x - 2] - cache[x - 1]);
        cache.push(y);
    }

    cache[(n) as usize]
}