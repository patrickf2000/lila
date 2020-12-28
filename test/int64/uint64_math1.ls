
#OUTPUT
#x + 5 = 25
#x * 5 = 100
#x / 5 = 4
#x % 6 = 2
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : uint64 = 20;
    a1 : uint64 = 0;
    a2 : uint64 = 0;
    a3 : uint64 = 0;
    a4 : uint64 = 0;
begin
    
    a1 = x + 5;
    a2 = x * 5;
    a3 = x / 5;
    a4 = x % 6;
    
    printf("x + 5 = %d\n", a1);
    printf("x * 5 = %d\n", a2);
    printf("x / 5 = %d\n", a3);
    printf("x % 6 = %d\n", a4);
    
    return 0;
end

