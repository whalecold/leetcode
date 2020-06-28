import "sort"

func findItinerary(tickets [][]string) []string {
	sort.Slice(tickets, func(i, j int) bool {
		return tickets[i][1] < tickets[j][1]
	})
	airMap := make(map[string][]string, len(tickets))
	for _, t := range tickets {
		airMap[t[0]] = append(airMap[t[0]], t[1])
	}
	route := make([]string, 0, len(tickets))
	slack := make([]string, 0, len(route))
	slack = append(slack, "JFK")

	for len(slack) != 0 {
		last := slack[len(slack)-1]
		// fmt.Printf("slack %v...\n", slack)
		airs := airMap[last]
		// fmt.Printf("airs %v...\n", airs)
		if len(airs) == 0 {
			route = append([]string{last}, route...)
			slack = slack[:len(slack)-1]
		} else {
			next := airs[0]
			airs = airs[1:]
			airMap[last] = airs
			slack = append(slack, next)
		}
	}
	return route
}

// 思路是先找到最后一个终点，然后把终点放入队列，因为是 dfs 所以终点肯定在最后，找到之后不断的回溯，一直到起点
// 这个是递归的版本 上面的是遍历的版本
func findItinerary1(tickets [][]string) []string {
	sort.Slice(tickets, func(i, j int) bool {
		return tickets[i][1] < tickets[j][1]
	})
	airMap := make(map[string][]string, len(tickets))

	for _, t := range tickets {
		airMap[t[0]] = append(airMap[t[0]], t[1])
	}
	route := make([]string, 0, len(tickets))
	dfs("JFK", airMap, &route)
	return route
}

func dfs(airport string, airMap map[string][]string, route *[]string) {
	for {
		airs := airMap[airport]
		if len(airs) == 0 {
			*route = append([]string{airport}, *route...)
			break
		}
		next := airs[0]
		airs = airs[1:]
		airMap[airport] = airs
		dfs(next, airMap, route)
	}

}
