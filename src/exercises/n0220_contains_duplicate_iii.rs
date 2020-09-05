/**
 * [220] Contains Duplicate III
 *
 * Given an array of integers, find out whether there are two distinct indices i and j in the array such that the absolute difference between nums[i] and nums[j] is at most t and the absolute difference between i and j is at most k.
 *  
 * Example 1:
 * Input: nums = [1,2,3,1], k = 3, t = 0
 * Output: true
 * Example 2:
 * Input: nums = [1,0,1,1], k = 1, t = 2
 * Output: true
 * Example 3:
 * Input: nums = [1,5,9,1,5,9], k = 2, t = 3
 * Output: false
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 2 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	0 <= k <= 10^4
 * 	0 <= t <= 2^31 - 1
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

/*
 *  最简单的方法 时间复杂度是 N*k，维护一个窗口
 *  优化：怎么去维护这个 k，可以维护二叉搜索树而不是数组，二叉搜索树支持 logN 的查找，删除和插入，所以
 *  时间复杂度就可以达到 N * logN
 *  再优化： 使用 bucket 存储数据，时间复杂度可以到 N
 */
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if k == 0 {
            return false;
        }
        let mut hash_map = HashMap::new();
        for i in 0..nums.len() {
            let num = nums.index(i).clone() as i64 - i32::MIN as i64;
            let bucket = num / (t as i64 + 1);
            if hash_map.contains_key(&bucket) {
                println!("add index {}", i);
                return true;
            }
            if hash_map.contains_key(&(bucket - 1))
                && num - hash_map.get(&(bucket - 1)).unwrap() <= t as i64
            {
                return true;
            }
            if hash_map.contains_key(&(bucket + 1))
                && hash_map.get(&(bucket + 1)).unwrap() - num <= t as i64
            {
                return true;
            }

            if hash_map.len() >= k as usize {
                let last_bucket = (nums[i - k as usize] as i64 - i32::MIN as i64) / (t as i64 + 1);
                hash_map.remove(&last_bucket);
            }
            hash_map.insert(bucket, num.clone() as i64);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_220() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(Vec::from([1, 2, 3, 1]), 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(Vec::from([1, 5, 9, 1, 5, 9]), 2, 3),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(
                Vec::from([2147483647, -1, 2147483647]),
                1,
                2147483647
            ),
            false
        );
    }
}
