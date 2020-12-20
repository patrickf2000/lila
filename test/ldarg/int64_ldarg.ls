
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x : int64 = ldarg(2, int64)
    y : int64 = ldarg(3, int64)
begin
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    x : int64 = 4
    y : int64 = 6
begin
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", y, x)
    
    return 0
end
