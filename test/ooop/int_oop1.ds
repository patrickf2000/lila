
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
    int x = 2 + 9 * 3
    printf("X: %d\n", x)
    
    int y = 2 * 1 + 9 * 2 - -7
    printf("Y: %d\n", y)
end

func test2
    int x = 2 * 1 + 9 * 2 - 9
    printf("X: %d\n", x)
end

func test3
    int x = 2 * 1 + 9 * 2
    printf("X: %d\n", x)
end

func test4
    int x = 2 + 9 * 3
    printf("X: %d\n", x)
    
    int y = x * 1 + 9 * 2 - 10
    printf("Y: %d\n", y)
end

func main -> int
    test1()
    test2()
    test3()
    test4()

    return 0
end

