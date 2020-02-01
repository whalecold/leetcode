/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut sb = vec![String::new(); num_rows as usize];
        let length = s.len();

        let mut i = 0 as usize;
        let (mut down, mut up) = (0, num_rows - 2);
        while i < length {
            // down
            while i < length && down < num_rows {
                sb[down as usize].push(s.as_bytes()[i] as char);
                i += 1;
                down += 1;
            }
            down = 1;
            // up
            while i < length && up >= 0 {
                sb[up as usize].push(s.as_bytes()[i] as char);
                i += 1;
                up -= 1;
            }
            up = num_rows - 2;
        }
        for i in 1..sb.len() {
            sb[0] = sb[0].to_owned() + sb[i].to_owned().as_str();
        }
        sb[0].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"));
    }
}
