/**
 * [73] Set Matrix Zeroes
 *
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *   [1,1,1],
 *   [1,0,1],
 *   [1,1,1]
 * ]
 * Output:
 * [
 *   [1,0,1],
 *   [0,0,0],
 *   [1,0,1]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * [
 *   [0,1,2,0],
 *   [3,4,5,2],
 *   [1,3,1,5]
 * ]
 * Output:
 * [
 *   [0,0,0,0],
 *   [0,4,5,0],
 *   [0,3,1,0]
 * ]
 *
 *
 * Follow up:
 *
 *
 * 	A straight forward solution using O(mn) space is probably a bad idea.
 * 	A simple improvement uses O(m + n) space, but still not the best solution.
 * 	Could you devise a constant space solution?
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }
        let (mut col0, rows, cols) = (1 as usize, matrix.len(), matrix[0].len());
        for i in 0..rows {
            if matrix[i][0] == 0 {
                col0 = 0;
            }
            for j in 1..cols {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in (0..rows).rev() {
            for j in (1..cols).rev() {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if col0 == 0 {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_73() {
        let mut matrix3 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix3);
        assert_eq!(matrix3, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut matrix3 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![0, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix3);
        assert_eq!(
            matrix3,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 0, 0, 0]]
        );

        let mut matrix3 = vec![vec![1, 1, 1], vec![0, 1, 3]];
        Solution::set_zeroes(&mut matrix3);
        assert_eq!(matrix3, vec![vec![0, 1, 1], vec![0, 0, 0]]);
    }
}
