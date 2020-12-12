
#OUTPUT
#X: A1B1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    ushort[10] numbers = array
    
    numbers[3] = 0xA1B1
    
    ushort x = numbers[3]
    
    printf("X: %X\n", x)
    
    return 0
end
