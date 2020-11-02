
#OUTPUT
#Syntax Error: Invalid use of subtraction operator.
# -> [17] numbers[2] = 3 - base
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint[10] numbers = array
    uint base = 2
    
    numbers[1] = 3 + base
    numbers[2] = 3 - base
    numbers[3] = 3 * base
    numbers[4] = 3 / base
    
    uint x1 = numbers[1]
    uint x2 = numbers[2]
    uint x3 = numbers[3]
    uint x4 = numbers[4]
    
    printf("X1: %d\n", x1)
    printf("X2: %d\n", x2)
    printf("X3: %d\n", x3)
    printf("X4: %d\n", x4)
    
    return 0
end
