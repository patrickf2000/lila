
#OUTPUT
#X1: 5
#X2: 6
#X3: 1
#X4: 1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint64[10] numbers = array
    uint64 base = 2
    
    numbers[1] = 3 + base
    numbers[2] = 3 * base
    numbers[3] = 3 / base
    numbers[4] = 3 % base
    
    uint64 x1 = numbers[1]
    uint64 x2 = numbers[2]
    uint64 x3 = numbers[3]
    uint64 x4 = numbers[4]
    
    printf("X1: %d\n", x1)
    printf("X2: %d\n", x2)
    printf("X3: %d\n", x3)
    printf("X4: %d\n", x4)
    
    return 0
end
