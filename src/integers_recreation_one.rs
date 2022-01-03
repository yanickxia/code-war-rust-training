fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut rs = vec![];
    for i in m..n {
        let (is_match, x) = is_list_squared(i);
        if is_match {
            rs.push((i, x * x))
        }
    }
    rs
}

fn is_list_squared(m: u64) -> (bool, u64) {
    let divisors = list_divisors(m);
    let mut sum_rs = 0;
    for item in divisors {
        sum_rs += item * item;
    }

    let x = (sum_rs as f64).sqrt() as u64;
    (x * x == sum_rs, x)
}


fn list_divisors(m: u64) -> Vec<u64> {
    let mut rs = vec![];
    let mut limit = m;


    for i in 1..limit {
        if m % i == 0 {
            rs.push(i);
            limit = limit / i;
        }
    }

    rs.push(m);

    rs
}


fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
}