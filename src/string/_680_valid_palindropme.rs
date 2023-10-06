pub fn valid_palindrome(s: String) -> bool {
    let size = s.len();
    let mut buffer = s.as_bytes();
    let mut is_missmatch_found = false;
    let mut left = 0;
    let mut right = size - 1;

    while left <= size / 2 && right >= size / 2 {
        if buffer[left] != buffer[right] {
            is_missmatch_found = true;
            // println!("break;");
            break;
        }
        left += 1;
        right -= 1;
    }

    let mut res1 = true;
    let mut res2 = true;
    if is_missmatch_found {
        let mut left2 = left;
        let mut right2 = right;

        left += 1;
        while left <= size / 2 && right >= size / 2 {
            if buffer[left] != buffer[right] {
                res1 = false;
            }
            // println!("left: {} = {}", buffer[left], buffer[right]);
            left += 1;
            right -= 1;
        }

        right2 -= 1;
        while left2 <= size / 2 && right2 >= size / 2 {
            if buffer[left2] != buffer[right2] {
                res2 = false;
            }
            // println!("right: {} = {}", buffer[left2], buffer[right2]);
            left2 += 1;
            right2 -= 1;
        }
    }
    // println!("res1: {res1} | res2: {res2}");

    if res1 == false && res2 == false {
        return false;
    }
    true
}

#[test]
fn t_680() {
    let s = "abc".to_string();
    println!("s: {s}");
    let ans = valid_palindrome(s);
    println!("ans: {ans}");
}
