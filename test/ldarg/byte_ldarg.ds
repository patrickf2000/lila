
#OUTPUT
#Numbers: : FFFFFFB1, FFFFFFB2
#Numbers: : FFFFFFA1, FFFFFFA2
#END

#RET 0

extern func printf(s:str, ...)

func print_num(msg:str, ...)
    byte x = ldarg(2, byte)
    byte y = ldarg(3, byte)
    
    printf("%s: %X, %X\n", msg, x, y)
end

func main -> int
    byte x = 0xA1
    byte y = 0xA2
    
    print_num("Numbers: ", 0xB1, 0xB2)
    print_num("Numbers: ", x, y)
    
    return 0
end
