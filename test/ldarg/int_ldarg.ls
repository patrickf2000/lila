
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x, y : int = 0
begin
    x = ldarg(2, int)
    y = ldarg(3, int)
    
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    x : int = 4
begin
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", 6, x)
    
    return 0
end
