/**
 * [60] Permutation Sequence
 *
 * The set [1,2,3,...,n] contains a total of n! unique permutations.
 *
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 *
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 *
 * Given n and k, return the k^th permutation sequence.
 *
 * Note:
 *
 *
 * 	Given n will be between 1 and 9 inclusive.
 * 	Given k will be between 1 and n! inclusive.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 3, k = 3
 * Output: "213"
 *
 *
 * Example 2:
 *
 *
 * Input: n = 4, k = 9
 * Output: "2314"
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let (mut factorial, mut str) = (vec![1; n + 1], String::new());
        // gets factorial array and digit string
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
            str.push(std::char::from_digit(i as u32, 10).unwrap());
        }

        // gets the index of k, so we need sub by 1.
        let (mut ret, mut k) = (String::new(), (k - 1) as usize);
        for i in (1..n).rev() {
            let index = k / factorial[i];
            ret.push(str.chars().nth(index).unwrap());
            str.remove(index);
            k = k % factorial[i];
        }
        ret.push(str.chars().nth(0).unwrap());

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(4, 6), String::from("1432"));
        assert_eq!(Solution::get_permutation(3, 3), String::from("213"));
        assert_eq!(Solution::get_permutation(3, 1), String::from("123"));
        assert_eq!(Solution::get_permutation(4, 9), String::from("2314"));
        assert_eq!(
            Solution::get_permutation(9, 273815),
            String::from("783269514")
        );
    }
}
