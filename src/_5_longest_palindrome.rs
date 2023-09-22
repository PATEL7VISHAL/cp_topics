pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let start = 0;
    let end = 0;
    let max = 0;
    let tmp = 0;
    let mut char = 0;
    let seen = [-1; 256];

    for i in bytes.iter() {
        char = *i;
        let info = seen[char as usize];
    }

    String::from_utf8_lossy(&bytes[start..=end]).to_string()
}
