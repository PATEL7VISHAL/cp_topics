/// # 33. Search in Rotated Sorted Array
/// There is an integer array nums sorted in ascending order (with distinct values).
///
/// Prior to being passed to your function, nums is possibly rotated at an unknown
/// pivot index k (1 <= k < nums.length) such that the resulting array is `[nums[k], nums[k+1],`
/// `..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]` (0-indexed). For example, [0,1,2,4,5,6,7]
/// might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
///
/// Given the array nums after the possible rotation and an integer target, return the index
/// of target if it is in nums, or -1 if it is not in nums.
///
/// You must write an algorithm with O(log n) runtime complexity.
/// # Example 1:
/// Input: nums = [4,5,6,7,0,1,2], target = 0
/// Output: 4
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0 as i32;
    let mut end = nums.len() as i32 - 1;
    let mut mid = 0;
    let is_in_left = target >= nums[0];

    while start <= end {
        mid = start + (end - start) / 2;
        let element = nums[mid as usize];

        if element == target {
            return mid;
        }

        if is_in_left {
            if element < nums[0] {
                end = mid - 1;
            } else {
                if target > element {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        } else {
            if element >= nums[0] {
                start = mid + 1;
            } else {
                if target > element {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }
    }

    return -1;
}
