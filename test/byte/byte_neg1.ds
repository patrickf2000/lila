
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
    byte x = -6 + -9
    printf("X: %d\n", x)
end

func test2
    byte x = -9
    byte y = 20 + x - -9
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test3
    byte x = 9
    byte y = -x
    
    printf("X: %d\n", x)
    printf("Y: %d\n", y)
end

func test4
    byte x = 10
    byte y = -x + 30 + -x
    
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
