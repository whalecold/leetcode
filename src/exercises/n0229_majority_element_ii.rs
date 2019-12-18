/**
 * [229] Majority Element II
 *
 * Given an integer array of size n, find all elements that appear more than &lfloor; n/3 &rfloor; times.
 *
 * Note: The algorithm should run in linear time and in O(1) space.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3]
 * Output: [3]
 *
 * Example 2:
 *
 *
 * Input: [1,1,1,3,3,2,2,2]
 * Output: [1,2]
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn majority_element_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        if nums.len() == 0 {
            return ret;
        }
        let (mut candidate1, mut candidate2, mut count1, mut count2) = (0, 0, 0, 0);
        for i in nums.iter() {
            if candidate1 == *i {
                count1 += 1;
            } else if candidate2 == *i {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = i.to_owned();
                count1 = 1;
            } else if count2 == 0 {
                candidate2 = i.to_owned();
                count2 = 1;
            } else {
                count2 -= 1;
                count1 -= 1;
            }
        }
        if Solution::element_nums(&nums, &candidate1) > nums.len() / 3 {
            ret.push(candidate1);
        }
        if candidate2 != candidate1 && Solution::element_nums(&nums, &candidate2) > nums.len() / 3 {
            ret.push(candidate2);
        }
        ret
    }
    fn element_nums(nums: &Vec<i32>, ele: &i32) -> usize {
        let mut number = 0;
        for i in nums.iter() {
            if i == ele {
                number += 1;
            }
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_229() {
        assert_eq!(Solution::majority_element_ii(vec![3, 2, 3]), vec![3]);
    }
}
