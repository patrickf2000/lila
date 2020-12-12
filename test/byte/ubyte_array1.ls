
#OUTPUT
#X: A1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    ubyte[10] numbers = array
    
    numbers[3] = 0xA1
    
    ubyte x = numbers[3]
    
    printf("X: %X\n", x)
    
    return 0
end
