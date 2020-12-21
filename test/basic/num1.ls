
#OUTPUT
#Hello!
#X: 15
#END

#RET 5

extern func printf(s:str, ...)

func main -> int
    x : int = 15
begin
    printf("Hello!\n")
    printf("X: %d\n", x)
    
    return 5
end

