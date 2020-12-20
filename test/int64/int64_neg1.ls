
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
    x : int64 = -6 + -9
begin
    printf("X: %d\n", x)
end

func test2
    x : int64 = 0
    y : int64 = 0
begin
    x = -9
    y = 20 + x - -9
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test3
    x : int64 = 9
    y : int64 = -x
begin    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test4
    x : int64 = 10
    y : int64 = 0
begin
    x = 10
    y = -x + 30 + -x
    
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
