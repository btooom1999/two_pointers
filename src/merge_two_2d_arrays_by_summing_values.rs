use std::collections::BTreeMap;

fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() || j < nums2.len() {
        if j >= nums2.len() || (i < nums1.len() && nums1[i][0] < nums2[j][0]) {
            res.push(nums1[i].clone());
            i += 1;
        } else if i >= nums1.len() || (j < nums2.len() && nums1[i][0] > nums2[j][0]) {
            res.push(nums2[j].clone());
            j += 1;
        } else {
            res.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
            i += 1;
            j += 1;
        }
    }

    res
}

pub fn main() {
    let nums1 = vec![vec![1,2],vec![2,3],vec![4,5]];
    let nums2 = vec![vec![1,4],vec![3,2],vec![4,1]];
    println!("{:?}", merge_arrays(nums1, nums2));
}
