
#OUTPUT
#X: -15
#X: -9
#Y: 20
#X: 9
#Y: -9
#X: 10
#Y: 10
#END

#RET 0

extern func printf(s:str, ...)

func test1
    x : int = -6 + -9;
begin
    printf("X: %d\n", x);
end

func test2
    x : int = -9;
    y : int = 20 + x - -9;
begin
    printf("X: %d\n", x);
    printf("Y: %d\n", y);
end

func test3
    x : int = 9;
    y : int = -x;
begin  
    printf("X: %d\n", x);
    printf("Y: %d\n", y);
end

func test4
    x : int = 10;
    y : int = -x + 30 + -x;
begin
    printf("X: %d\n", x);
    printf("Y: %d\n", y);
end

func main -> int
begin
    test1();
    test2();
    test3();
    test4();

    return 0;
end
