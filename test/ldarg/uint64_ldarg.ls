
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x, y : uint64 = 0
begin
    x = ldarg(2, uint64)
    y = ldarg(3, uint64)
    
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    x : uint64 = 4
    y : uint64 = 6
begin
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", y, x)
    
    return 0
end
