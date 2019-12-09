/**
 * [57] Insert Interval
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 *
 * You may assume that the intervals were initially sorted according to their start times.
 *
 * Example 1:
 *
 *
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 *
 *
 * Example 2:
 *
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
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
    pub fn insert1(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if intervals.len() == 0 {
            ret.push(new_interval);
            return ret;
        }
        let mut insert = false;
        for i in 0..intervals.len() {
            if intervals[i][1] < new_interval[0] {
                ret.push(intervals[i].to_owned());
                continue;
            }
            if !insert {
                insert = true;
                if new_interval[1] < intervals[i][0] {
                    ret.push(new_interval.to_owned());
                    ret.push(intervals[i].to_owned());
                    continue;
                }
                let mut new_interval = new_interval.to_owned();
                new_interval[0] = i32::min(new_interval[0], intervals[i][0]);
                new_interval[1] = i32::max(new_interval[1], intervals[i][1]);
                ret.push(new_interval);
            } else {
                let last = ret.last_mut().unwrap();
                if last[1] < intervals[i][0] {
                    ret.push(intervals[i].to_owned());
                } else {
                    last[1] = i32::max(last[1], intervals[i][1]);
                }
            }
        }
        if !insert {
            ret.push(new_interval);
        }
        ret
    }
    #[allow(dead_code)]
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut index = 0 as usize;
        let mut new_interval = new_interval;
        while index < intervals.len() && intervals[index][1] < new_interval[0] {
            ret.push(intervals[index].to_owned());
            index += 1;
        }
        while index < intervals.len() && intervals[index][0] <= new_interval[1] {
            new_interval[0] = i32::min(intervals[index][0], new_interval[0]);
            new_interval[1] = i32::max(intervals[index][1], new_interval[1]);
            index += 1;
        }
        ret.push(new_interval);
        while index < intervals.len() {
            ret.push(intervals[index].to_owned());
            index += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_57() {
        assert_eq!(Solution::insert(vec![], vec![2, 5]), vec![vec![2, 5]]);
        assert_eq!(
            Solution::insert(vec![vec![1, 3]], vec![2, 5]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 3]], vec![4, 5]),
            vec![vec![1, 3], vec![4, 5]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
