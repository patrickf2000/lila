
#OUTPUT
#X: -15.400000
#X: -9.300000
#Y: 20.534000
#X: 9.000000
#Y: -9.000000
#X: 10.000000
#Y: 10.123400
#END

#RET 0

extern func printf(s:str, ...)

func test1
    double x = -6.3 + -9.1
    printf("X: %f\n", x)
end

func test2
    double x = -9.3
    double y = 20.534 + x - -9.3
    
    printf("X: %f\n", x)
    printf("Y: %f\n", y)
end

func test3
    double x = 9.0
    double y = -x
    
    printf("X: %f\n", x)
    printf("Y: %f\n", y)
end

func test4
    double x = 10.0
    double y = -x + 30.1234 + -x
    
    printf("X: %f\n", x)
    printf("Y: %f\n", y)
end

func main -> int
    test1()
    test2()
    test3()
    test4()

    return 0
end

