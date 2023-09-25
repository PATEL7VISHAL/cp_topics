/// # 1552. Magnetic Force Between Two Balls [https://leetcode.com/problems/magnetic-force-between-two-balls/description/]
///In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls
///if they are put in his new invented basket. Rick has n empty baskets, the ith basket is
///at position[i], Morty has m balls and needs to distribute the balls into the baskets
///such that the minimum magnetic force between any two balls is maximum.
/// Rick stated that magnetic force between two different balls at positions x and y is |x - y|.
///
/// Given the integer array position and the integer m. Return the required force.
///
/// # Example
/// Input: position = [1,2,3,4,7], m = 3
/// Output: 3
/// Explanation: Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between
/// ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort();

    let mut start = 0;
    let mut end = *position.iter().max().unwrap();
    let mut mid = 0;
    let mut balls = 0;
    let mut ans = -1;

    while start <= end {
        mid = start + (end - start) / 2;
        let balls = distribute_balls(position.as_ref(), mid);
        println!("mid: {mid}, balls: {balls}");

        if balls < m {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    end
}

fn distribute_balls(position: &[i32], force: i32) -> i32 {
    let mut count_balls = 1;
    let mut sum = 0;
    let mut last = position[0];

    for i in &position[1..] {
        let i = *i;
        if i - last >= force {
            last = i;
            count_balls += 1;
        }
    }

    count_balls
}

#[test]
fn t_1552() {
    let position = vec![1, 2, 3, 4, 7];
    // let position = vec![5, 4, 3, 2, 1, 1000000000];
    let position = vec![79, 74, 57, 22];
    println!("positions: {:?}", position);

    let m = 3;
    let m = 4;
    let ans = max_distance(position, m);
    println!("ans: {ans}");
}
