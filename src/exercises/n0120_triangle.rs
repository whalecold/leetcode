/// Given a triangle, find the minimum path sum from top to bottom.
/// Each step you may move to adjacent numbers on the row below.
/// For example, given the following triangle
/// ```
/// [
///      [2],
///     [3,4],
///    [6,5,7],
///   [4,1,8,3]
/// ]
/// The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut path = vec![vec![0; triangle.len() + 1]; triangle.len() + 1];
        for i in (0..triangle.len()).rev() {
            for j in 0..=i {
                path[i][j] = triangle[i][j] + i32::min(path[i + 1][j + 1], path[i + 1][j]);
                print!("{}..", path[i][j]);
            }
            println!("{}", ".")
        }
        path[0][0]
    }
    #[allow(dead_code)]
    pub fn minimum_total1(triangle: Vec<Vec<i32>>) -> i32 {
        let mut min_path = triangle.last().unwrap().clone();
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..=i {
                min_path[j] = triangle[i][j] + i32::min(min_path[j], min_path[j + 1]);
            }
        }
        min_path[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_120() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    pub fn test_120_1() {
        assert_eq!(
            Solution::minimum_total1(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
