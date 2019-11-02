/// We have two special characters. The first character can be represented by one bit 0.
/// The second character can be represented by two bits (10 or 11).
/// Now given a string represented by several bits.
/// Return whether the last character must be a one-bit character or not.
/// The given string will always end with a zero.

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut index = 0;
        loop {
            if index == bits.len() - 1 && bits[index] == 0 {
                return true;
            }
            if index >= bits.len() {
                break;
            }
            if bits[index] == 1 {
                index += 2;
            } else {
                index += 1;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_717() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
        assert_eq!(Solution::is_one_bit_character(vec![0]), true);
    }
}
