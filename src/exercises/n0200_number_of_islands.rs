/**
 * [200] Number of Islands
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 *
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}
use std::collections::VecDeque;
impl Solution {
    #[allow(dead_code)]
    // 这是广度优先  也可以使用深度优先去解决这个问题。
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut num = 0;
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let (row, column) = (grid.len(), grid[0].len());
        let mut grid_mark = vec![vec![false; column]; row];
        for i in 0..row {
            for j in 0..column {
                if grid[i][j].eq(&'0') || grid_mark[i][j] {
                    continue;
                }
                grid_mark[i][j] = true;
                let mut deque = VecDeque::new();
                deque.push_back((i, j));
                while !deque.is_empty() {
                    let length = deque.len();
                    for di in 0..length {
                        let cur = deque[di];
                        let mut bfs = |x: usize, y: usize| {
                            if x >= row || y >= column || grid[x][y].eq(&'0') || grid_mark[x][y] {
                                return;
                            }
                            deque.push_back((x, y));
                            grid_mark[x][y] = true;
                        };
                        if cur.0 > 0 {
                            bfs(cur.0 - 1, cur.1);
                        }
                        if cur.1 > 0 {
                            bfs(cur.0, cur.1 - 1);
                        }
                        bfs(cur.0 + 1, cur.1);
                        bfs(cur.0, cur.1 + 1);
                    }
                    deque.pop_front();
                }
                num += 1;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    // use super::Solution;

    #[test]
    fn test_200() {}
}
