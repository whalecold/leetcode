
func numSub(s string) int {
	// 连续 n 个 1 所包含的数量是固定的，所以先把这个
	// 值算出来，下面要用的时候直接取值就好了
	data := make([]int, len(s)+1)
	for i := 1; i <= len(s); i++ {
		data[i] = i + data[i-1]
	}
	var sum, nums int
	for i := 0; i < len(s); i++ {
		if s[i] == '1' {
			nums++
			continue
		}
		sum += data[nums]
		nums = 0
	}
	// 注意下最后
	if nums != 0 {
		sum += data[nums]
	}
	mod := int(1e9 + 7)
	return sum % mod
}
