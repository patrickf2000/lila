
#OUTPUT
#X: 3.140000
#X + 5.4 = 8.540000
#X - 5.4 = -2.260000
#X * 5.4 = 16.956001
#X / 5.4 = 0.581482
#END

#RET 0

extern func printf(s:str, ...)


func main -> int
    x : float = 3.14
    a1, a2, a3, a4 : float = 0.0
begin
    a1 = x + 5.4
    a2 = x - 5.4
    a3 = x * 5.4
    a4 = x / 5.4
    
    printf("X: %f\n", x)
    printf("X + 5.4 = %f\n", a1)
    printf("X - 5.4 = %f\n", a2)
    printf("X * 5.4 = %f\n", a3)
    printf("X / 5.4 = %f\n", a4)
    
    return 0
end

