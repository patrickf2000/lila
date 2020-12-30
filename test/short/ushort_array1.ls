
#OUTPUT
#X: A1B1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    numbers : ushort[10];
    x : ushort = 0;
begin
    numbers[3] = 0xA1B1;
    
    x = numbers[3];
    
    printf("X: %X\n", x);
    
    return 0;
end
