/// # 167. Two Sum II - Input Array Is Sorted
/// Given a 1-indexed array of integers numbers that is already sorted in
/// non-decreasing order, find two numbers such that they add up to a specific
/// target number. Let these two numbers be numbers\[index1] and
/// `numbers[index2] where 1 <= index1 < index2 < numbers.length`.
///
/// Return the indices of the two numbers, index1 and index2, added by one as
/// an integer array [index1, index2] of length 2.
///
/// The tests are generated such that there is exactly one solution. You may
/// not use the same element twice.
///
/// Your solution must use only constant extra space.
/// - Time complexity `O(n)`
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    let sum = 0;

    while i < j {
        if numbers[i] + numbers[j] == target {
            return vec![i as i32 + 1, j as i32 + 1];
        }

        if numbers[i] + numbers[j] > target {
            j -= 1;
        } else {
            i += 1;
        }
    }

    return vec![];
}

/// Un optimize  taking n(log(n)) time but can be solve in O(n)
fn two_sum2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, value) in numbers.iter().enumerate() {
        if let Ok(j) = numbers[(i + 1)..].binary_search(&(target - (*value))) {
            return vec![i as i32 + 1, j as i32 + 1];
        }
    }

    vec![]
}

#[test]
fn t_167() {
    let numbers = vec![2, 7, 11, 15]; // target: 9 | ans: [1,2]
    let numbers = vec![2, 3, 4]; // target: 6 | ans: [1,3]
                                 // let numbers = vec![-1, 0]; // target: -1 | ans: [1,2]
    let numbers = vec![5, 25, 75];
    let target = 100;
    println!("numbers: {numbers:?} | target: {target}");

    let ans = two_sum(numbers, target);
    println!("ans: {ans:?}");
}
