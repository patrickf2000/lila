
#OUTPUT
#Equal
#Not equal
#Less
#Greater
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func main -> int
    x, y : double = 3.14
begin
    
    if x == y
        puts("Equal")
    else
        puts("Not equal")
    end
    
    y = 4.44
    if x != y
        puts("Not equal")
    else
        puts("Equal")
    end
    
    y = 5.5532
    if x == y
        puts("Equal")
    elif x < y
        puts("Less")
    else
        puts("Idk")
    end
    
    y = 1.1000009
    if x > y
        puts("Greater")
    else
        puts("Idk")
    end
    
    return 0
end

