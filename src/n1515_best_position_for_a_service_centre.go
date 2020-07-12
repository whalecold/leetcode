import "math"

// 渐进方法：下次要记住了！
func getMinDistSum(positions [][]int) float64 {
	var right, top float64
	left, bottom := float64(100), float64(100)
	for _, pos := range positions {
		left = min(left, float64(pos[0]))
		right = max(right, float64(pos[0]))
		top = max(top, float64(pos[1]))
		bottom = min(bottom, float64(pos[1]))
	}
	sum := math.MaxFloat64
	var tempX, tempY float64
	for delta := float64(10); delta >= 0.000001; delta /= 10 {
		for x := float64(left); x <= right; x += delta {
			for y := float64(bottom); y <= top; y += delta {
				res := distances(positions, x, y)
				if res < sum {
					tempX = x
					tempY = y
					sum = res
				}
			}
		}

		left = tempX - delta
		right = tempX + delta
		top = tempY + delta
		bottom = tempY - delta
	}
	return sum
}

func distances(positions [][]int, x, y float64) float64 {
	var sum float64
	for _, pos := range positions {
		px, py := float64(pos[0]), float64(pos[1])
		sum += math.Pow((x-px)*(x-px)+(y-py)*(y-py), 0.5)
	}
	return sum
}

func max(i, j float64) float64 {
	if i > j {
		return i
	}
	return j
}

func min(i, j float64) float64 {
	if i < j {
		return i
	}
	return j
}
