
#OUTPUT
#Syntax Error: Invalid use of subtraction operator.
# -> [19] uint a5 = x - 5
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint x = 20
    
    uint a1 = x + 5
    uint a2 = x * 5
    uint a3 = x / 5
    uint a4 = x % 6
    uint a5 = x - 5
    
    printf("x + 5 = %d\n", a1)
    printf("x * 5 = %d\n", a2)
    printf("x / 5 = %d\n", a3)
    printf("x % 6 = %d\n", a4)
    
    return 0
end

