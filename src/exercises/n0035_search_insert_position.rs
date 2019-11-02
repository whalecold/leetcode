/**
 * [35] Search Insert Position
 *
 * Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 * 
 * You may assume no duplicates in the array.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,3,5,6], 5
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [1,3,5,6], 2
 * Output: 1
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: [1,3,5,6], 7
 * Output: 4
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: [1,3,5,6], 0
 * Output: 0
 * 
 * 
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 ||nums.first().unwrap() > &target {
            return 0;
        }
        if nums.last().unwrap() < &target {
            return nums.len() as i32;
        }
        let (mut l, mut r) = (0 as usize, nums.len()-1);
        while l < r {
            let mid = (l + r) >> 1;
            if nums[mid] >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);
    }
}
