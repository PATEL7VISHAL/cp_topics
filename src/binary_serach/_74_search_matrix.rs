/// 74. Search a 2D Matrix [https://leetcode.com/problems/search-a-2d-matrix/description/]
/// You are given an m x n integer matrix matrix with the following two properties:
///
/// Each row is sorted in non-decreasing order.
/// The first integer of each row is greater than the last integer of the previous row.
/// Given an integer target, return true if target is in matrix or false otherwise.
///
/// You must write a solution in O(log(m * n)) time complexity.
/// # Example
/// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
/// Output: true
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    println!("matrix: {matrix:?}, target: {target}");
    let mut rows = matrix.len();
    let mut columns = matrix[0].len();
    let mut start = 0;
    let mut end = (rows * columns) as i32 - 1;
    let mut mid = 0;
    let mut element = 0;

    // for i in 0..=(end as usize) {
    //     let r = i / columns;
    //     let c = i % columns;
    //     print!("{} ", matrix[r][c]);
    // }

    while start <= end {
        let mid = start + (end - start) / 2;
        let element = matrix[mid as usize / columns][mid as usize % columns];

        // print!("start: {start} | end: {end}");
        // println!(" | mid: {mid} | element: {element}");

        if element == target {
            return true;
        }

        if element > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    false
}

#[test]
fn t_74() {
    let matrix = vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]];
    let matrix = matrix.into_iter().map(|e| e.to_vec()).collect();
    let target = 3;
    let ans = search_matrix(matrix, target);
    println!("ans: {ans}");
}
