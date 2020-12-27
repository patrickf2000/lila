
#OUTPUT
#Hello!
#X: 15
#X2: 23
#END

#RET 0

use std.text_io;

func main -> int
    x : int = 15;
begin
    printLn("Hello!");

    printLnStrInt("X: ", x);
    printLnStrInt("X2: ", 23);
    
    return 0;
end

