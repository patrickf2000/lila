
#OUTPUT
#Correct
#Correct
#Wrong
#Wrong
#Correct
#Wrong
#END

#RET 0

func main -> int
    b1, b2 : byte = 0xA1;
    b3 : byte = 0xA3;
begin
    
    #################################
    # Test 1
    if b1 == 0xA1
        println("Correct");
    else
        println("Wrong");
    end
    
    #################################
    # Test 2
    
    if b1 == b2
        println("Correct");
    else
        println("Wrong");
    end
    
    if b1 == b3
        println("Correct");
    else
        println("Wrong");
    end
    
    #################################
    # Test 3
    
    if b1 == 0xA2
        println("Correct");
    else
        println("Wrong");
    end
    
    #################################
    # Test 4
    
    if 0xA1 == b1
        println("Correct");
    else
        println("Wrong");
    end
    
    if 0xA2 == b1
        println("Correct");
    else
        println("Wrong");
    end
    
    return 0;
end
