/**
 * [841] Keys and Rooms
 *
 * There are N rooms and you start in room 0.  Each room has a distinct number in 0, 1, 2, ..., N-1, and each room may have some keys to access the next room.
 *
 * Formally, each room i has a list of keys rooms[i], and each key rooms[i][j] is an integer in [0, 1, ..., N-1] where N = rooms.length.  A key rooms[i][j] = v opens the room with number v.
 *
 * Initially, all the rooms start locked (except for room 0).
 *
 * You can walk back and forth between rooms freely.
 *
 * Return true if and only if you can enter every room.
 *
 * <ol>
 * </ol>
 *
 * Example 1:
 *
 *
 * Input: [[1],[2],[3],[]]
 * Output: true
 * Explanation:  
 * We start in room 0, and pick up key 1.
 * We then go to room 1, and pick up key 2.
 * We then go to room 2, and pick up key 3.
 * We then go to room 3.  Since we were able to go to every room, we return true.
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,3],[3,0,1],[2],[0]]
 * Output: false
 * Explanation: We can't enter the room with number 2.
 *
 *
 * Note:
 *
 * <ol>
 * 	1 <= rooms.length <= 1000
 * 	0 <= rooms[i].length <= 1000
 * 	The number of keys in all rooms combined is at most 3000.
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}
use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    // 可以递归 也可以使用栈 递归函数也是一种栈
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut reach = HashSet::new();
        Solution::dfs_visit_rooms(&mut reach, &rooms, 0);
        reach.len() == rooms.len()
    }
    fn dfs_visit_rooms(reach: &mut HashSet<usize>, rooms: &Vec<Vec<i32>>, index: usize) {
        if index >= rooms.len() || reach.contains(&index) {
            return;
        }
        reach.insert(index);
        for i in 0..rooms[index].len() {
            Solution::dfs_visit_rooms(reach, rooms, rooms[index][i] as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_841() {}
}
