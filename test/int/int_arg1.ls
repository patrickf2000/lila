
#OUTPUT
#Result: 42
#END

#RET 0

extern func printf(s:str, ...)

func add_two(x:int, y:int)
    int answer = x + y
    printf("Result: %d\n", answer)
end

func main -> int
    int x = 22
    add_two(20, x)
    return 0
end