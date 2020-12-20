
#OUTPUT
#Numbers: : B1, B2
#Numbers: : A1, A2
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    x, y : ubyte = 0
begin
    x = ldarg(2, ubyte)
    y = ldarg(3, ubyte)
    
    printf("%s: %X, %X\n", msg, x, y)
end

func main -> int
    x : ubyte = 0xA1
    y : ubyte = 0xA2
begin
    print_num("Numbers: ", 0xB1, 0xB2)
    print_num("Numbers: ", x, y)
    
    return 0
end
