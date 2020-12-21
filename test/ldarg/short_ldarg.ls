
#OUTPUT
#Numbers: : FFFFB123, FFFFB234
#Numbers: : FFFFA123, FFFFA234
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x : short = ldarg(2, short);
    y : short = ldarg(3, short);
begin
    printf("%s: %X, %X\n", msg, x, y);
end

func main -> int
    x : short = 0xA123;
    y : short = 0xA234;
begin
    print_num("Numbers: ", 0xB123, 0xB234);
    print_num("Numbers: ", x, y);
    
    return 0;
end
