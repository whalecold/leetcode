/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut start = -1;
        let mut end = 0;
        for i in (0..nums.len()).rev() {
            if i != nums.len() - 1 && nums[i] < nums[i + 1] {
                start = i as i32;
                break;
            }
        }
        if start == -1 {
            nums.reverse();
            return;
        }
        for i in (0..nums.len()).rev() {
            if nums[i] > nums[start as usize] {
                end = i;
                break;
            }
        }
        nums.swap(start as usize, end as usize);
        let mut start = start as usize + 1;
        end = nums.len() - 1;
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {}
}
