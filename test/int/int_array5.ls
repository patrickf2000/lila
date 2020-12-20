
#OUTPUT
#X1: 5
#X2: 2
#X3: 18
#X4: 4
#X5: 5
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    numbers : int[10] = array
    base : int = 2
    x1 : int = 0
    x2 : int = 0
    x3 : int = 0
    x4 : int = 0
    x5 : int = 0
begin  
    numbers[1] = 3 + base
    numbers[2] = 3 - base
    numbers[3] = 3 * base
    numbers[4] = 3 / base
    numbers[5] = 3 % base
    
    x1 = numbers[1] * 1
    x2 = numbers[2] * 2
    x3 = numbers[3] * 3
    x4 = numbers[4] * 4
    x5 = numbers[5] * 5
    
    printf("X1: %d\n", x1)
    printf("X2: %d\n", x2)
    printf("X3: %d\n", x3)
    printf("X4: %d\n", x4)
    printf("X5: %d\n", x5)
    
    return 0
end
