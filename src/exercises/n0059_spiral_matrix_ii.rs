/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate a square matrix filled with elements from 1 to n^2 in spiral order.
 *
 * Example:
 *
 *
 * Input: 3
 * Output:
 * [
 *  [ 1, 2, 3 ],
 *  [ 8, 9, 4 ],
 *  [ 7, 6, 5 ]
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
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ret = vec![vec![0; n]; n];
        let mut ele = 1;
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0 as usize, n, 0 as usize, n);
        loop {
            for i in min_y..max_y {
                ret[min_x][i] = ele;
                ele += 1;
            }

            min_x += 1;
            if min_x == max_x {
                break;
            }
            for i in min_x..max_x {
                ret[i][max_y - 1] = ele;
                ele += 1;
            }
            max_y -= 1;
            if min_y == max_y {
                break;
            }

            for i in (min_y..max_y).rev() {
                ret[max_x - 1][i] = ele;
                ele += 1;
            }
            max_x -= 1;
            if min_x == max_x {
                break;
            }
            for i in (min_x..max_x).rev() {
                ret[i][min_y] = ele;
                ele += 1;
            }
            min_y += 1;
            if max_y == min_y {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_59() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}
