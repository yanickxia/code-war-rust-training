fn high_and_low(numbers: &str) -> String {
    let s = numbers.to_string();
    let numbs: Vec<i32> = s.split(" ")
        .map(|a| a.to_string().parse::<i32>().unwrap())
        .collect();

    let mut maxi = numbs[0];
    let mut mini = numbs[0];
    for n in numbs {
        if n > maxi {
            maxi = n
        }

        if n < mini {
            mini = n
        }
    }

    maxi.to_string() + " " + &mini.to_string()
}


#[test]
fn sample_test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}