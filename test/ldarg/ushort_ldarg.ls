
#OUTPUT
#Numbers: : B123, B234
#Numbers: : A123, A234
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    ushort x = ldarg(2, ushort)
    ushort y = ldarg(3, ushort)
    
    printf("%s: %X, %X\n", msg, x, y)
end

func main -> int
    ushort x = 0xA123
    ushort y = 0xA234
    
    print_num("Numbers: ", 0xB123, 0xB234)
    print_num("Numbers: ", x, y)
    
    return 0
end
