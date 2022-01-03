fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }
    let mut rs: Vec<i32> = vec![];

    let mut x = std::cmp::max(lng, wdth);
    let mut y = std::cmp::min(lng, wdth);

    loop {
        for _i in 0..(x / y) {
            rs.push(y);
        }
        let z = x % y;
        x = y;
        y = z;

        if y == 0 {
            break;
        }

        if y == 1 {
            for _i in 0..(x / y) {
                rs.push(1);
            }
            break;
        }
    }


    Some(rs)
}


fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
//    testing(10, 3, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}