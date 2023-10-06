/// # 7. Reverse Integer [https://leetcode.com/problems/reverse-integer/]
/// Given a signed 32-bit integer x, return x with its digits reversed.
/// jf reversing x causes the value to go outside the signed 32-bit integer
/// range [-231, 231 - 1], then return 0.
///
/// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
/// # Example
/// Example 1:
/// Input: x = 123
/// Output: 321
fn reverse(mut x: i32) -> i32 {
    let mut ans = 0;
    let is_minus = x < 0;
    let max_r;
    let max_by_10;
    if is_minus {
        max_r = -(i32::min_value() % 10);
        max_by_10 = -(i32::min_value() / 10);
        if x == i32::min_value() {
            return 0;
        }
        x = -x;
    } else {
        max_r = i32::max_value() % 10;
        max_by_10 = i32::max_value() / 10;
    };

    while x > 0 {
        if ans > max_by_10 {
            return 0;
        }

        ans = ans * 10;
        if ans >= max_by_10 {
            let r = x % 10;
            if r > max_r {
                return 0;
            }
        }

        ans += x % 10;
        x = x / 10;
    }

    if is_minus {
        return -ans;
    }
    ans
}

#[test]
fn t_7_() {
    //  MAX 2147483647
    //  MIN -2147483648

    let x = i32::max_value();
    let x = -123;
    // let x = -2147483648;
    let x = -900000;
    let ans = reverse(x);
    println!("x: {x} | ans: {ans}");
}
