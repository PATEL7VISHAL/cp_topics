use std::ops::Not;
/// # 442. Find All Duplicates in an Array [https://leetcode.com/problems/find-all-duplicates-in-an-array/]
/// Given an integer array nums of length n where all the integers of nums are in the
/// range [1, n] and each integer appears once or twice, return an array of all
/// the integers that appears twice.
///
/// You must write an algorithm that runs in O(n) time and uses only constant extra space.
fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut i = 0;
    let mut index = 0;
    let mut ans = Vec::with_capacity(nums.len() / 2);
    let a = 09;

    while i < nums.len() {
        index = nums[i].abs() as usize - 1;
        if nums[index] < 0 {
            ans.push(index as i32 + 1);
        }

        nums[index] = nums[index].not() + 1;
        i += 1;
    }
    ans
}

#[test]
fn t_442() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("nums: {nums:?}");
    let ans = find_duplicates(nums);
    println!("ans: {ans:?}");
}
