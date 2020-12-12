
#OUTPUT
#Answer: 15
#Answer: 9
#Answer1: 15
#Answer2: 18
#END

#RET 0

extern func printf(s:str, ...)

func add_two(x:int64, y:int64) -> int64
    int64 answer = x + y
    printf("Answer: %d\n", answer)
    return answer
end

func main -> int
    int64 answer1 = add_two(10, 5)
    int64 answer2 = add_two(6, 3) * 2
    
    printf("Answer1: %d\n", answer1)
    printf("Answer2: %d\n", answer2)
    
    return 0
end

