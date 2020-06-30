import (
	"fmt"
	"strconv"
)

// 感觉不是一道好题目
func solveEquation(equation string) string {
	var xNums, integer, i int
	operator := map[byte]int{'+': 1, '-': -1}
	lastOp, digit := byte('+'), 0
	for ; i < len(equation); i += 1 {
		if equation[i] == '=' {
			integer += operator[lastOp] * digit
			operator['+'] = -1
			operator['-'] = 1
			lastOp, digit = '+', 0
			continue
		}

		if equation[i] == 'x' {
			if digit == 0 {
				if i > 0 && equation[i-1] == '0' {
					continue
				}
				digit = 1
			}
			xNums += operator[lastOp] * digit
			// fmt.Printf("x digit %v  %v\n", digit, xNums)
			digit = 0
		} else if equation[i] == '+' || equation[i] == '-' {
			integer += operator[lastOp] * digit
			// fmt.Printf("integer digit %v  %v\n", digit, integer)
			lastOp = equation[i]
			digit = 0
		} else {
			n, _ := strconv.Atoi(string(equation[i]))
			digit = 10*digit + n
		}
	}
	integer += operator[lastOp] * digit

	if xNums == 0 {
		if integer == 0 {
			return "Infinite solutions"
		}
		return "No solution"
	}
	return fmt.Sprintf("x=%v", -integer/xNums)
}
