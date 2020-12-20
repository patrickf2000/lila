
#OUTPUT
#C: A
#C2: A
#C: b
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    c, c2 : char = 'A'
begin
    printf("C: %c\n", c)
    
    c2 = c
    printf("C2: %c\n", c2)
    
    c = 'b'
    printf("C: %c\n", c)
    
    return 0
end

