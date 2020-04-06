/**
 * [1401] Circle and Rectangle Overlapping
 *
 * Given a circle represented as (radius, x_center, y_center) and an axis-aligned rectangle represented as (x1, y1, x2, y2), where (x1, y1) are the coordinates of the bottom-left corner, and (x2, y2) are the coordinates of the top-right corner of the rectangle.
 *
 * Return True if the circle and rectangle are overlapped otherwise return False.
 *
 * In other words, check if there are any point (xi, yi) such that belongs to the circle and the rectangle at the same time.
 *
 *  
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/20/sample_4_1728.png" style="width: 258px; height: 167px;" />
 *
 *
 * Input: radius = 1, x_center = 0, y_center = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
 * Output: true
 * Explanation: Circle and rectangle share the point (1,0)
 *
 *
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/20/sample_2_1728.png" style="width: 150px; height: 135px;" />
 *
 *
 * Input: radius = 1, x_center = 0, y_center = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1
 * Output: true
 *
 *
 * Example 3:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/03/sample_6_1728.png" style="width: 175px; height: 165px;" />
 *
 *
 * Input: radius = 1, x_center = 1, y_center = 1, x1 = -3, y1 = -3, x2 = 3, y2 = 3
 * Output: true
 *
 *
 * Example 4:
 *
 *
 * Input: radius = 1, x_center = 1, y_center = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
 * Output: false
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= radius <= 2000
 * 	-10^4 <= x_center, y_center, x1, y1, x2, y2 <= 10^4
 * 	x1 < x2
 * 	y1 < y2
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    // Ref: https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
    #[allow(dead_code)]
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        // in the rectangle
        let clamp =
            |val: i32, min: i32, max: i32| -> i32 { std::cmp::max(min, std::cmp::min(max, val)) };
        let close_x = clamp(x_center, x1, x2);
        let close_y = clamp(y_center, y1, y2);

        return (close_x - x_center) * (close_x - x_center)
            + (close_y - y_center) * (close_y - y_center)
            <= radius * radius;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1401() {}
}
