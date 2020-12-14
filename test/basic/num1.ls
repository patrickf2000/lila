
#OUTPUT
#Hello!
#X: 15
#END

#RET 5

extern func printf(s:str, ...)

func main -> int
    printf("Hello!\n")

    int x = 15
    printf("X: %d\n", x)
    
    return 5
end

