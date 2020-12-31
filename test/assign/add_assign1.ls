
#OUTPUT
#0
#2
#4
#6
#8
#10
#END

#RET 0

use std.io;

func main() -> int
    i : int = 0;
begin
    while i <= 10
        printf("%d\n", i);
        i += 2;
    end
    
    return 0;
end
