/// # 2815. Max Pair Sum in an Array
/// You are given a 0-indexed integer array nums. You have to find the maximum sum
/// of a pair of numbers from nums such that the maximum digit in both numbers are equal.
/// Return the maximum sum or -1 if no such pair exists.
/// # Example 1:
/// Input: nums = [51,71,17,24,42]
/// Output: 88
/// Explanation:
/// For i = 1 and j = 2, nums[i] and nums[j] have equal maximum digits with a pair sum of 71 + 17 = 88.
/// For i = 3 and j = 4, nums[i] and nums[j] have equal maximum digits with a pair sum of 24 + 42 = 66.
/// It can be shown that there are no other pairs with equal maximum digits, so the answer is 88.
pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut counted_digits = [0; 10];
    let mut ans = -1;
    let mut tmp = 0;
    let mut max = 1;
    let mut max_number = -1;

    for i in nums.iter() {
        count_digits_from_number(&mut counted_digits, *i);
    }

    for i in nums.iter() {
        if get_total_occurance(&counted_digits, *i) > max {
            if *i > max_number {
                max_number = *i
            }
        }
    }

    println!("max_number: {max_number}");

    ans
}

fn count_digits_from_number(mut counted_digits: &mut [i32], mut number: i32) {
    let mut value = 0;
    let mut tmp = 0;
    let mut seen = [false; 10];

    while number != 0 {
        tmp = number % 10;
        // value = (value * 10) + tmp;
        number = number / 10;

        if !seen[tmp as usize] {
            seen[tmp as usize] = true;
            counted_digits[tmp as usize] += 1;
        }
    }
}

fn get_total_occurance(counted_digits: &[i32], mut number: i32) -> i32 {
    let mut tmp = 0;
    let mut seen = [false; 10];
    let mut occurance = 0;

    while number != 0 {
        tmp = number % 10;
        // value = (value * 10) + tmp;
        number = number / 10;

        if !seen[tmp as usize] {
            occurance += counted_digits[tmp as usize];
        }
    }

    occurance
}

#[test]
fn t_2815() {
    let nums = vec![51, 71, 17, 24, 42]; // ans = 88
                                         // let nums = vec![1, 2, 3, 4];
    println!("nums: {nums:?}");
    let ans = max_sum(nums);
    println!("ans: {ans}");

    // let mut counted_digits = [0; 10];
    // count_digits_from_number(&mut counted_digits, 1243);
    // println!("digits: {counted_digits:?}");
}
