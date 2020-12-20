
extern func printf(s:str, ...)

func main -> int
    x : int = 20
    a1, a2, a3, a4, a5 : int = 0
begin
    
    a1 = x + 5
    a2 = x - 5
    a3 = x * 5
    a4 = x / 5
    a5 = x % 6
    
    printf("x + 5 = %d\n", a1)
    printf("x - 5 = %d\n", a2)
    printf("x * 5 = %d\n", a3)
    printf("x / 5 = %d\n", a4)
    printf("x % 6 = %d\n", a5)
    
    return 0
end

