
#OUTPUT
#Numbers: : FFFFB123, FFFFB234
#Numbers: : FFFFA123, FFFFA234
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    short x = ldarg(2, short)
    short y = ldarg(3, short)
    
    printf("%s: %X, %X\n", msg, x, y)
end

func main -> int
    short x = 0xA123
    short y = 0xA234
    
    print_num("Numbers: ", 0xB123, 0xB234)
    print_num("Numbers: ", x, y)
    
    return 0
end
