
#OUTPUT
#0
#2
#4
#6
#8
#10
#12
#14
#16
#18
#Done
#END

#RET 0

use std.text_io;

func main -> int
    numbers : int[10] = array;
    i : int = 0;
    x : int = 0;
begin

    while i < 10
        numbers[i] = i * 2;
        i = i + 1;
    end
    
    i = 0;
    while i < 10
        x = numbers[i];
        printLnInt(x);
        i = i + 1;
    end
    
    printLn("Done");
    
    return 0;
end

