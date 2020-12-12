
#OUTPUT
#X: 22.000000
#X: 25.000000
#END

#RET 0

extern func printf(s:str, ...)

func test1
    float[10] numbers = array
    
    int i = 5
    numbers[i+1] = 22.0
    
    float x = numbers[6]
    
    printf("X: %f\n", x)
end

func test2
    float[10] numbers = array
    
    int i = 5
    numbers[6] = 25.0
    
    float x = numbers[i+1]
    
    printf("X: %f\n", x)
end

func main -> int
    test1()
    test2()
    
    return 0
end
