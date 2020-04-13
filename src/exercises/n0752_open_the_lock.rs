/**
 * [752] Open the Lock
 *
 *
 * You have a lock in front of you with 4 circular wheels.  Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'.  The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'.  Each move consists of turning one wheel one slot.
 *
 * The lock initially starts at '0000', a string representing the state of the 4 wheels.
 *
 * You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.
 *
 * Given a target representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.
 *
 *
 * Example 1:<br />
 *
 * Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
 * Output: 6
 * Explanation:
 * A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
 * Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
 * because the wheels of the lock become stuck after the display becomes the dead end "0102".
 *
 *
 *
 * Example 2:<br />
 *
 * Input: deadends = ["8888"], target = "0009"
 * Output: 1
 * Explanation:
 * We can turn the last wheel in reverse to move from "0000" -> "0009".
 *
 *
 *
 * Example 3:<br />
 *
 * Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
 * Output: -1
 * Explanation:
 * We can't reach the target without getting stuck.
 *
 *
 *
 * Example 4:<br />
 *
 * Input: deadends = ["0000"], target = "8888"
 * Output: -1
 *
 *
 *
 * Note:<br>
 * <ol>
 * The length of deadends will be in the range [1, 500].
 * target will not be in the list deadends.
 * Every string in deadends and the string target will be a string of 4 digits from the 10,000 possibilities '0000' to '9999'.
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
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let init = vec![48, 48, 48, 48];
        let (mut dead_set, mut exist_set) = (HashSet::new(), HashSet::new());
        for s in deadends.iter() {
            dead_set.insert(s);
        }
        exist_set.insert(String::from_utf8(init.clone()).unwrap());
        let mut deque = VecDeque::new();
        deque.push_back(init);
        let mut nums = 0;
        while !deque.is_empty() {
            let size = deque.len();
            for _i in 0..size {
                let front = deque.pop_front().unwrap();

                let front_str = String::from_utf8(front.clone()).unwrap();
                if dead_set.get(&front_str).is_some() {
                    break;
                }
                if front_str.eq(&target) {
                    return nums;
                }
                for i in 0..front.len() {
                    let mut add = front.clone();
                    Solution::get_add(&mut add[i]);
                    let add_queue = String::from_utf8(add.clone()).unwrap();
                    if dead_set.get(&add_queue).is_none() && exist_set.get(&add_queue).is_none() {
                        exist_set.insert(add_queue);
                        deque.push_back(add);
                    }
                    let mut sub = front.clone();
                    Solution::get_sub(&mut sub[i]);
                    let sub_queue = String::from_utf8(sub.clone()).unwrap();
                    if dead_set.get(&sub_queue).is_none() && exist_set.get(&sub_queue).is_none() {
                        exist_set.insert(sub_queue);
                        deque.push_back(sub);
                    }
                }
            }
            nums += 1;
        }
        -1
    }
    fn get_sub(input: &mut u8) {
        if *input == 48 {
            *input = 57;
        } else {
            *input -= 1;
        }
    }
    fn get_add(input: &mut u8) {
        if *input == 57 {
            *input = 48;
        } else {
            *input += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_752() {
        // assert_eq!(
        //     Solution::open_lock(vec![String::from("1234")], String::from("0100")),
        //     1
        // );
        // assert_eq!(
        //     Solution::open_lock(
        //         vec![
        //             String::from("0201"),
        //             String::from("0101"),
        //             String::from("0102"),
        //             String::from("1212"),
        //             String::from("2002")
        //         ],
        //         String::from("0202")
        //     ),
        //     6
        // );
        // assert_eq!(
        //     Solution::open_lock(vec![String::from("0000")], String::from("8888")),
        //     -1
        // );
        assert_eq!(
            Solution::open_lock(
                vec![
                    String::from("8887"),
                    String::from("8889"),
                    String::from("8878"),
                    String::from("8898"),
                    String::from("8788"),
                    String::from("8988"),
                    String::from("7888"),
                    String::from("9888")
                ],
                String::from("8888")
            ),
            -1
        );
    }
}
