
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
    short x = -6 + -9
    printf("X: %d\n", x)
end

func test2
    short x = -9
    short y = 20 + x - -9
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test3
    short x = 9
    short y = -x
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test4
    short x = 10
    short y = -x + 30 + -x
    
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
