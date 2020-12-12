
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
    float x = 3.14
    
    if x == 3.14
        puts("Equal")
    else
        puts("Not equal")
    end
    
    if x != 4.44
        puts("Not equal")
    else
        puts("Equal")
    end
    
    if x == 5.5532
        puts("Equal")
    elif x < 5.5532
        puts("Less")
    else
        puts("Idk")
    end
    
    if x > 1.1000009
        puts("Greater")
    else
        puts("Idk")
    end
    
    return 0
end

