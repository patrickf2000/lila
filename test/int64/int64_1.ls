
#OUTPUT
#123
#10
#88
#END

#RET 0

use std.text_io;

func main -> int
    x : int64 = 123;
begin
    
    printLnInt(x);
    printLnInt(10);
    
    x = 88;
    printLnInt(x);
    
    return 0;
end
