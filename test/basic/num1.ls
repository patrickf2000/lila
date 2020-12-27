
#OUTPUT
#Hello!
#X: 15
#END

#RET 5

use std.text_io;

func main -> int
    x : int = 15;
begin
    printLn("Hello!");
    printLnStrInt("X: ", x);
    
    return 5;
end

