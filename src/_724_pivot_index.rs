/// `724. Find Pivot Index`
///Given an array of integers nums, calculate the pivot index of this array.
///The `pivot index` is the index where the sum of all the numbers strictly to
///the left of the index is equal to the sum of all the numbers strictly to the index's right.
///If the index is on the left edge of the array, then the left sum is 0 because
///there are no elements to the left. This also applies to the right edge of the array.
///Return the `leftmost pivot index`. If no such index exists, return -1.
///# Example
/// Input: nums = [1,7,3,6,5,6]
/// Output: 3
/// Explanation:
/// The pivot index is 3.
/// Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
/// Right sum = nums[4] + nums[5] = 5 + 6 = 11
fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut l_sum = 0;
    let mut r_sum = 0;

    while left < right {
        if l_sum < r_sum {
            l_sum += nums[left as usize];
            left += 1;
        } else {
            r_sum += nums[right as usize];
            right -= 1;
        }

        if l_sum == r_sum && left == right {
            return left;
        }
    }
    return -1;
}

#[test]
fn t_724() {
    // let nums = vec![1, 7, 3, 6, 5, 6];
    let nums = vec![2, 1, 1, -1, 2, 2];
    // let nums = vec![2, 1, -1];
    let nums = vec![-1, -1, -1, -1, -1, 0];
    print!("nums: {:?} => ", nums);
    let ans = pivot_index(nums);
    println!("ans: {ans}");
}
