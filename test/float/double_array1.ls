
#OUTPUT
#X: 3.140000
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    numbers : double[10];
    x : double = 0.0;
begin
    numbers[3] = 3.14;
    
    x = numbers[3];
    
    printf("X: %f\n", x);
    
    return 0;
end
