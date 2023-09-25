/// # 349. Intersection of Two Arrays [https://leetcode.com/problems/intersection-of-two-arrays]
/// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element
/// in the result must be unique and you may return the result in any order
///
/// # Example 1:
/// Input: nums1 = [1,2,2,1], nums2 = [2,2]
/// Output: [2]
fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut seen = [0; 1001];
    let mut prev = 0;
    let mut ans = Vec::new();

    for i in nums1 {
        seen[i as usize] += 1;
    }

    for i in nums2 {
        prev = seen[i as usize];
        if prev != 0 {
            seen[i as usize] = 0;
            ans.push(i)
        }
    }

    ans
}

#[test]
fn t_349() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2]; // ans = 2;
    println!("nums1: {nums1:?}");
    println!("nums2: {nums2:?}");

    let ans = intersection(nums1, nums2);
    println!("ans: {ans:?}");
}
