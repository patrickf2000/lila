
#OUTPUT
#10
#9
#8
#7
#6
#5
#4
#3
#2
#1
#0
#END

#RET 0

use std.io;

func main() -> int
    i : int = 10;
begin
    while i >= 0
        printf("%d\n", i);
        i--;
    end
    
    return 0;
end
