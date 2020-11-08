
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    uint x = ldarg(2, uint)
    uint y = ldarg(3, uint)
    
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    uint x = 4
    uint y = 6
    
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", y, x)
    
    return 0
end
