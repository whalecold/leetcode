/**
 * [54] Spiral Matrix
 *
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 *
 * Example 2:
 *
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.len() < 1 {
            return result;
        }
        let (mut x_min, mut x_max, mut y_min, mut y_max) =
            (0 as usize, matrix.len(), 0 as usize, matrix[0].len());
        loop {
            for i in y_min..y_max {
                result.push(matrix[x_min][i]);
            }
            x_min += 1;
            if x_min == x_max {
                break;
            }
            for i in x_min..x_max {
                result.push(matrix[i][y_max - 1]);
            }
            y_max -= 1;
            if y_max == y_min {
                break;
            }
            for i in (y_min..y_max).rev() {
                result.push(matrix[x_max - 1][i]);
            }
            x_max -= 1;
            if x_max == x_min {
                break;
            }
            for i in (x_min..x_max).rev() {
                result.push(matrix[i][y_min]);
            }
            y_min += 1;
            if y_min == y_max {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_54() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 7, 8, 9]]),
            vec![1, 2, 3, 7, 8, 9]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3], vec![7], vec![8], vec![9]]),
            vec![1, 2, 3, 7, 8, 9]
        );
    }
}
