
#OUTPUT
#16
#END

#RET 3

use std.text_io;

func main -> int
    x : int = 4;
    y : int = x * 3 + x;
begin
    
    printLnInt(y);
    
    return 3;
end

