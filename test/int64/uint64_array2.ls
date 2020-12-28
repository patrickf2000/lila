
#OUTPUT
#X1: 5
#X2: 6
#X3: 1
#X4: 1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    numbers : uint64[10] = array;
    base : uint64 = 2;
    x1 : uint64 = 0;
    x2 : uint64 = 0;
    x3 : uint64 = 0;
    x4 : uint64 = 0;
begin
    
    numbers[1] = 3 + base;
    numbers[2] = 3 * base;
    numbers[3] = 3 / base;
    numbers[4] = 3 % base;
    
    x1 = numbers[1];
    x2 = numbers[2];
    x3 = numbers[3];
    x4 = numbers[4];
    
    printf("X1: %d\n", x1);
    printf("X2: %d\n", x2);
    printf("X3: %d\n", x3);
    printf("X4: %d\n", x4);
    
    return 0;
end
