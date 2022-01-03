#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    INCREMENT,
    DECREMENT,
    SQUARE,
    OUTPUT,
    UNKONW,
}


impl Operator {
    fn read(u: u8) -> Operator {
        if u == 'i' as u8 {
            return Operator::INCREMENT;
        } else if u == 'd' as u8 {
            return Operator::DECREMENT;
        } else if u == 's' as u8 {
            return Operator::SQUARE;
        } else if u == 'o' as u8 {
            return Operator::OUTPUT;
        }
        return Operator::UNKONW;
    }
}

fn parse(code: &str) -> Vec<i32> {
    let mut x = 0;
    let mut v = vec!();

    for c in code.as_bytes() {
        let opt = Operator::read(*c);
        if opt == Operator::OUTPUT {
            v.push(x);
        }

        x = change(x, opt);
    }

    v
}


fn change(i: i32, opt: Operator) -> i32 {
    let x;
    match opt {
        Operator::INCREMENT => x = i + 1,
        Operator::DECREMENT => x = i - 1,
        Operator::SQUARE => x = i * i,
        Operator::OUTPUT => x = i,
        Operator::UNKONW => x = i
    }
    x
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}