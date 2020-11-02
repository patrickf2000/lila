
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
    uint64 b1 = 100
    
    #################################
    # Test 1
    if b1 == 100
        puts("Correct")
    else
        puts("Wrong")
    end
    
    uint64 b2 = 100
    uint64 b3 = 300
    
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

