fn high_and_low(numbers: &str) -> String {
    let f =
        |(mini, maxi), a| (std::cmp::min(mini, a), std::cmp::max(maxi, a));
        
    let answer = numbers.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::max_value(), i32::min_value()), f);

    format!("{} {}", answer.1, answer.0)
}


#[test]
fn sample_test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}