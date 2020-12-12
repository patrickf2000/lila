
#OUTPUT
#Syntax Error: Invalid use of subtraction operator.
# -> [16] ubyte aa = x - 5
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    ubyte x = 20
    
    ubyte a1 = x + 5
    ubyte aa = x - 5
    ubyte a2 = x * 5
    ubyte a3 = x / 5
    ubyte a4 = x % 6
    
    printf("x + 5 = %d\n", a1)
    printf("x * 5 = %d\n", a2)
    printf("x / 5 = %d\n", a3)
    printf("x % 6 = %d\n", a4)
    
    return 0
end

