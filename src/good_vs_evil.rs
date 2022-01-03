use std::cmp::Ordering;

const GOOD_WORTH: [usize; 6] = [1, 2, 3, 3, 4, 10];
const EVIL_WORTH: [usize; 7] = [1, 2, 2, 2, 3, 5, 10];

fn good_vs_evil(good: &str, evil: &str) -> String {
    let g: usize = good.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .zip(GOOD_WORTH.iter())
        .map(|(x, y)| x * y)
        .sum();
    let e: usize = evil.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .zip(EVIL_WORTH.iter())
        .map(|(x, y)| x * y)
        .sum();
    match g.cmp(&e) {
        Ordering::Equal => "Battle Result: No victor on this battle field".to_string(),
        Ordering::Greater => "Battle Result: Good triumphs over Evil".to_string(),
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good".to_string(),
    }
}


#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}