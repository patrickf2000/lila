
#OUTPUT
#Syntax Error: Invalid use of subtraction operator.
# -> [17] aa = x - 5
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : ubyte = 20
    a1, aa, a2, a3, a4 : ubyte = 0
begin
    a1 = x + 5
    aa = x - 5
    a2 = x * 5
    a3 = x / 5
    a4 = x % 6
    
    printf("x + 5 = %d\n", a1)
    printf("x * 5 = %d\n", a2)
    printf("x / 5 = %d\n", a3)
    printf("x % 6 = %d\n", a4)
    
    return 0
end

