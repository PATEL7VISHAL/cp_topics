/// # 11. Container With Most Water [https://leetcode.com/problems/container-with-most-water/]
/// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
///
/// Find two lines that together with the x-axis form a container, such that the container contains
/// the most water.
///
/// Return the maximum amount of water a container can store.
///
/// Notice that you may not slant the container.
/// # Example
/// Input: height = [1,8,6,2,5,4,8,3,7]
/// Output: 49
fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut start = 0;
    let mut end = height.len() - 1;

    while start != end {
        max_area = max_area.max((end - start) as i32 * height[start].min(height[end]));

        if height[start] < height[end] {
            start += 1;
        } else {
            end -= 1;
        }
    }

    max_area
}

#[test]
fn t_11_() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("height: {height:?}");
    let ans = max_area(height);
    println!("max_area: {ans}");
}
