
#OUTPUT
#Y: 16
#END

#RET 3

extern func printf(s:str, ...)

func main -> int
    int x = 4
    int y = x * 3 + x
    
    printf("Y: %d\n", y)
    
    return 3
end
