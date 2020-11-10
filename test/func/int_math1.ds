
#OUTPUT
#T: 5
#T: 105
#END

#RET 0

extern func printf(s:str, ...)

func get_num -> int
    int x = 20
    int y = 30
    int answer = x + y
    return answer
end

# Output should be 5, then 105
func main -> int
    int total = 0
    
    total = total + 5
    printf("T: %d\n", total)
    
    total = total + get_num()
    total = total + get_num()
    
    printf("T: %d\n", total)
    
    return 0
end

