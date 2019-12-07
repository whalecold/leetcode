/**
 * [56] Merge Intervals
 *
 * Given a collection of intervals, merge all overlapping intervals.
 *
 * Example 1:
 *
 *
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];
        if intervals.len() == 0 {
            return ret;
        }
        let mut intervals = intervals;
        intervals.sort();
        ret.push(intervals[0].to_owned());
        for i in 1..intervals.len() {
            let last = ret.last_mut().unwrap();
            if intervals[i][0] > last[1] {
                ret.push(intervals[i].to_owned());
            } else {
                last[1] = i32::max(last[1], intervals[i][1]);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 6], vec![8, 10], vec![15, 19]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 19]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![2, 4], vec![5, 6], vec![8, 10], vec![1, 18]]),
            vec![vec![1, 18]]
        );
    }
}
