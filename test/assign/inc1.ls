
#OUTPUT
#0
#1
#2
#3
#4
#5
#6
#7
#8
#9
#END

#RET 0

use std.io;

func main() -> int
    i : int = 0;
begin
    while i < 10
        printf("%d\n", i);
        i++;
    end
    
    return 0;
end
