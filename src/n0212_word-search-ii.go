// dfs 暴力破解法
// 可以借用 Prefix Tree 来优化代码 有点懒的实现了 - - ！
// Prefix Tree 实现见 ref: https://leetcode.com/problems/implement-trie-prefix-tree/
func findWords(board [][]byte, words []string) []string {
	boardMap := make(map[byte][][]int)
	for i := range board {
		for j := range board[i] {
			boardMap[board[i][j]] = append(boardMap[board[i][j]], []int{i, j})
		}
	}
	var ret []string
	for _, word := range words {
		if len(word) == 0 {
			continue
		}
		vals := boardMap[word[0]]
		for _, val := range vals {
			if dfs(board, word, val[0], val[1], 0) {
				ret = append(ret, word)
				break
			}
		}
	}
	return ret
}

func dfs(board [][]byte, word string, i, j, index int) bool {
	// fmt.Printf("index %v i %v j %v\n", index, i, j)
	if i < 0 || i >= len(board) || j < 0 || j >= len(board[i]) {
		return false
	}

	if board[i][j] == '.' {
		return false
	}

	if index >= len(word) || board[i][j] != word[index] {
		return false
	}

	if index == len(word)-1 {
		return true
	}
	board[i][j] = '.'
	var ret bool
	if dfs(board, word, i+1, j, index+1) || dfs(board, word, i, j+1, index+1) || dfs(board, word, i-1, j, index+1) || dfs(board, word, i, j-1, index+1) {
		ret = true
	}
	board[i][j] = word[index]
	return ret
}
