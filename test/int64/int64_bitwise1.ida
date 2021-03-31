
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
    x : int64 = 4;
    a1 : int64 = 0;
    a2 : int64 = 0;
    a3 : int64 = 0;
    a4 : int64 = 0;
    a5 : int64 = 0;
begin
    
    a1 = x & 5;
    a2 = x | 5;
    a3 = x ^ 5;
    a4 = x << 2;
    a5 = x >> 2;
    
    printf("X = %d\n", x);
    printf("x & 5 = %x\n", a1);
    printf("x | 5 = %x\n", a2);
    printf("x ^ 5 = %x\n", a3);
    printf("x << 2 = %x\n", a4);
    printf("x >> 2 = %x\n", a5);
    
    return 0;
end

