
#OUTPUT
#X: A1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    numbers : ubyte[10] = array;
    x : ubyte = 0;
begin
    
    numbers[3] = 0xA1;
    
    x = numbers[3];
    
    printf("X: %X\n", x);
    
    return 0;
end
