// 一个比较典型的 dp 问题，从 1 到 n 每次都把结果算出来，
// 计算到 n+1 的时候第一步尝试放小于等于 n 的所有 square 数量，
// 因为对手的放的 stone 肯定是算过的了，所以看下对手能不能赢，自己的结果
// 取反就好了！！！
func winnerSquareGame(n int) bool {
	dp := make([]bool, n+1)
	lastSquare := 1
	for i := 1; i <= n; i++ {
		if lastSquare*lastSquare == i {
			dp[i] = true
			lastSquare += 1
			continue
		}
		for j := 1; j < lastSquare; j++ {
			if !dp[i-j*j] {
				dp[i] = true
				break
			}
		}
	}
	return dp[n]
}
