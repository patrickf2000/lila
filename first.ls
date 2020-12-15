
#OUTPUT
#x + 5 = 25
#x - 5 = 15
#x * 5 = 100
#x / 5 = 4
#x % 6 = 2
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    int x = 20
    
    int a1 = x + 5
    int a2 = x - 5
    int a3 = x * 5
    int a4 = x / 5
    int a5 = x % 6

    printf("X: %d\n", x)
    printf("x + 5 = %d\n", a1)
    printf("x - 5 = %d\n", a2)
    printf("x * 5 = %d\n", a3)
    printf("x / 5 = %d\n", a4)
    printf("x % 6 = %d\n", a5)
    
    return 0
end

