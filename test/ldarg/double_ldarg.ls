
#OUTPUT
#Numbers: : 5.500000, 5.600000
#Numbers: : 1.100000, 1.200000
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    double x = ldarg(1, double)
    double y = ldarg(2, double)
    
    printf("%s: %f, %f\n", msg, x, y)
end

func main -> int
    double x = 1.1
    double y = 1.2
    
    print_num("Numbers: ", 5.5, 5.6)
    print_num("Numbers: ", x, y)
    
    return 0
end
