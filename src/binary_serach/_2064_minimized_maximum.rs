/// # 2064. Minimized Maximum of Products Distributed to Any Store [https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/]
///You are given an integer n indicating there are n specialty retail stores.
///There are m product types of varying amounts, which are given as a
///0-indexed integer array quantities, where quantities[i] represents
///the number of products of the ith product type.
///
///You need to distribute all products to the retail stores following these rules:
///
/// A store can only be given at most one product type but can be given any amount of it.
/// After distribution, each store will have been given some number of products (possibly 0).
/// Let x represent the maximum number of products given to any store. You want x to be
/// as small as possible, i.e., you want to minimize the maximum number of products that are given to any store.
/// Return the minimum possible x.
/// # Example
/// Input: n = 6, quantities = [11,6]
/// Output: 3
/// Explanation: One optimal way is:
/// - The 11 products of type 0 are distributed to the first four stores in these amounts: 2, 3, 3, 3
/// - The 6 products of type 1 are distributed to the other two stores in these amounts: 3, 3
/// The maximum number of products given to any store is max(2, 3, 3, 3, 3, 3) = 3.
pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut start = 1;
    let mut end = 0;
    let mut mid = 0;
    let mut products = 0;

    for i in quantities.iter() {
        // end.checked_add(*i);
        end += *i as i64;
    }

    while start <= end {
        mid = start + (end - start) / 2;
        products = distribute_quantities(quantities.as_ref(), mid);
        println!("mid: {mid}, products:{products}");

        if products > n {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    start as i32
}

pub fn distribute_quantities(quantities: &[i32], products: i64) -> i32 {
    let mut count = 0;
    let mut tmp = 0;

    for i in quantities {
        let i = *i as i64;
        tmp = i / products;
        count += tmp;

        if (i % products) != 0 {
            count += 1;
        }
    }

    count as i32
}

#[test]
fn t_2064() {
    // let quantities = vec![11, 6]; // n = 6 | ans  = 3
    let quantities = vec![15, 10, 10]; // n = 7 | ans = 5
    let quantities = vec![100000]; // n = 1 | 100000
    let quantities = vec![1];
    println!("quantities :{quantities:?}");
    let n = 1;
    let ans = minimized_maximum(n, quantities);
    println!("ans: {ans}");
}
