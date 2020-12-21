
#OUTPUT
#X1: 22
#X2: 132
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : int = 22;
    y : int = 3;
begin
    
    printf("X1: %d\n", x);

    x = 44 * y;
    
    printf("X2: %d\n", x);
    
    return 0;
end
