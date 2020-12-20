
#OUTPUT
#Syntax Error: Invalid use of subtraction operator.
# -> [20] a5 = x - 5
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : uint64 = 20
    a1, a2, a3, a4, a5 : uint64 = 0
begin
    a1 = x + 5
    a2 = x * 5
    a3 = x / 5
    a4 = x % 6
    a5 = x - 5
    
    printf("x + 5 = %d\n", a1)
    printf("x * 5 = %d\n", a2)
    printf("x / 5 = %d\n", a3)
    printf("x % 6 = %d\n", a4)
    
    return 0
end

