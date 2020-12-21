
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
    b1, b2 : byte = 0xA1;
    b3 : byte = 0xA3;
begin
    
    #################################
    # Test 1
    if b1 == 0xA1
        puts("Correct");
    else
        puts("Wrong");
    end
    
    #################################
    # Test 2
    
    if b1 == b2
        puts("Correct");
    else
        puts("Wrong");
    end
    
    if b1 == b3
        puts("Correct");
    else
        puts("Wrong");
    end
    
    #################################
    # Test 3
    
    if b1 == 0xA2
        puts("Correct");
    else
        puts("Wrong");
    end
    
    #################################
    # Test 4
    
    if 0xA1 == b1
        puts("Correct");
    else
        puts("Wrong");
    end
    
    if 0xA2 == b1
        puts("Correct");
    else
        puts("Wrong");
    end
    
    return 0;
end
