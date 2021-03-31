
#OUTPUT
#X: 22
#X: 25
#END

#RET 0

extern func printf(s:str, ...)

func test1
    numbers : uint64[10];
    x : uint64 = 0;
begin
    numbers[3] = 22;
    
    x = numbers[3];
    
    printf("X: %d\n", x);
end

func test2
    numbers : uint64[10];
    x : uint64 = 0;
    i : int = 5;
begin
    numbers[i] = 25;
    
    x = numbers[i];
    
    printf("X: %d\n", x);
end

func main -> int
begin
    test1();
    test2();
    
    return 0;
end
