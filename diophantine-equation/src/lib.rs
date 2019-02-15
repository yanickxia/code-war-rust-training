#[cfg(test)]
mod tests {
    fn solequa(n: u64) -> Vec<(u64, u64)> {
        let mut res: Vec<(u64, u64)> = vec![];
        let mut i: u64 = 1;
        while i * i <= n {
            if n % i == 0 {
//             println!("{} {}", i, n / i);
                if (i + n/i) % 2 == 0 && (n/i - i) % 4 == 0 {
                    res.push(((i + n/i) / 2, (n/i - i) / 4));
                }
            }
            i += 1;
        }
        res
    }


    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]);
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
    }
}
