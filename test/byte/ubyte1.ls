
#OUTPUT
#B1: -14
#B2: 242
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    b1 : byte = 0xF2;
    b2 : ubyte = 0xF2;
begin
    
    printf("B1: %d\n", b1);
    printf("B2: %d\n", b2);
    
    return 0;
end

