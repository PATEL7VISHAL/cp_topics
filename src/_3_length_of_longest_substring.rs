///# 3. Longest Substring Without Repeating Characters
///Given a string s, find the length of the longest
///substring without repeating characters
///## example
///Input: s = "abcabcbb"
///Output: 3
///Explanation: The answer is "abc", with the length of 3.
//Take 3 ms
pub fn length_of_longest_substring(s: String) -> i32 {
    // let mut hsh: [i16; 256] = [-1; 256];
    let mut hsh = vec![-1; 256];
    let mut occurance = 0;
    let mut char;
    let mut tmp = 0;
    let mut start = 0;
    let mut end = 0;

    let mut m_start = 0;
    let mut m_end = 0;
    let mut sum = 0;

    if s.len() == 0 {
        return 0;
    }

    for (i, v) in s.as_bytes().iter().enumerate() {
        char = *v as usize;
        occurance = hsh[char];
        end = i;
        hsh[char] = i as i16;
        if occurance == -1 {
            tmp = end - start;
            if sum < tmp {
                m_start = start;
                m_end = end;
                sum = tmp;
            }
        } else {
            tmp = occurance as usize + 1;
            if tmp > start {
                start = tmp
            }
            tmp = end - start;

            println!("start: {}", start);
            if sum < tmp {
                m_start = start;
                m_end = end;
                sum = tmp;
            }
        }

        println!(
            "{:?} => ({} - {})",
            char::from_u32(*v as u32).unwrap(),
            start,
            end
        );
    }

    // let sub_str = String::from_utf8_lossy(&s.as_bytes()[m_start..=m_end]).to_string();
    // println!("{} => {} ({}-{})", s, sub_str, m_start, m_end);
    //
    // sub_str.len() as i32
    return (m_end - m_start + 1) as i32;
}

//Take 1 ms
pub fn length_of_longest_substring2(s: String) -> i32 {
    let mut f_i = 0;
    let mut s_i = 0;
    let mut max_len = 0;
    let s_bytes = s.as_bytes();
    // let mut seen = HashMap::with_capacity(26);
    let mut seen: [i32; 256] = [-1; 256];

    while s_i < s_bytes.len() {
        let seen_i = seen[s_bytes[s_i] as usize];

        f_i = ((seen_i + 1) as usize).max(f_i);
        seen[s_bytes[s_i] as usize] = s_i as i32;

        max_len = max_len.max(s_i - f_i + 1);
        s_i += 1;
    }

    max_len as i32
}

#[test]
fn lols_test() {
    // assert_eq!(length_of_longest_substring("hi".to_owned()), 1);

    // println!("ans: {}", length_of_longest_substring("pwwkew".to_string()));

    // println!("ans: {}", length_of_longest_substring("pa".to_string()));
    // println!("ans: {}", length_of_longest_substring("pacp".to_string()));
    // println!("ans: {}", length_of_longest_substring("".to_string()));
    // println!(
    //     "ans: {}",
    //     length_of_longest_substring("abcabcbb".to_string())
    // );

    // println!(
    //     "ans: {}",
    //     length_of_longest_substring("bbbbbbb".to_string())
    // );

    println!("ans: {}", length_of_longest_substring("abba".to_string()));
}
