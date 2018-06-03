fn main() {
    println!("Hello, world!");
}

fn row_sum_odd_numbers(n: i64) -> i64 {
//    // your solution here
//    if n == 1 {
//        1
//    } else {
//        let mut s = 0 as i64;
//        for i in 1..n {
//            s += i;
//        }
//
//        let start = s * 2 + 1;
//        let mut all = 0 as i64;
//
//        for i in 1..(n+1) {
//            let s = start + (2 * (i - 1));
//            all += s;
//        }
//
//        all
//    }

    n.pow(3)
}


#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(3), 27);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}