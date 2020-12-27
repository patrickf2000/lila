
#OUTPUT
#Also correct!
#Idk!!
#Hello!
#END

#RET 0

use std.text_io;

func main() -> int
    x : int = 3;
begin
    
    if x == 5
        printLn("Correct");
    elif x == 3
        printLn("Also correct!");
        if x == 3
            printLn("Idk!!");
        end
    else
        printLn("Idk");
    end

    printLn("Hello!");
    return 0;
end

