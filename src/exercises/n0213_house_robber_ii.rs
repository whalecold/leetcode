/*
*
*
*
*
*
*
*/

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return nums[0];
        }
        i32::max(
            Solution::sub_rob(&nums, 0, len - 2),
            Solution::sub_rob(&nums, 1, len - 1),
        )
    }
    fn sub_rob(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        let (mut pre, mut p_pre) = (0, 0);
        for i in l..=r {
            let num = nums[i] as usize;
            let rob = usize::max(p_pre + num, pre);
            p_pre = pre;
            pre = rob;
        }
        pre as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    }
}
