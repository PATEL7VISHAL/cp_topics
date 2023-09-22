fn my_sqrt(x: i32) -> i32 {
    let mut start = 0;
    let mut end = x;
    let mut end = x.checked_add(1).unwrap_or(x) / 2;
    let mut mid = 0;

    while start <= end {
        mid = start + (end - start) / 2;
        if (mid as u128 * mid as u128) > x as u128 {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    return end;
}

//PERF: OTHER:
/// More Presise ans
fn sqrt_presise(x: i32, round_ans: i32, presise: u8) -> f64 {
    let x = x as f64;
    let mut ans = round_ans as f64;

    for i in 1..=presise {
        let mut right = 1f64 / 10u64.pow(i as u32) as f64;
        let increamentor = right;
        let range = (right * 10f64);
        // println!("increamenter: {increamentor}");

        while right < range {
            let multiplyer = ans as f64 + increamentor;
            // println!("multiplyer :{multiplyer}");
            // println!("right: {right}");
            let tmp = multiplyer * multiplyer;
            if tmp > x {
                break;
            }
            ans = multiplyer;

            right += increamentor;
        }
    }

    ans
}

#[test]
fn t_69() {
    let x = 2147483647;
    // let x = 10;
    let ans = my_sqrt(x);
    println!("{x} => {ans}");
    let ans2 = sqrt_presise(x, ans, 5);
    println!("more presise => {ans2}");
}
