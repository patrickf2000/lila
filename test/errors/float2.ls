
#OUTPUT
#Syntax Error: Modulo is only valid with integer values.
# -> [19] float a5 = x % 5.4
#
#END

#RET 0

extern func printf(s:str, ...)


func main -> int
    float x = 3.14
    float a1 = x + 5.4
    float a2 = x - 5.4
    float a3 = x * 5.4
    float a4 = x / 5.4
    float a5 = x % 5.4
    
    printf("X: %f\n", x)
    printf("X + 5.4 = %f\n", a1)
    printf("X - 5.4 = %f\n", a2)
    printf("X * 5.4 = %f\n", a3)
    printf("X / 5.4 = %f\n", a4)
    
    return 0
end
