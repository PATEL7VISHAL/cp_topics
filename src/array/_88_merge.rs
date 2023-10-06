/// # 88. Merge Sorted Array [https://leetcode.com/problems/merge-sorted-array/]
/// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order,
/// and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
///
/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
///
/// The final sorted array should not be returned by the function, but instead be stored
/// inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
/// # Example
/// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// Output: [1,2,2,3,5,6]
/// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
/// The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m as usize;
    let mut j = n as usize;
    let mut index = (m + n) as usize;

    while j > 0 {
        if i > 0 && nums1[i - 1] > nums2[j - 1] {
            i -= 1;
            index -= 1;
            nums1[index] = nums1[i];
        } else {
            j -= 1;
            index -= 1;
            nums1[index] = nums2[j];
        }
    }
}

#[test]
fn t_88_() {
    // let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    // let mut nums2 = vec![2, 5, 6];
    // let m = 3;
    // let n = 3;

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    let m = 0;
    let n = 1;

    println!("nums1:{nums1:?}");
    println!("nums2: {nums2:?}");
    merge(&mut nums1, m, &mut nums2, n);
    println!("nums1:{nums1:?}");
}
