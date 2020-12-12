
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
    ushort b1 = 0xA1B1
    
    #################################
    # Test 1
    if b1 == 0xA1B1
        puts("Correct")
    else
        puts("Wrong")
    end
    
    ushort b2 = 0xA1B1
    ushort b3 = 0xA3B3
    
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
    
    if b1 == 0xA2B2
        puts("Correct")
    else
        puts("Wrong")
    end
    
    #################################
    # Test 4
    
    if 0xA1B1 == b1
        puts("Correct")
    else
        puts("Wrong")
    end
    
    if 0xA2B2 == b1
        puts("Correct")
    else
        puts("Wrong")
    end
    
    return 0
end
