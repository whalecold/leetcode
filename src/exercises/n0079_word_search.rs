/**
 * [79] Word Search
 *
 * Given a 2D board and a word, find if the word exists in the grid.
 *
 * The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *
 * Example:
 *
 *
 * board =
 * [
 *   ['A','B','C','E'],
 *   ['S','F','C','S'],
 *   ['A','D','E','E']
 * ]
 *
 * Given word = "ABCCED", return true.
 * Given word = "SEE", return true.
 * Given word = "ABCB", return false.
 *
 *
 */
use std::collections::{HashSet};

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() || word.is_empty() {
            return false;
        }
        let word = word.as_bytes();
        let mut set = HashSet::new();
        let board = board;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == word[0].to_owned() as char {
                    if Solution::back_track(i, j, 1, &mut set, &board, word) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn back_track(
        x: usize,
        y: usize,
        word_index: usize,
        set: &mut HashSet<[usize; 2]>,
        board: &Vec<Vec<char>>,
        word: &[u8],
    ) -> bool {
        if word_index == word.len() {
            return true;
        }
        let (x_len, y_len) = (board.len(), board[0].len());
        set.insert([x, y]);
        if x > 0
            && board[x - 1][y] == word[word_index].to_owned() as char
            && !set.contains(&[x - 1, y])
        {
            if Solution::back_track(x - 1, y, word_index + 1, set, board, word) {
                return true;
            }
        }
        if x + 1 < x_len
            && board[x + 1][y] == word[word_index].to_owned() as char
            && !set.contains(&[x + 1, y])
        {
            if Solution::back_track(x + 1, y, word_index + 1, set, board, word) {
                return true;
            }
        }
        if y > 0
            && board[x][y - 1] == word[word_index].to_owned() as char
            && !set.contains(&[x, y - 1])
        {
            if Solution::back_track(x, y - 1, word_index + 1, set, board, word) {
                return true;
            }
        }
        if y + 1 < y_len
            && board[x][y + 1] == word[word_index].to_owned() as char
            && !set.contains(&[x, y + 1])
        {
            if Solution::back_track(x, y + 1, word_index + 1, set, board, word) {
                return true;
            }
        }
        set.remove(&[x, y]);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_79() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(
            Solution::exist(board.to_owned(), String::from("ABCCED")),
            true
        );
        assert_eq!(
            Solution::exist(board.to_owned(), String::from("SFCS")),
            true
        );
        assert_eq!(
            Solution::exist(board.to_owned(), String::from("ADEE")),
            true
        );
        assert_eq!(Solution::exist(board.to_owned(), String::from("SEE")), true);
        assert_eq!(
            Solution::exist(board.to_owned(), String::from("SEECCE")),
            true
        );
    }
}
