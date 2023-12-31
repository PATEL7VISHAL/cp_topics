/// # 81. Search in Rotated Sorted Array II
/// There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).
///
/// Before being passed to your function, nums is rotated at an unknown pivot index k `(0 <= k < nums.length)`
/// such that the resulting array is `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]`
/// (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].
///
/// Given the array nums after the rotation and an integer target, return true if target is in nums, or
/// false if it is not in nums.
///
/// You must decrease the overall operation steps as much as possible.
/// # Example 1:
/// Input: nums = [2,5,6,0,0,1,2], target = 0
/// Output: true
fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut start = 0;
    let mut end = nums.len() as i32 - 1;
    let mut mid = 0;
    let mut element = 0;

    while start <= end {
        mid = start + (end - start) / 2;
        element = nums[mid as usize];

        if element == target {
            return true;
        }

        if element == nums[start as usize] && element == nums[end as usize] {
            start += 1;
            end -= 1;
            continue;
        }

        if nums[start as usize] <= element {
            if nums[start as usize] <= target && target <= element {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        } else {
            if element <= target && target <= nums[end as usize] {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
    }

    false
}

#[test]
fn t_81() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let nums = vec![1, 0, 1, 1, 1];
    let nums = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1];
    let target = 2;

    println!("nums: {nums:?} | target: {target}");
    let ans = search(nums, target);
    println!("ans: {ans}");
}
