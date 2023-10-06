/// 240. Search a 2D Matrix II [https://leetcode.com/problems/search-a-2d-matrix-ii/description/]
/// You are given an m x n integer matrix matrix with the following two properties:
///
/// Each row is sorted in non-decreasing order.
/// The first integer of each row is greater than the last integer of the previous row.
/// Given an integer target, return true if target is in matrix or false otherwise.
///
/// You must write a solution in `O(log(m * n))` time complexity.
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    println!("matrix: {matrix:?}, target: {target}");

    let mut element = 0;
    let mut row = 0 as i32;
    let mut col = matrix[0].len() as i32 - 1;

    while row < matrix.len() as i32 && col >= 0 {
        element = matrix[row as usize][col as usize];

        if target == element {
            return true;
        } else if element > target {
            col -= 1;
        } else {
            row += 1;
        }
    }

    false
}

#[test]
fn t_240() {
    let matrix = vec![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30],
    ];
    // let matrix = vec![
    //     [1, 2, 3, 4, 5],
    //     [6, 7, 8, 9, 10],
    //     [11, 12, 13, 14, 15],
    //     [16, 17, 18, 19, 20],
    //     [21, 22, 23, 24, 25],
    // ];
    let matrix = matrix.into_iter().map(|e| e.to_vec()).collect();
    let target = 26;
    let ans = search_matrix(matrix, target);
    println!("ans: {ans}");
}
