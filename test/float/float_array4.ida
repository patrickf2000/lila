
#OUTPUT
#X: 22.000000
#X: 25.000000
#END

#RET 0

extern func printf(s:str, ...)

func test1
    numbers : float[10];
    x : float = 0.0;
    i : int = 5;
begin
    numbers[i+1] = 22.0;
    
    x = numbers[6];
    
    printf("X: %f\n", x);
end

func test2
    numbers : float[10];
    x : float = 0.0;
    i : int = 5;
begin
    numbers[6] = 25.0;
    
    x = numbers[i+1];
    
    printf("X: %f\n", x);
end

func main -> int
begin
    test1();
    test2();
    
    return 0;
end
