
#OUTPUT
#Numbers: : 10, 20
#Numbers: : 6, 4
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    int x = ldarg(2, int)
    int y = ldarg(3, int)
    
    printf("%s: %d, %d\n", msg, x, y)
end

func main -> int
    int x = 4
    
    print_num("Numbers: ", 10, 20)
    print_num("Numbers: ", 6, x)
    
    return 0
end
