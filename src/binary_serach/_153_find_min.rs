/// # 153. Find Minimum in Rotated Sorted Array
/// Suppose an array of length n sorted in ascending order is rotated between 1 and n times.
/// For example, the array nums = \[0,1,2,4,5,6,7] might become:
///
/// [4,5,6,7,0,1,2] if it was rotated 4 times.
/// [0,1,2,4,5,6,7] if it was rotated 7 times.
/// Notice that rotating an array \[a\[0], a\[1], a\[2], ..., a\[n-1]] 1 time results in the array
/// `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.
///
/// Given the sorted rotated array nums of unique elements, return the minimum element of this array.
///
/// You must write an algorithm that runs in O(log n) time.
///
/// # Example 1:
/// Input: nums = \[3,4,5,1,2]
/// Output: 1
/// Explanation: The original array was \[1,2,3,4,5] rotated 3 times.
fn find_min(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() as i32;
    let mut mid = 1;
    let mut element = nums[0];

    if end == 1 {
        return nums[0];
    }

    while start <= end {
        mid = start + (end - start) / 2;
        if mid == nums.len() as i32 {
            return nums[0];
        }

        if nums[mid as usize] < nums[0] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    if start == nums.len() as i32 {
        return nums[0];
    }
    nums[start as usize]
}

#[test]
fn t_154() {
    let nums = vec![3, 4, 5, 1, 2];
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    // let nums = vec![11, 13, 15, 17];
    // let nums = vec![2, 1];
    println!("nums: {nums:?}");
    let ans = find_min(nums);
    println!("ans: {ans}");
}
