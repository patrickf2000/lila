
#OUTPUT
#Numbers: : B123, B234
#Numbers: : A123, A234
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x : ushort = ldarg(2, ushort);
    y : ushort = ldarg(3, ushort);
begin
    printf("%s: %X, %X\n", msg, x, y);
end

func main -> int
    x : ushort = 0xA123;
    y : ushort = 0xA234;
begin
    print_num("Numbers: ", 0xB123, 0xB234);
    print_num("Numbers: ", x, y);
    
    return 0;
end
