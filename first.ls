
extern func printf(s:str, ...)

func main -> int
    x : int = 10;
    
    enum Token = X, Y, Z;
    t : Token = X;
begin
    printf("%d\n", x);
    printf("T: %d\n", t);
    
    t = Y;
    printf("T: %d\n", t);
    
    return 0;
end

