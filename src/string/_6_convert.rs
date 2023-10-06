use std::{
    ops::Add,
    str::{from_utf8, from_utf8_unchecked},
};

/// # 6. Zigzag Conversion [https://leetcode.com/problems/zigzag-conversion/]
/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
/// of rows like this: (you may want to display this pattern in a fixed font for better legibility)
///```
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///```
/// And then read line by line: "PAHNAPLSIIGYIR"
/// # Example
/// Input: s = "PAYPALISHIRING", numRows = 4
/// Output: "PINALSIGYAHRPI"
/// Explanation:
/// ```
/// P     I    N
/// A   L S  I G
/// Y A   H R
/// P     I
///```
fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }
    let mut z_string: Vec<Vec<u8>> = vec![vec![]; num_rows as usize];
    let mut is_in_reverse = false;
    let mut index = 0 as i32;

    for (i, ch) in s.as_bytes().iter().enumerate() {
        let ch = *ch;

        z_string[index as usize].push(ch);
        if !is_in_reverse {
            index += 1;
            if index == num_rows as i32 {
                is_in_reverse = !is_in_reverse;
                index -= 2;
            }
        } else {
            index -= 1;
            if index == -1 {
                is_in_reverse = !is_in_reverse;
                index += 2;
            }
        }
    }

    let mut ans = String::with_capacity(s.len());
    for chars in z_string {
        ans.push_str(String::from_utf8(chars).unwrap().as_str());
    }

    ans
}

fn convert2(mut s: String, num_rows: i32) -> String {
    let mut num_rows = num_rows as usize;
    let mut toggle = 0;
    let mut refs = unsafe { s.as_mut_str().as_bytes_mut() };
    let mut tmp = 0;
    let mut z_string: Vec<Vec<u8>> = vec![vec![]; num_rows as usize];

    let mut last = 0;
    let mut i = 0;

    println!("refs: {refs:?}");

    while num_rows > 0 && i < refs.len() {
        println!("num_rows: {num_rows}");
        while toggle < refs.len() {
            tmp = refs[i];
            refs[i] = refs[toggle];
            refs[toggle] = tmp;
            // println!("i: {} | toggle: {}", i, toggle);
            // println!("i: {} | toggle: {}", refs[i], refs[toggle]);

            i += 1;
            toggle += num_rows + 1;
        }
        break;

        toggle = i;
        num_rows -= 1;
    }

    // while toggle < refs.len() {
    //     tmp = refs[i];
    //     refs[i] = refs[toggle];
    //     refs[toggle] = tmp;
    //
    //     i += 1;
    //     toggle += num_rows;
    // }

    // println!("{:?}", refs);
    // println!("i: {i} | toggle: {toggle}");

    // num_rows -= 1;
    // toggle = i;
    // while toggle < refs.len() {
    //     tmp = refs[i];
    //     refs[i] = refs[toggle];
    //     refs[toggle] = tmp;
    //     println!("i: {} | toggle: {}", refs[i], refs[toggle]);
    //
    //     i += 1;
    //     toggle += num_rows;
    // }
    // println!("{:?}", refs);

    s
}

#[test]
fn t_6() {
    let s = "PAYPALISHIRING".to_string();
    // let s = "abcdefg".to_string();
    let num_rows = 3;
    println!("s: {s} | num_row: {num_rows}");
    let ans = convert2(s, num_rows);
    println!("ans: {ans}");
}
