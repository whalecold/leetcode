/**
 * [118] Pascal's Triangle
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
 *
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" /><br />
 * <small>In Pascal's triangle, each number is the sum of the two numbers directly above it.</small>
 *
 * Example:
 *
 *
 * Input: 5
 * Output:
 * [
 *      [1],
 *     [1,1],
 *    [1,2,1],
 *   [1,3,3,1],
 *  [1,4,6,4,1]
 * ]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if num_rows == 0 {
            return ret;
        }
        for i in 0..num_rows {
            let i = (i + 1) as usize;
            let mut sub = vec![1; i as usize];
            for j in 0..i {
                if j == 0 || j == i-1 {
                    continue;
                }
                sub[j] = ret[i - 2][j] + ret[i - 2][j - 1];
            }
            ret.push(sub);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_118() {
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1],]
        );
    }
}
