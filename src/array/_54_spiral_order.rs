/// # 54. Spiral Matrix [https://leetcode.com/problems/spiral-matrix/]
/// Given an m x n matrix, return all elements of the matrix in spiral order.
/// # Example
/// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
/// Output: [1,2,3,6,9,8,7,4,5]
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut elements = matrix.len() * matrix[0].len();
    let mut ans = Vec::with_capacity(elements);
    let mut top = 0;
    let mut right = matrix[0].len();
    let mut bottom = matrix.len();
    let mut left = 0;
    let mut i = 0;

    while elements > 0 {
        i = left;
        while i < right {
            if elements == 0 {
                break;
            }
            ans.push(matrix[top][i]);
            i += 1;
            elements -= 1;
        }
        top += 1;

        i = top;
        right -= 1;
        while i < bottom {
            if elements == 0 {
                break;
            }
            ans.push(matrix[i][right]);
            i += 1;
            elements -= 1;
        }

        i = right;
        bottom -= 1;
        while i > left {
            println!("elements: {elements}");
            if elements == 0 {
                break;
            }
            i -= 1;
            elements -= 1;
            ans.push(matrix[bottom][i]);
        }

        i = bottom;
        while i > top {
            if elements == 0 {
                break;
            }
            i -= 1;
            elements -= 1;
            ans.push(matrix[i][left]);
        }
        left += 1;
    }

    ans
}

#[test]
fn t_54_() {
    let matrix = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let matrix = vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    let matrix = vec![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        // [10, 13, 14, 17, 24],
        // [18, 21, 23, 26, 30],
    ];
    let matrix = matrix.into_iter().map(|e| e.to_vec()).collect::<Vec<_>>();
    println!("matrix: {matrix:?}");

    let ans = spiral_order(matrix);
    println!("ans: {ans:?}");
}

// another solutions
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn spiral_order2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut direction = Direction::Right;
    let mut round = 0;
    let mut row = 0;
    let mut col = 0;
    let m = matrix.len();
    let n = matrix[0].len();
    let mut flatten_matrix = vec![];

    for _ in 0..m * n {
        flatten_matrix.push(matrix[row][col]);

        match direction {
            Direction::Up => {
                // reach the limit
                if row == round + 1 {
                    col += 1;
                    direction = Direction::Right;
                    round += 1;
                } else {
                    row -= 1;
                }
            }
            Direction::Down => {
                // reach the limit
                if row + 1 == m - round {
                    col -= 1;
                    direction = Direction::Left;
                } else {
                    row += 1;
                }
            }
            Direction::Right => {
                // reach the limit
                if col + 1 == n - round {
                    row += 1;
                    direction = Direction::Down;
                } else {
                    col += 1;
                }
            }
            Direction::Left => {
                // reach the limit
                if col == round {
                    row -= 1;
                    direction = Direction::Up;
                } else {
                    col -= 1;
                }
            }
        }
    }

    flatten_matrix
}
