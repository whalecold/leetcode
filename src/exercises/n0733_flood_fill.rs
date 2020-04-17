/**
 * [733] Flood Fill
 *
 *
 * An image is represented by a 2-D array of integers, each integer representing the pixel value of the image (from 0 to 65535).
 *
 * Given a coordinate (sr, sc) representing the starting pixel (row and column) of the flood fill, and a pixel value newColor, "flood fill" the image.
 *
 * To perform a "flood fill", consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color as the starting pixel), and so on.  Replace the color of all of the aforementioned pixels with the newColor.
 *
 * At the end, return the modified image.
 *
 * Example 1:<br />
 *
 * Input:
 * image = [[1,1,1],[1,1,0],[1,0,1]]
 * sr = 1, sc = 1, newColor = 2
 * Output: [[2,2,2],[2,2,0],[2,0,1]]
 * Explanation:
 * From the center of the image (with position (sr, sc) = (1, 1)), all pixels connected
 * by a path of the same color as the starting pixel are colored with the new color.
 * Note the bottom corner is not colored 2, because it is not 4-directionally connected
 * to the starting pixel.
 *
 *
 *
 * Note:
 * The length of image and image[0] will be in the range [1, 50].
 * The given starting pixel will satisfy 0 <= sr < image.length and 0 <= sc < image[0].length.
 * The value of each color in image[i][j] and newColor will be an integer in [0, 65535].
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        if image.len() == 0 {
            return image;
        }
        let limit = (image.len(), image[0].len());
        let equal_color = image[sr as usize][sc as usize].clone();
        if equal_color == new_color {
            return image;
        }
        Solution::dfs_flood_fill(&mut image, sr, sc, new_color, equal_color, limit.0, limit.1);
        image
    }
    fn dfs_flood_fill(
        image: &mut Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        new_color: i32,
        equal_color: i32,
        r_size: usize,
        c_size: usize,
    ) {
        if image[sr as usize][sc as usize] != equal_color {
            return;
        }
        image[sr as usize][sc as usize] = new_color;
        if sr > 0 {
            Solution::dfs_flood_fill(image, sr - 1, sc, new_color, equal_color, r_size, c_size);
        }
        if sc > 0 {
            Solution::dfs_flood_fill(image, sr, sc - 1, new_color, equal_color, r_size, c_size);
        }
        if (sr + 1) < r_size as i32 {
            Solution::dfs_flood_fill(image, sr + 1, sc, new_color, equal_color, r_size, c_size);
        }
        if (sc + 1) < c_size as i32 {
            Solution::dfs_flood_fill(image, sr, sc + 1, new_color, equal_color, r_size, c_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_733() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
            vec![vec![0, 0, 0], vec![0, 1, 1]]
        );
    }
}
