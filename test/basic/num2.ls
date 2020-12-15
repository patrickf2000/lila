
#OUTPUT
#Hello!
#X: 15
#X2: 23
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    printf("Hello!\n")

    int x = 15
    printf("X: %d\n", x)

    printf("X2: %d\n", 23)
    
    return 0
end

