
#OUTPUT
#Numbers: : 5.500000, 5.600000
#Numbers: : 1.100000, 1.200000
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    float x = ldarg(1, float)
    float y = ldarg(2, float)
    
    printf("%s: %f, %f\n", msg, x, y)
end

func main -> int
    float x = 1.1
    float y = 1.2
    
    print_num("Numbers: ", 5.5, 5.6)
    print_num("Numbers: ", x, y)
    
    return 0
end