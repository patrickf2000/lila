
#OUTPUT
#X: 22
#X: 25
#END

#RET 0

extern func printf(s:str, ...)

func test1
    numbers : uint64[10];
    i : int = 5;
    x : uint64 = 0;
begin
    numbers[i+1] = 22;
    
    x = numbers[6];
    
    printf("X: %d\n", x);
end

func test2
    numbers : uint64[10];
    x : uint64 = 0;
    i : int = 5;
begin
    numbers[6] = 25;
    
    x = numbers[i+1];
    
    printf("X: %d\n", x);
end

func main -> int
begin
    test1();
    test2();
    
    return 0;
end
