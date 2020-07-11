func totalHammingDistance(nums []int) int {
    var total int
    for offset := 0; offset < 32; offset++ {
        var count int
        for i := 0; i < len(nums); i++ {
            count += (nums[i] >> offset) & 1
        }
        total += (len(nums)-count) * count
    }
    return total
}

// 我就猜到这样会 TLE fuck
// 1.先加 cache
// 2.用二分法？？？
// 3.我真的是太蠢了！
func totalHammingDistance1(nums []int) int {
    cache := make(map[string]int)
    var sum int
    for i := 0; i < len(nums); i++ {
        for j := i+1; j < len(nums); j++ {
            if nums[i] == nums[j] {
                continue
            }
            sum += hammingDistance(nums[i], nums[j], cache)
        }
    }
    return sum
}

func hammingDistance(m, n int, cache map[string]int) int {
    key := fmt.Sprintf("%v_%v", m, n)
    if m > n {
        key = fmt.Sprintf("%v_%v", n, m)
    }
    if d, ok := cache[key]; ok {
        return d
    }
    var sum int
    for m != 0 || n != 0 {
        if m & 1 != n & 1 {
            sum++
        }
        m >>= 1
        n >>= 1
    }
    cache[key] = sum
    return sum
}
