const MAPPINGS: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "A", "B", "C", "D", "E", "F"];

fn rgb(r: i32, g: i32, b: i32) -> String {
    return one_rgb(r) + one_rgb(g).as_str() + one_rgb(b).as_str();
}

fn one_rgb(i: i32) -> String {
    if i <= 0 {
        return "00".to_string();
    }

    if i >= 255 {
        return "FF".to_string();
    }

    let x = MAPPINGS[(i / 16) as usize];
    let y = MAPPINGS[(i - ((i / 16) * 16)) as usize];

    return x.to_string() + y;
}


macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}