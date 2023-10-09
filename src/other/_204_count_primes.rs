/// # 204. Count Primes [https://leetcode.com/problems/count-primes/]
/// Given an integer n, return the number of prime numbers that are strictly less than n.
/// # Example
/// Input: n = 10
/// Output: 4
/// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
///
/// # Apporach { Sieve of EratosthenesS }
fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    let mut count = 0;
    let mut tmp = 0;
    let mut checks = vec![true; n + 1];
    let mut multiplyer = 2;

    for i in 2..n {
        if checks[i] {
            count += 1;
            // multiplyer = 2;
            // tmp = i * multiplyer;
            // while tmp <= n {
            //     checks[tmp] = false;
            //     multiplyer += 1;
            //     tmp = i * multiplyer;
            // }

            // IDK but it seems below for loop is fater then above while loop
            for j in (i * i..n).step_by(i) {
                checks[j] = false;
            }
        }
    }
    // println!("arrs: {:?}", &checks[0..=n]);

    count
}

#[test]
fn t_204_() {
    let n = 2;
    let ans = count_primes(n);
    println!("n: {n} | ans: {ans}")
}
