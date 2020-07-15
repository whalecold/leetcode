func scoreOfParentheses(S string) int {
	stack := make([]interface{}, 0, len(S))
	for i := range S {
		if S[i] == '(' {
			stack = append(stack, S[i])
			continue
		}
		if S[i] == ')' {
			var num int
			for {
				// get all number
				n, ok := stack[len(stack)-1].(int)
				if !ok {
					break
				}
				num += n
				stack = stack[:len(stack)-1]
			}
			// remove '('
			stack = stack[:len(stack)-1]
			if num == 0 {
				stack = append(stack, 1)
			} else {
				stack = append(stack, num*2)
			}
		}
	}
	var score int
	for _, num := range stack {
		if n, ok := num.(int); ok {
			score += n
		}
	}
	return score
}
