
#OUTPUT
#B1: -95
#B2: -81
#END

#RET 0

extern func printf(s:str, ...)

func byte1 -> byte
begin
    return 0xA1;
end

func byte2 -> byte
    x : byte = 0xAF;
begin
    return x;
end

func main -> int
    b1, b2 : byte = 0;
begin
    b1 = byte1();
    b2 = byte2();
    
    printf("B1: %d\n", b1);
    printf("B2: %d\n", b2);
    
    return 0;
end

