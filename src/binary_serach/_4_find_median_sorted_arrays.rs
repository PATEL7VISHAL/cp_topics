pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n1 = nums1.len();
    let n2 = nums2.len();
    if (n1 > n2) {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let mut low: i32 = 0;
    let mut high = n1 as i32;
    let mut left = (n1 + n2 + 1) / 2;
    let n = n1 + n2;
    while (low <= high) {
        let mid1 = low + high >> 1;
        let mid2 = left as i32 - mid1;
        let mut l1 = i32::MIN;
        let mut l2 = i32::MIN;
        let mut r1 = i32::MAX;
        let mut r2 = i32::MAX;

        if (mid1 < n1 as i32) {
            r1 = nums1[mid1 as usize];
        }
        if (mid2 < n2 as i32) {
            r2 = nums2[mid2 as usize];
        }
        if (mid1 - 1 >= 0) {
            l1 = nums1[(mid1 - 1) as usize];
        }
        if (mid2 - 1 >= 0) {
            l2 = nums2[(mid2 - 1) as usize];
        }

        if (l1 <= r2 && l2 <= r1) {
            if (n % 2 == 1) {
                return l1.max(l2) as f64;
            }
            return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
        } else if l1 > r2 {
            high = mid1 - 1;
        } else {
            low = mid1 + 1
        }
    }

    return 0 as f64;
}
