
#OUTPUT
#X1: 5
#X2: 1
#X3: 6
#X4: 1
#X5: 1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    int64[10] numbers = array
    int64 base = 2
    
    numbers[1] = 3 + base
    numbers[2] = 3 - base
    numbers[3] = 3 * base
    numbers[4] = 3 / base
    numbers[5] = 3 % base
    
    int64 x1 = numbers[1]
    int64 x2 = numbers[2]
    int64 x3 = numbers[3]
    int64 x4 = numbers[4]
    int64 x5 = numbers[5]
    
    printf("X1: %d\n", x1)
    printf("X2: %d\n", x2)
    printf("X3: %d\n", x3)
    printf("X4: %d\n", x4)
    printf("X5: %d\n", x5)
    
    return 0
end
