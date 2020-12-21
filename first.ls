
extern func printf(s:str, ...)

func main -> int
    x : int = 10;
begin
    printf("%d\n", x);
    
    x = 20;
    printf("%d\n", x);
    
    return 0
end

