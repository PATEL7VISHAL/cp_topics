/// # 35. Search Insert Position
/// Given a sorted array of distinct integers and a target value, return the index
/// if the target is found. If not, return the index where it would be if it were inserted in order.
///
/// # Example 1:
/// Input: nums = [1,3,5,6], target = 5
/// Output: 2
///
/// You must write an algorithm with O(log n) runtime complexity.
/// # Example 2:
///
/// Input: nums = [1,3,5,6], target = 7
/// Output: 4
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = (nums.len() - 1) as i32;
    let mut mid = 0;
    let mut last = 0;
    let mut is_greater = false;

    while start <= end {
        mid = start + (end - start) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] > target {
            end = mid - 1;
            is_greater = false;
        } else {
            start = mid + 1;
            is_greater = true;
        }
        last = mid;
    }

    println!("is_greater:{is_greater}, last:{last}");

    if is_greater {
        return last + 1;
    } else {
        return last;
    }
}

#[test]
fn t_35() {
    let nums = vec![1, 3, 5, 6, 8, 9, 11];

    let target = 7;
    println!("nums: {nums:?} | target: {target}");

    let ans = search_insert(nums, target);
    println!("ans: {ans}");
}
