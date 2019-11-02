/**
 * [221] Maximal Square
 *
 * Given a 2D binary matrix filled with 0's and 1's,
 * find the largest square containing only 1's and return its area.
 *
 * Example:
 *
 *
 * Input:
 *
 * 1 0 1 0 0
 * 1 0 1 1 1
 * 1 1 1 1 1
 * 1 0 0 1 0
 *
 * Output: 4
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = true;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let (m, n, mut max) = (matrix.len(), matrix[0].len(), 0 as u32);
        let mut cur = vec![0; n];
        let mut pre = 0;
        for i in 0..m {
            for j in 0..n {
                let temp = cur[j];
                if i == 0 || j == 0 || matrix[i][j] == '0' {
                    cur[j] = matrix[i][j].to_digit(10).unwrap();
                } else {
                    cur[j] = u32::min(pre, u32::min(cur[j], cur[j - 1])) + 1;
                }
                max = u32::max(max, cur[j]);
                pre = temp;
            }
        }
        (max * max) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![vec!['0', '1', '1', '0', '0'],]),
            1
        );
        assert_eq!(Solution::maximal_square(vec![]), 0);
    }
}
