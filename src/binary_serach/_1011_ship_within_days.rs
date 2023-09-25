/// # 1011. Capacity To Ship Packages Within D Days [https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/]
/// A conveyor belt has packages that must be shipped from one port to another within days days.
///
/// The `ith` package on the conveyor belt has a weight of `weights[i]`. Each day, we load the ship with packages
/// on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.
///
/// Return the least weight capacity of the ship that will result in
/// all the packages on the conveyor belt being shipped within days days.
/// # Example 1:
/// Input: weights = [1,2,3,4,5,6,7,8,9,10], days = 5
/// Output: 15
/// Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
/// 1st day: 1, 2, 3, 4, 5
/// 2nd day: 6, 7
/// 3rd day: 8
/// 4th day: 9
/// 5th day: 10
///
/// Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages
/// into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut require_days = 0;
    let mut start: i64 = 0;
    let mut end: i64 = 0;
    let mut mid = 0;
    let mut weights = weights;

    for i in weights.iter() {
        if *i as i64 > start {
            start = *i as i64;
        }
        end += *i as i64;
    }

    if weights.len() == days as usize {
        return start as i32;
    }
    if weights.len() == 1 {
        return end as i32;
    }

    while start <= end {
        mid = start + (mid - start) / 2;

        require_days = ship_packages(weights.as_ref(), mid);
        println!("weight: {mid} days: {require_days}");
        if require_days <= days {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    start as i32
}

// return day will be token to ship packages under maximum weight
fn ship_packages(weights: &[i32], max_weight: i64) -> i32 {
    let mut days = 1;
    let mut sum = 0;

    for i in weights {
        let i = *i;
        if sum + i as i64 > max_weight {
            days += 1;
            sum = i as i64;
        } else {
            sum += i as i64;
        }
    }

    days
}

#[test]
fn t_1011() {
    let mut weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Days: 5  | ans: 15
                                                           // let mut weights = vec![1, 2, 3, 1, 1]; // Days: 4 | ans: 3
                                                           // let mut weights = vec![3, 2, 2, 4, 1, 4]; // days: 3 | ans: 6
    let days = 5;
    println!("weight: {weights:?}  | days: {days}");
    let ans = ship_within_days(weights, days);
    println!("ans: {ans}");

    // let days = ship_packages(&weights, 15);
    // println!("days: {days}");
    // return ();
}
