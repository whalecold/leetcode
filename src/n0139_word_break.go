// dp
func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s)+1)
	dp[0] = true
	for i := 1; i <= len(s); i++ {
		for j := 0; j < len(wordDict); j++ {
			if len(wordDict[j]) <= i && s[i-len(wordDict[j]):i] == wordDict[j] {
				dp[i] = dp[i] || dp[i-len(wordDict[j])]
			}
		}
	}
	return dp[len(s)]
}
