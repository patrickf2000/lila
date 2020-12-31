
#OUTPUT
#10
#8
#6
#4
#2
#0
#END

#RET 0

use std.io;

func main() -> int
    i : int = 10;
begin
    while i >= 0
        printf("%d\n", i);
        i -= 2;
    end
    
    return 0;
end
