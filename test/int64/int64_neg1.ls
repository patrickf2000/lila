
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
    int64 x = -6 + -9
    printf("X: %d\n", x)
end

func test2
    int64 x = -9
    int64 y = 20 + x - -9
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test3
    int64 x = 9
    int64 y = -x
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test4
    int64 x = 10
    int64 y = -x + 30 + -x
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func main -> int
    test1()
    test2()
    test3()
    test4()

    return 0
end
