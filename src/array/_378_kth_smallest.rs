use std::collections::BinaryHeap;
/// # 378. Kth Smallest Element in a Sorted Matrix
/// Given an n x n matrix where each of the rows and columns is sorted in
/// ascending order, return the kth smallest element in the matrix.
///
/// Note that it is the kth smallest element in the sorted order, not the kth distinct element.
///
/// You must find a solution with a memory complexity better than O(n2).
/// # Example
/// Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
/// Output: 13
/// Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
    println!("matrix: {matrix:?} | k: {k}");
    let mut pq = BinaryHeap::new();

    for arr in matrix {
        for element in arr {
            pq.push(element);
            if pq.len() as i32 > k {
                pq.pop();
            }
        }
    }

    return pq.pop().unwrap();
}

/*
*
*/

#[test]
fn t_378() {
    let matrix = vec![[1, 5, 9], [10, 11, 13], [12, 13, 15]];
    let matrix = vec![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30],
    ];
    let matrix = matrix.into_iter().map(|e| e.to_vec()).collect::<Vec<_>>();
    let k = 4;
    let ans = kth_smallest(matrix, k);
    println!("ans: {ans}");
}
