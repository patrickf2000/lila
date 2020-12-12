
#OUTPUT
#X: 3.140000
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    double[10] numbers = array
    
    numbers[3] = 3.14
    
    double x = numbers[3]
    
    printf("X: %f\n", x)
    
    return 0
end
