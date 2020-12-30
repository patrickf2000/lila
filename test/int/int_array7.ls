
#OUTPUT
#X: 22
#Sizeof numbers: 10
#END

#RET 0

extern func printf(s:str, ...)

func print_num(numbers:int[])
    x, length : int = 0;
begin
    x = numbers[3];
    printf("X: %d\n", x);
    
    length = sizeof(numbers);
    printf("Sizeof numbers: %d\n", length);
end

func test1
    numbers : int[10];
    x, length : int = 0;
begin    
    numbers[3] = 22;
    
    print_num(numbers);
end

func main -> int
begin
    test1();
    
    return 0;
end
