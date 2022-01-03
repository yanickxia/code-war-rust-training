fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut cache: Vec<u64> = vec![0, 1];
    loop {
        let l = cache.len();
        let nex = cache[l - 1] + cache[l - 2];
        cache.push(nex);
        if nex > prod / 2 {
            break;
        }
    }


    let mut start = 0;
    loop {
        let (x, y) = (cache[start], cache[start + 1]);
        if x * y == prod {
            return (x, y, true);
        }

        let (x2, y2) = (cache[start + 1], cache[start + 2]);
        if x * y < prod && x2 * y2 > prod {
            return (x2, y2, false);
        }

        start += 1
    }
}


fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}