/// You are climbing a stair case. It takes n steps to reach to the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways
/// can you climb to the top?
/// Note: Given n will be a positive integer.
//
/// ## Example 1:
//
/// Input: 2
/// Output: 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut resolve = vec![0; n];
        for i in 0..n {
            if i == 0 {
                resolve[i] = 1;
            } else if i == 1 {
                resolve[i] = 2;
            } else {
                resolve[i] = resolve[i - 1] + resolve[i - 2];
            }
        }
        resolve.last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_70() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
