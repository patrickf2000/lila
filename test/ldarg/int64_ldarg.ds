
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    int64 x = ldarg(2, int64)
    int64 y = ldarg(3, int64)
    
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    int64 x = 4
    int64 y = 6
    
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", y, x)
    
    return 0
end
