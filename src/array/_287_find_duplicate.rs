use std::ops::Not;

/// # 287. Find the Duplicate Number [https://leetcode.com/problems/find-the-duplicate-number/]
///Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
///
/// There is only one repeated number in nums, return this repeated number.

/// You must solve the problem without modifying the array nums and uses only constant extra space.
/// # Example
/// Input: nums = [1,3,4,2,2]
/// Output: 2
///
///Only work for only two time duplicated number
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut fast = 0;
    let mut slow = 0;

    while true {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if fast == slow {
            break;
        }
    }

    slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

pub fn find_duplicate2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut i = 0;
    let mut index = 0;

    while i < nums.len() {
        index = nums[i].abs() as usize;
        if nums[index] < 0 {
            return index as i32;
        }

        nums[index] = nums[index].not() + 1;
        i += 1;
    }

    println!("nums: {:?}", nums);
    return -1;
}

#[test]
fn t_287() {
    let nums = vec![1, 3, 4, 2, 2]; // | ans = 2
    let nums = vec![3, 1, 3, 4, 2]; // | ans = 3
    println!("Nums: {nums:?}");
    // let ans = find_duplicate(nums);
    let ans = find_duplicate2(nums);
    println!("ans: {ans}");
}
