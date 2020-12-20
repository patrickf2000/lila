
#OUTPUT
#X: 29
#Y: 27
#X: 11
#X: 20
#X: 29
#Y: 37
#END

#RET 0

extern func printf(s:str, ...)

func test1
    x : int = 2 + 9 * 3
    y : int = 2 * 1 + 9 * 2 - -7
begin
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test2
    x : int = 2 * 1 + 9 * 2 - 9
begin
    printf("X: %d\n", x)
end

func test3
    x : int = 2 * 1 + 9 * 2
begin
    printf("X: %d\n", x)
end

func test4
    x : int = 2 + 9 * 3
    y : int = x * 1 + 9 * 2 - 10
begin
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func main -> int
begin
    test1()
    test2()
    test3()
    test4()

    return 0
end

