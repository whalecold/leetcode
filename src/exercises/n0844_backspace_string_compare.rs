/**
 * [844] Backspace String Compare
 *
 * Given two strings S and T, return if they are equal when both are typed into empty text editors. # means a backspace character.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: S = <span id="example-input-1-1">"ab#c"</span>, T = <span id="example-input-1-2">"ad#c"</span>
 * Output: <span id="example-output-1">true
 * </span><span>Explanation: Both S and T become "ac".</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: S = <span id="example-input-2-1">"ab##"</span>, T = <span id="example-input-2-2">"c#d#"</span>
 * Output: <span id="example-output-2">true
 * </span><span>Explanation: Both S and T become "".</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: S = <span id="example-input-3-1">"a##c"</span>, T = <span id="example-input-3-2">"#a#c"</span>
 * Output: <span id="example-output-3">true
 * </span><span>Explanation: Both S and T become "c".</span>
 *
 *
 * <div>
 * Example 4:
 *
 *
 * Input: S = <span id="example-input-4-1">"a#c"</span>, T = <span id="example-input-4-2">"b"</span>
 * Output: <span id="example-output-4">false
 * </span><span>Explanation: S becomes "c" while T becomes "b".</span>
 *
 *
 * <span>Note:</span>
 *
 * <ol>
 * 	<span>1 <= S.length <= 200</span>
 * 	<span>1 <= T.length <= 200</span>
 * 	<span>S and T only contain lowercase letters and '#' characters.</span>
 * </ol>
 *
 * Follow up:
 *
 *
 * 	Can you solve it in O(N) time and O(1) space?
 *
 * </div>
 * </div>
 * </div>
 * </div>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        // O(N) time and O(N) space
        // let calc = |x: &String| -> Vec<_> {
        //     let mut temp = vec![];
        //     for i in 0..x.len() {
        //         if x.chars().nth(i).unwrap().eq(&'#') {
        //             if temp.len() > 0 {
        //                 temp.pop();
        //             }
        //             continue;
        //         }
        //         temp.push(x.chars().nth(i).unwrap());
        //     }
        //     temp
        // };
        // return calc(&s) == calc(&t);
        // O(n) time O(1) space
        let (mut l_s, mut l_t) = (s.len() as i32 - 1, t.len() as i32 - 1);
        while l_s >= 0 || l_t >= 0 {
            let calc = |x: &String, index: &mut i32| -> i32 {
                let mut backspace_num = 0;
                while *index >= 0 {
                    let ch = x.chars().nth(*index as usize).unwrap();
                    if !ch.eq(&'#') && backspace_num == 0 {
                        break;
                    }
                    if ch.eq(&'#') {
                        backspace_num += 1;
                    } else {
                        backspace_num -= 1;
                    }
                    *index -= 1;
                }
                *index
            };
            calc(&s, &mut l_s);
            calc(&t, &mut l_t);
            if l_s < 0 || l_t < 0 {
                break;
            }
            if !s
                .chars()
                .nth(l_s as usize)
                .unwrap()
                .eq(&t.chars().nth(l_t as usize).unwrap())
            {
                return false;
            }
            l_s -= 1;
            l_t -= 1;
        }
        l_s == l_t
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_844() {
        assert_eq!(
            Solution::backspace_compare(String::from("#a#c"), String::from("##c")),
            true
        );
        assert_eq!(
            Solution::backspace_compare(String::from("nzp#o#g"), String::from("b#nzp#o#g")),
            true
        );
    }
}
