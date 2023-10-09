/// # 796. Rotate String [https://leetcode.com/problems/rotate-string/]
/// Given two strings s and goal, return true if and only if s can become
/// goal after some number of shifts on s.
///
/// A shift on s consists of moving the leftmost character of s to the rightmost position.
///
/// For example, if s = "abcde", then it will be "bcdea" after one
/// # Example
/// Example 1:
/// Input: s = "abcde", goal = "cdeab"
/// Output: true
pub fn rotate_string(mut s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    s += s.clone().as_ref();
    return s.find(goal.as_str()).is_some();
}

#[test]
fn t_796_() {
    // let s = "abcde".to_string();
    // let goal = "addeb".into();

    let s = "gcmbf".to_string();
    let goal = "fgcmb".to_string();

    println!("s: {s}, goal:{goal}");
    let ans = rotate_string(s, goal);
    println!("ans: {ans}");
}

/// if string characters are in order
fn rotate_string2(s: String, goal: String) -> bool {
    let s_buffer = s.as_bytes();
    let goal_buffer = goal.as_bytes();
    let mut count = 0;
    for i in 1..s.len() {
        if goal_buffer[i] < goal_buffer[i - 1] {
            count += 1;
        }
    }

    if goal_buffer[0] > goal_buffer[s.len() - 1] {
        count += 1;
    }

    println!("count:{count}");
    if count > 2 {
        return false;
    }

    let mut start = 0;
    let mut end = s.len() - 1;
    let mut mid = 0;
    let mut element = 0;
    while start <= end {
        mid = start + (end - start) / 2;
        element = goal_buffer[mid as usize];

        if goal_buffer[start as usize] == element && element == goal_buffer[end as usize] {
            start += 1;
            end -= 1;
            continue;
        }

        if element < goal_buffer[0] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    for i in 0..s.len() {
        if goal_buffer[i] != s_buffer[(i + end) % s.len()] {
            return false;
        }
    }

    true
}
