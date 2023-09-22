use std::num::TryFromIntError;
/// Given an array of integers nums sorted in non-decreasing order,
/// find the starting and ending position of a given target value.
/// If target is not found in the array, return [-1, -1].
/// You must write an algorithm with O(log n) runtime complexity.
/// # Example
/// Input: nums = [5,7,7,8,8,10], target = 8
/// Output: [3,4]
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first = -1;
    let mut second = -1;
    let size = nums.len();

    let mut start: i32 = 0;
    let mut end: i32 = size as i32 - 1;
    let mut mid: i32 = 0;

    // finding first samller occurrence
    while start <= end {
        let mid = (start + end + 1) / 2;
        let element = nums[mid as usize];

        if element == target {
            first = mid;
        } else if element < target {
            start = mid + 1;
            continue;
        }
        end = mid - 1;
    }

    // finding last samller occurrence
    start = 0;
    end = size as i32 - 1;
    while start <= end {
        let mid = (start + end + 1) / 2;
        let element = nums[mid as usize];

        if element == target {
            second = mid;
        } else if element > target {
            end = mid - 1;
            continue;
        }
        start = mid + 1;
    }

    if second == -1 && first != -1 {
        second = first;
    }
    return Vec::from([first, second]);
}

#[test]
fn t_34() {
    let nums = vec![2, 2];
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    println!("nums: {:?} | target: ({})", nums, target);
    let ans = search_range(nums, target);
    println!("ans: {:?}", ans);

    let mut total_elements = ans[1] - ans[0] + 1;
    if ans[0] == -1 {
        total_elements = 0;
    }
    println!("total elements: {:?}", total_elements);
}
