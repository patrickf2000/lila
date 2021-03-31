
#OUTPUT
#X = 4
#x & 5 = 4
#x | 5 = 5
#x ^ 5 = 1
#x << 2 = 10
#x >> 2 = 1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x  : int = 4;
    a1 : int = x & 5;
    a2 : int = x | 5;
    a3 : int = x ^ 5;
    a4 : int = x << 2;
    a5 : int = x >> 2;
begin
    
    printf("X = %d\n", x);
    printf("x & 5 = %x\n", a1);
    printf("x | 5 = %x\n", a2);
    printf("x ^ 5 = %x\n", a3);
    printf("x << 2 = %x\n", a4);
    printf("x >> 2 = %x\n", a5);
    
    return 0;
end

