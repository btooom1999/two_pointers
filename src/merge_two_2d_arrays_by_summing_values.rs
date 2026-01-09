use std::collections::BTreeMap;

fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hashmap = BTreeMap::<i32, i32>::new();
    for nums in nums1 {
        hashmap.insert(nums[0], nums[1]);
    }

    for nums in nums2 {
        hashmap.entry(nums[0]).and_modify(|v| *v += nums[1]).or_insert(nums[1]);
    }

    hashmap.into_iter().map(|(k, v)| Vec::from([k, v])).collect::<Vec<_>>()
}

pub fn main() {
    let nums1 = vec![vec![1,2],vec![2,3],vec![4,5]];
    let nums2 = vec![vec![1,4],vec![3,2],vec![4,1]];
    println!("{:?}", merge_arrays(nums1, nums2));
}
