/**
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 *
 * Integers in each row are sorted in ascending from left to right.
 * Integers in each column are sorted in ascending from top to bottom. *
 *
 * Example:
 *
 * Consider the following matrix:
 *
 * [
 *   [1,   4,  7, 11, 15],
 *   [2,   5,  8, 12, 19],
 *   [3,   6,  9, 16, 22],
 *   [10, 13, 14, 17, 24],
 *   [18, 21, 23, 26, 30]
 * ]
 * Given target = 5, return true.
 * Given target = 20, return false.
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let (mut i, mut j) = (0 as usize, cols - 1);
        while i < rows && j >= 0 {
            if matrix[i][j] == target {
                return true;
            }
            if matrix[i][j] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        false
    }
}

struct TestCase {
    matrix: Vec<Vec<i32>>,
    target: i32,
    expected: bool,
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use exercises::n0240_search_a_2d_matrix_ii::TestCase;

    #[test]
    fn test_74() {
        let m = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let m1 = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let mut tests = Vec::new();
        tests.push(TestCase {
            matrix: m.to_owned(),
            target: 5,
            expected: true,
        });
        tests.push(TestCase {
            matrix: m.to_owned(),
            target: 20,
            expected: false,
        });
        tests.push(TestCase {
            matrix: m.to_owned(),
            target: 1,
            expected: true,
        });
        tests.push(TestCase {
            matrix: m1.to_owned(),
            target: 15,
            expected: true,
        });

        for test in tests.iter() {
            assert_eq!(
                Solution::search_matrix_ii(test.matrix.to_owned(), test.target),
                test.expected
            )
        }
    }
}
