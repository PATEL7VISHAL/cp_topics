/// # 1752. Check if Array Is Sorted and Rotated
/// Given an array nums, return true if the array was originally sorted in non-decreasing order,
/// then rotated some number of positions (including zero). Otherwise, return false.
///
/// There may be duplicates in the original array.
///
/// Note: An array A rotated by x positions results in an array B of the same length such
/// that A\[i] == B\[(i+x) % A.length], where % is the modulo operati
/// # exmaple
/// Example 1:
/// Input: nums = \[3,4,5,1,2]
/// Output: true
/// Explanation: \[1,2,3,4,5] is the original sorted array.
/// You can rotate the array by x = 3 positions to begin on the the element of value 3: \[3,4,5,1,2].
pub fn check(nums: Vec<i32>) -> bool {
    let mut count = 0;
    let mut iter1 = nums.iter();
    let mut iter2 = nums.iter();
    iter2.next();

    while let (Some(i), Some(j)) = (iter1.next(), iter2.next()) {
        if i > j {
            count += 1;
        }
    }

    if nums[nums.len() - 1] > nums[0] {
        count += 1;
    }
    if count > 1 {
        return false;
    }

    return true;
}

#[test]
fn t_1752() {
    let nums = vec![3, 4, 5, 1, 2];
    println!("nums: {nums:?}");
    let ans = check(nums);
    println!("ans: {ans}");
}
