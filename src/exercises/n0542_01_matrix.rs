/**
 * [542] 01 Matrix
 *
 * Given a matrix consists of 0 and 1, find the distance of the nearest 0 for each cell.
 *
 * The distance between two adjacent cells is 1.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input:
 * [[0,0,0],
 *  [0,1,0],
 *  [0,0,0]]
 *
 * Output:
 * [[0,0,0],
 *  [0,1,0],
 *  [0,0,0]]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * [[0,0,0],
 *  [0,1,0],
 *  [1,1,1]]
 *
 * Output:
 * [[0,0,0],
 *  [0,1,0],
 *  [1,2,1]]
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	The number of elements of the given matrix will not exceed 10,000.
 * 	There are at least one 0 in the given matrix.
 * 	The cells are adjacent in only four directions: up, down, left and right.
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::{HashSet, VecDeque};

impl Solution {
    #[allow(dead_code)]
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        if matrix.len() == 0 {
            return matrix;
        }
        let mut deque = VecDeque::new();
        let mut set = HashSet::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    deque.push_back((i, j));
                    set.insert((i, j));
                }
            }
        }
        let (mut distance, row, col) = (1, matrix.len(), matrix[0].len());
        while !deque.is_empty() {
            let size = deque.len();
            for _ in 0..size {
                let node = deque.pop_front().unwrap();
                if node.0 > 0 && !set.contains(&(node.0 - 1, node.1)) {
                    deque.push_back((node.0 - 1, node.1));
                    set.insert((node.0 - 1, node.1));
                    matrix[node.0 - 1][node.1] = distance;
                }
                if node.0 + 1 < row && !set.contains(&(node.0 + 1, node.1)) {
                    deque.push_back((node.0 + 1, node.1));
                    set.insert((node.0 + 1, node.1));
                    matrix[node.0 + 1][node.1] = distance;
                }
                if node.1 > 0 && !set.contains(&(node.0, node.1 - 1)) {
                    deque.push_back((node.0, node.1 - 1));
                    set.insert((node.0, node.1 - 1));
                    matrix[node.0][node.1 - 1] = distance;
                }
                if node.1 + 1 < col && !set.contains(&(node.0, node.1 + 1)) {
                    deque.push_back((node.0, node.1 + 1));
                    set.insert((node.0, node.1 + 1));
                    matrix[node.0][node.1 + 1] = distance;
                }
            }
            distance += 1;
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_542() {}
}
