import (
	"fmt"
	"strings"
)

func reformatDate(date string) string {
	monthMap := map[string]string{
		"Jan": "01",
		"Feb": "02",
		"Mar": "03",
		"Apr": "04",
		"May": "05",
		"Jun": "06",
		"Jul": "07",
		"Aug": "08",
		"Sep": "09",
		"Oct": "10",
		"Nov": "11",
		"Dec": "12",
	}
	res := strings.Split(date, " ")
	var index int
	for ; index < len(res[0]); index++ {
		if res[0][index] > '9' || res[0][index] < '0' {
			break
		}
	}
	day := string(res[0][:index])
	if index == 1 {
		day = "0" + day
	}
	return fmt.Sprintf("%s-%s-%s", res[2], monthMap[res[1]], day)
}
