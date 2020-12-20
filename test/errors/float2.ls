
#OUTPUT
#Syntax Error: Modulo is only valid with integer values.
# -> [21] a5 = x % 5.4
#
#END

#RET 0

extern func printf(s:str, ...)


func main -> int
    x : float = 3.14
    a1, a2, a3, a4, a5 : float = 0.0
begin
    a1 = x + 5.4
    a2 = x - 5.4
    a3 = x * 5.4
    a4 = x / 5.4
    a5 = x % 5.4
    
    printf("X: %f\n", x)
    printf("X + 5.4 = %f\n", a1)
    printf("X - 5.4 = %f\n", a2)
    printf("X * 5.4 = %f\n", a3)
    printf("X / 5.4 = %f\n", a4)
    
    return 0
end

