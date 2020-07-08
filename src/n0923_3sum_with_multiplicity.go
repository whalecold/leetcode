// 和 3sum 差不多 需要注意 A[l] == A[r] 的情况
func threeSumMulti(A []int, target int) int {
	factorial := make([]int, len(A))
	for i := 1; i < len(factorial); i++ {
		factorial[i] = factorial[i-1] + i
	}
	mod := int(1e9 + 7)
	var ret int
	for i := 0; i < len(A)-2; i++ {
		if A[i] > target {
			break
		}

		l, r := i+1, len(A)-1
		for l < r {
			sum := A[i] + A[l] + A[r]
			if sum > target {
				r--
			} else if sum < target {
				l++
			} else {
				if A[l] == A[r] {
					ret += factorial[r-l]
					ret %= mod
					break
				}
				lNum, rNum := 1, 1
				for l+1 < len(A) && A[l+1] == A[l] {
					l++
					lNum++
				}
				for r > i && A[r] == A[r-1] {
					r--
					rNum++
				}
				l++
				r--
				ret += lNum * rNum
				ret %= mod
			}
		}
	}
	return ret
}
