import "sort"

// 时间复杂度是 O(N*N) 空间复杂度是长数 比下面的解法更优
func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	ret := make([][]int, 0, len(nums))
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] > 0 {
			break
		}
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		l, r := i+1, len(nums)-1
		for l < r {
			sum := nums[i] + nums[l] + nums[r]
			if sum > 0 {
				r--
			} else if sum < 0 {
				l++
			} else {
				ret = append(ret, []int{nums[i], nums[l], nums[r]})
				for l+1 < len(nums) && nums[l+1] == nums[l] {
					l++
				}
				for r > i && nums[r] == nums[r-1] {
					r--
				}
				l++
				r--
			}
		}
	}
	return ret
}

// 这个办法时间复杂度是 O(N*N) 空间复杂度是 O(N)
func threeSum1(nums []int) [][]int {
	threeSums := make([][]int, 0, 10)
	sort.Ints(nums)
	set := make(map[int]struct{})
	for i, v := range nums {
		if len(nums)-i < 3 {
			break
		}
		_, ok := set[v]
		if ok {
			continue
		}
		set[v] = struct{}{}
		twoSums := twoSum3(nums[i+1:], -v)
		if len(twoSums) != 0 {
			for _, sum := range twoSums {
				sum = append(sum, v)

				threeSums = append(threeSums, sum)
			}
		}
	}
	return threeSums
}

func twoSum3(nums []int, target int) [][]int {
	twoSums := make([][]int, 0, 10)
	numMap := make(map[int]int)
	for index, num := range nums {
		if i, ok := numMap[num]; ok && i == index {
			continue
		}
		if _, ok := numMap[target-num]; ok {
			twoSums = append(twoSums, []int{num, target - num})
		}
		numMap[num] = index
	}
	return twoSums
}
