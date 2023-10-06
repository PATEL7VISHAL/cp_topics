/// # 209. Minimum Size Subarray Sum
/// Given an array of positive integers nums and a positive integer target,
/// return the minimal length of a subarray whose sum is greater than or equal
/// to target. If there is no such subarray, return 0 instead.
///
/// # Example
/// Input: target = 7, nums = \[2,3,1,2,4,3]
/// Output: 2
/// Explanation: The subarray \[4,3] has the minimal length under the problem constraint.
pub fn min_sub_array_len(target: i32, mut nums: Vec<i32>) -> i32 {
    let mut ans = i32::max_value();
    let mut start = 0;
    let mut end = 0;
    let mut sum = nums[0];

    while end < nums.len() - 1 {
        if sum < target {
            end += 1;
            sum += nums[end];
        } else {
            ans = ans.min((end + 1 - start) as i32);

            sum -= nums[start];
            start += 1;
            if start > end {
                end += 1;
            }
        }
    }
    while sum >= target && start < nums.len() {
        ans = ans.min((end + 1 - start) as i32);
        sum -= nums[start];
        start += 1;
    }

    if ans == i32::max_value() {
        return 0;
    }
    ans
}

#[test]
fn t_209() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    // let nums = vec![1, 4, 4];
    // let nums = vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12];
    // let nums = vec![1, 2, 3, 4, 5];
    // let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    // let nums = vec![1, 2, 3, 4, 5];
    let target = 7;

    let ans = min_sub_array_len(target, nums);
    println!("ans: {ans}");
}
