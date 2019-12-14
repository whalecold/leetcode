/**
 * [74] Search a 2D Matrix
 *
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 *
 *
 * 	Integers in each row are sorted from left to right.
 * 	The first integer of each row is greater than the last integer of the previous row.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 13
 * Output: false
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false
        }
        let (rows, cols) = (matrix.len(), matrix[0].len());
        if matrix[0][0] > target || matrix[rows - 1][cols - 1] < target {
            return false;
        }
        let (mut l, mut r) = (0 as usize, cols * rows - 1);
        let get_coordinate = |x: usize| -> (usize, usize) { (x / cols, x % cols) };
        let mut mid = get_coordinate((l + r) / 2);
        while l < r {
            if matrix[mid.0][mid.1] == target {
                return true;
            }
            if matrix[mid.0][mid.1] > target {
                r = (l + r) / 2;
            } else {
                l = (l + r) / 2 + 1;
            }
            mid = get_coordinate((l + r) / 2);
        }
        matrix[mid.0][mid.1] == target
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_74() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            ),
            false
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                7
            ),
            true
        );
        assert_eq!(Solution::search_matrix(vec![vec![]], 1), false);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
    }
}
