
#OUTPUT
#123
#10
#88
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : int64 = 123;
begin
    
    printf("%d\n", x);
    printf("%d\n", 10);
    
    x = 88;
    printf("%d\n", x);
    
    return 0;
end
