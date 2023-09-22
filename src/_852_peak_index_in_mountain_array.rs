/// `852. Peak Index in a Mountain Array`
/// An array arr is a mountain if the following properties hold:
/// arr.length >= 3
/// There exists some i with 0 < i < arr.length - 1 such that:
/// arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
/// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
/// Given a mountain array arr, return the index i such that
/// `arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1]`
/// `> ... > arr[arr.length - 1]`.
/// You must solve it in O(log(arr.length)) time complexity.
/// # Example
/// Input: arr = [0,1,0]
/// Output: 1
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = arr.len() as i32 - 1;
    let mut mid = 0;
    let mut left = 0;
    let mut right = 0;
    let mut mid_number = 0;

    while start <= end {
        mid = ((start + end + 1) / 2) as usize;
        mid_number = arr[mid];
        left = arr[mid - 1];
        right = arr[mid + 1];

        if left < mid_number && mid_number > right {
            println!("peek element: {}", mid_number);
            return mid as i32;
        } else if left < mid_number && mid_number < right {
            start = mid as i32;
        } else {
            end = mid as i32;
        }
    }
    -1
}

//more efficient
pub fn peak_index_in_mountain_array2(arr: Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut e: i32 = arr.len() as i32 - 1;
    let mut mid: i32 = s + (e - s) / 2;

    // let mut mid: i32 = (s + e + 1) / 2;

    while s < e {
        mid = s + (e - 2) / 2;
        if arr[mid as usize] < arr[(mid + 1) as usize] {
            s = mid + 1;
        } else {
            e = mid;
        }
    }
    s
}

#[test]
pub fn t_852() {
    let arr = vec![1, 2, 3, 5, 6, 7, 1, 2, 3, 4];
    let arr = vec![1, 3, 2];
    let arr = vec![1, 2, 2, 3, 1];
    let arr = vec![0, 3, 5, 12, 2];
    println!("arr:{:?}", arr);
    let ans = peak_index_in_mountain_array2(arr);
    println!("ans:{}", ans);
}
