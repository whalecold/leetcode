/**
 * [119] Pascal's Triangle II
 *
 * Given a non-negative index k where k &le; 33, return the k^th index row of the Pascal's triangle.
 *
 * Note that the row index starts from 0.
 *
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 *
 * Example:
 *
 *
 * Input: 3
 * Output: [1,3,3,1]
 *
 *
 * Follow up:
 *
 * Could you optimize your algorithm to use only O(k) extra space?
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ret = vec![1; 1];
        for i in 1..=row_index {
            let i = i as usize;
            let mut temp = vec![1; i + 1];
            for j in 0..temp.len() {
                if j == 0 || j == temp.len() - 1 {
                    continue;
                }
                temp[j] = ret[j] + ret[j - 1];
            }
            ret = temp.to_owned();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_119() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
    }
}
