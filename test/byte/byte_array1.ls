
#OUTPUT
#X: FFFFFFA1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    byte[10] numbers = array
    
    numbers[3] = 0xA1
    
    byte x = numbers[3]
    
    printf("X: %X\n", x)
    
    return 0
end