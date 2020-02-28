use std::cmp::min;
use std::option::Option;

fn main() {}

fn next_smaller_number(n: u64) -> Option<u64> {
    let n_str = n.to_string();
    for i in 1..n_str.len() {
        match try_switch_i_th(i, n) {
            Some(x) => {
                return Some(x);
            }
            _ => {}
        }
    }

    None
}

fn try_switch_i_th(i: usize, n: u64) -> Option<u64> {
    let numbers = number_to_array(n);

    let mut fst_numbers = numbers[0..(numbers.len() - i)].to_vec();
    let mut sub_numbers = numbers[(numbers.len() - i)..].to_vec();

    let x = *fst_numbers.last().unwrap();
    let mut y = 0;

    let is_last = fst_numbers.len() == 1;

    for j in (0..sub_numbers.len()).rev() {
        if sub_numbers[j] < x {
            if is_last && sub_numbers[j] == 0 {
                continue;
            }
            y = sub_numbers[j];
            sub_numbers[j] = x;
            let index = fst_numbers.len() - 1;
            fst_numbers[index] = y;
            sub_numbers.sort();
            sub_numbers.reverse();

            let mut vec = fst_numbers.to_vec();
            vec.append(&mut Vec::from(sub_numbers));
            return Option::Some(array_to_number(&vec));
        }
    }

    None
}

fn array_to_number(array: &Vec<u64>) -> u64 {
    let mut res: u64 = 0;
    for i in 0..array.len() {
        res *= 10;
        res += array[i]
    }

    res
}

fn number_to_array(n: u64) -> Vec<u64> {
    let mut res = vec![];

    let mut x = n;

    while x > 0 {
        let v = x % 10;
        x = x / 10;
        res.push(v);
    }
    res.reverse();
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
}
