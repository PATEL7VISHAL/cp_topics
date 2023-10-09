/// # 59. Spiral Matrix II [https://leetcode.com/problems/spiral-matrix-ii/]
/// Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.
/// # Example
/// Input: n = 3
/// Output: [[1,2,3],[8,9,4],[7,6,5]]

#[derive(Debug)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut ans = vec![vec![0; n]; n];
    let mut row = 0;
    let mut col = 0;
    let mut direction = Direction::Right;
    let mut round = 0;

    for i in 1..=(n * n) as i32 {
        println!("Direction: {:?}", direction);
        match direction {
            Direction::Right => {
                ans[row][col] = i;

                col += 1;
                if col == (n - round) {
                    col -= 1;
                    row += 1;
                    direction = Direction::Bottom;
                    // round += 1;
                }
            }

            Direction::Bottom => {
                ans[row][col] = i;
                row += 1;
                if row == (n - round) {
                    row -= 1;
                    col -= 1;
                    direction = Direction::Left
                }
            }

            Direction::Left => {
                ans[row][col] = i;

                if col == (round) {
                    row -= 1;
                    direction = Direction::Top
                } else {
                    col -= 1;
                }
            }

            Direction::Top => {
                ans[row][col] = i;

                if row == (round + 1) {
                    col += 1;
                    direction = Direction::Right;
                    round += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }

    ans
}

#[test]
fn t_59_() {
    let n = 3;
    let ans = generate_matrix(n);

    println!("n: {n}");
    println!("ans: {ans:?}");
}
