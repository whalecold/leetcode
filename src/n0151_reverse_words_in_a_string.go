func reverseWords(s string) string {
    subs := make([]string, 0, 30)
    var str string
    for i := 0; i < len(s); i += 1 {
        if s[i] == ' '{
            if len(str) != 0 {
                subs = append(subs, str)
            }
            str = ""
        } else {
            str += string(s[i])
        }
    }
    if len(str) != 0 {
        subs = append(subs, str)
    }
    reverse(subs)
    return strings.Join(subs, " ")
}

func reverse(str []string) {
    l, r := 0, len(str)-1
    for l < r {
        str[l], str[r] = str[r], str[l]
        l += 1
        r -= 1
    }
}
