
#OUTPUT
#X: 20
#x + 5 = 25
#x - 5 = 15
#x * 5 = 100
#x / 5 = 4
#x % 6 = 2
#X = 4
#x & 5 = 4
#x | 5 = 5
#x ^ 5 = 1
#x << 2 = 10
#x >> 2 = 1
#END

#RET 0

extern func printf(s:str, ...)

func test1
    x : int = 20;
    a1, a2, a3, a4, a5 : int = 0;
begin
    a1 = x + 5;
    a2 = x - 5;
    a3 = x * 5;
    a4 = x / 5;
    a5 = x % 6;

    printf("X: %d\n", x);
    printf("x + 5 = %d\n", a1);
    printf("x - 5 = %d\n", a2);
    printf("x * 5 = %d\n", a3);
    printf("x / 5 = %d\n", a4);
    printf("x % 6 = %d\n", a5);
end

func test2
    x : int = 4;
    a1, a2, a3, a4, a5 : int = 0;
begin
    a1 = x & 5;
    a2 = x | 5;
    a3 = x ^ 5;
    a4 = x << 2;
    a5 = x >> 2;

    printf("X = %d\n", x);
    printf("x & 5 = %x\n", a1);
    printf("x | 5 = %x\n", a2);
    printf("x ^ 5 = %x\n", a3);
    printf("x << 2 = %x\n", a4);
    printf("x >> 2 = %x\n", a5);
end

func main -> int
begin
    test1();
    test2();
    return 0;
end
