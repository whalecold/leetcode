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
        let (mut pre, mut p_pre) = (0, 0);
        for i in nums.iter() {
            let rob = i32::max(p_pre + i.to_owned(), pre);
            p_pre = pre;
            pre = rob;
        }
        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
