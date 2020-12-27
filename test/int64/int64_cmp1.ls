
#OUTPUT
#Correct
#Correct
#Wrong
#Wrong
#Correct
#Wrong
#END

#RET 0

use std.text_io;

func main -> int
    b1 : int64 = 100;
    b2 : int64 = 100;
    b3 : int64 = 300;
begin
    
    #################################
    # Test 1
    if b1 == 100
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    #################################
    # Test 2
    
    if b1 == b2
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    if b1 == b3
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    #################################
    # Test 3
    
    if b1 == 200
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    #################################
    # Test 4
    
    if 100 == b1
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    if 200 == b1
        printLn("Correct");
    else
        printLn("Wrong");
    end
    
    return 0;
end

