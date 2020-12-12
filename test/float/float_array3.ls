
#OUTPUT
#X1: 5.290000
#X2: 0.990000
#X3: 6.751000
#X4: 1.460465
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    float[10] numbers = array
    float base = 2.15
    
    numbers[1] = 3.14 + base
    numbers[2] = 3.14 - base
    numbers[3] = 3.14 * base
    numbers[4] = 3.14 / base
    
    float x1 = numbers[1]
    float x2 = numbers[2]
    float x3 = numbers[3]
    float x4 = numbers[4]
    
    printf("X1: %f\n", x1)
    printf("X2: %f\n", x2)
    printf("X3: %f\n", x3)
    printf("X4: %f\n", x4)
    
    return 0
end
