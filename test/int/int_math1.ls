
#OUTPUT
#Y: 16
#END

#RET 3

extern func printf(s:str, ...)

func main -> int
    x : int = 4;
    y : int = x * 3 + x;
begin
    
    printf("Y: %d\n", y);
    
    return 3;
end

