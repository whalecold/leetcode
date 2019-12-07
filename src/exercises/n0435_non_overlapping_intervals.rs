/**
 * [435] Non-overlapping Intervals
 *
 * Given a collection of intervals, find the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
 * <ol>
 * </ol>
 *  
 * Example 1:
 *
 * Input: [[1,2],[2,3],[3,4],[1,3]]
 * Output: 1
 * Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.
 *
 * Example 2:
 *
 * Input: [[1,2],[1,2],[1,2]]
 * Output: 2
 * Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.
 *
 * Example 3:
 *
 * Input: [[1,2],[2,3]]
 * Output: 0
 * Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
 *
 *  
 * Note:
 * <ol>
 * 	You may assume the interval's end point is always bigger than its start point.
 * 	Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        let mut ret = vec![];
        let mut intervals = intervals;
        intervals.sort();
        let mut number = 0;
        ret.push(intervals[0].to_owned());
        for i in 1..intervals.len() {
            let last = ret.last_mut().unwrap();
            if intervals[i][0] >= last[1] {
                ret.push(intervals[i].to_owned());
            } else {
                last[1] = i32::min(last[1], intervals[i][1]);
                number += 1;
            }
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_435() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
