
#OUTPUT
#Correct
#Correct
#Wrong
#Wrong
#Correct
#Wrong
#END

#RET 0

extern func puts(s:str, ...)

func main -> int
    b1 : int = 100
    b2 : int = 100
    b3 : int = 300
begin
    
    #################################
    # Test 1
    if b1 == 100
        puts("Correct")
    else
        puts("Wrong")
    end
    
    #################################
    # Test 2
    
    if b1 == b2
        puts("Correct")
    else
        puts("Wrong")
    end
    
    if b1 == b3
        puts("Correct")
    else
        puts("Wrong")
    end
    
    #################################
    # Test 3
    
    if b1 == 200
        puts("Correct")
    else
        puts("Wrong")
    end
    
    #################################
    # Test 4
    
    if 100 == b1
        puts("Correct")
    else
        puts("Wrong")
    end
    
    if 200 == b1
        puts("Correct")
    else
        puts("Wrong")
    end
    
    return 0
end

