/**
 * [71] Simplify Path
 *
 * Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the canonical path.
 * In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double period .. moves the directory up a level.
 * Note that the returned canonical path must always begin with a slash /, and there must be only a single slash / between two directory names. The last directory name (if it exists) must not end with a trailing /. Also, the canonical path must be the shortest string representing the absolute path.
 *  
 * Example 1:
 *
 * Input: "<span id="example-input-1-1">/home/"</span>
 * Output: "<span id="example-output-1">/home"
 * Explanation: Note that there is no trailing slash after the last directory name.</span>
 *
 * Example 2:
 *
 * Input: "<span id="example-input-1-1">/../"</span>
 * Output: "<span id="example-output-1">/"</span>
 * Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
 *
 * Example 3:
 *
 * Input: "<span id="example-input-1-1">/home//foo/"</span>
 * Output: "<span id="example-output-1">/home/foo"</span>
 * Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
 *
 * Example 4:
 *
 * Input: "<span id="example-input-1-1">/a/./b/../../c/"</span>
 * Output: "<span id="example-output-1">/c"</span>
 *
 * Example 5:
 *
 * Input: "<span id="example-input-1-1">/a/../../b/../c//.//"</span>
 * Output: "<span id="example-output-1">/c"</span>
 *
 * Example 6:
 *
 * Input: "<span id="example-input-1-1">/a//b////c/d//././/.."</span>
 * Output: "<span id="example-output-1">/a/b/c"</span>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn simplify_path(path: String) -> String {
        const SLASH: char = '/';
        let mut str = vec![];
        let skip = vec![".", "", ".."];
        let sub_paths: Vec<&str> = path.split(SLASH).collect();
        for i in 0..sub_paths.len() {
            if sub_paths[i] == ".." && !str.is_empty() {
                str.pop();
            }
            if !skip.contains(&sub_paths[i]) {
                str.push(sub_paths[i]);
            }
        }
        let mut ret = String::new();
        for i in 0..str.len() {
            ret = ret + "/" + str[i];
        }
        if ret.len() == 0 {
            String::from("/")
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_71() {
        assert_eq!(
            Solution::simplify_path(String::from("/...")),
            String::from("/...")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/..")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home///foo/")),
            String::from("/home/foo")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a//b////c/d//././/..")),
            String::from("/a/b/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/.")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a/../../b/../c//.//")),
            String::from("/c")
        );
    }
}
