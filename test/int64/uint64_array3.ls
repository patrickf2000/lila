
#OUTPUT
#X: 22
#X: 25
#END

#RET 0

extern func printf(s:str, ...)

func test1
    uint64[10] numbers = array
    
    uint64 i = 5
    numbers[i+1] = 22
    
    uint64 x = numbers[6]
    
    printf("X: %d\n", x)
end

func test2
    uint64[10] numbers = array
    
    uint64 i = 5
    numbers[6] = 25
    
    uint64 x = numbers[i+1]
    
    printf("X: %d\n", x)
end

func main -> int
    test1()
    test2()
    
    return 0
end
