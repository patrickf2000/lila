
#OUTPUT
#Syntax Error: Invalid variable.
# -> [14] x = 10;
#
#END

#RET 0

use std.io;

func main -> int
begin
    x = 10;
    printf("%d\n", x);
    
    return 0;
end
