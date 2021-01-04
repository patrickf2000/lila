
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
    b1 : uint = 100;
    b2 : uint = 100;
    b3 : uint = 300;
begin
    
    #################################
    # Test 1
    if b1 == 100
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
    
    if b1 == 200
        println("Correct");
    else
        println("Wrong");
    end
    
    #################################
    # Test 4
    
    if 100 == b1
        println("Correct");
    else
        println("Wrong");
    end
    
    if 200 == b1
        println("Correct");
    else
        println("Wrong");
    end
    
    return 0;
end

