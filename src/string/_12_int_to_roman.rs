use std::ops::{Add, AddAssign};

/// # 12. Integer to Roman
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// For example, 2 is written as II in Roman numeral, just two one's added together.
/// 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
/// # Example
/// Input: num = 58
/// Output: "LVIII"
/// Explanation: L = 50, V = 5, III = 3.

fn str_mutiply(s: &str, mut times: i32) -> String {
    let mut t = String::new();
    while times > 0 {
        t.add_assign(s);
        times -= 1;
    }
    t
}

fn int_to_roman(mut num: i32) -> String {
    let mut ans = String::with_capacity(10);
    let table = [
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ];
    let mut tmp = 0;

    for (chars, value) in table.into_iter().rev() {
        tmp = num / value;
        if tmp > 0 {
            let s = str_mutiply(chars, tmp);
            ans += &s;
            num = num % value;
        }
    }

    ans
}

#[test]
fn t_12_() {
    let num = 1994;
    let ans = int_to_roman(num); // ans : MCMXCIV
    println!("num: {num} | ans: {ans}");
}
