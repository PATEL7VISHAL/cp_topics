use std::collections::HashMap;
/// # 13. Roman to Integer [https://leetcode.com/problems/roman-to-integer/]
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// For example, 2 is written as II in Roman numeral, just two ones added together.
/// 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
/// # Example:
/// Input: s = "LVIII"
/// Output: 58
/// Explanation: L = 50, V= 5, III = 3.
fn roman_to_int(s: String) -> i32 {
    let map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut ans = 0;
    let mut chs = s.chars().rev().peekable();
    let mut tmp = 0;

    while let Some(ch) = chs.peek() {
        if *ch == 'I' {
            ans += 1;
            chs.next();
        } else {
            break;
        }
    }

    for ch in chs {
        if ch == 'I' {
            ans -= 1;
        } else {
            ans += map[&ch];
        }
    }

    ans
}

#[test]
fn t_13() {
    let s = "LVIII".to_string();
    println!("s: {s}");
    let ans = roman_to_int(s);
    println!("ans: {ans}");
}
