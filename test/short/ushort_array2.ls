
#OUTPUT
#X: 22
#X: 25
#END

#RET 0

extern func printf(s:str, ...)

func test1
    ushort[10] numbers = array
    
    numbers[5] = 22
    
    ushort x = numbers[5]
    
    printf("X: %d\n", x)
end

func test2
    ushort[10] numbers = array
    
    int i = 5
    numbers[i] = 25
    
    ushort x = numbers[i]
    
    printf("X: %d\n", x)
end

func main -> int
    test1()
    test2()
    
    return 0
end