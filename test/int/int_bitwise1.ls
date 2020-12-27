
#OUTPUT
#X = 4
#x & 5 = 4
#x | 5 = 5
#x ^ 5 = 1
#x << 2 = 16
#x >> 2 = 1
#END

#RET 0

use std.text_io;

func main -> int
    x  : int = 4;
    a1 : int = x & 5;
    a2 : int = x | 5;
    a3 : int = x ^ 5;
    a4 : int = x << 2;
    a5 : int = x >> 2;
begin
    
    printLnStrInt("X = ", x);
    printLnStrInt("x & 5 = ", a1);
    printLnStrInt("x | 5 = ", a2);
    printLnStrInt("x ^ 5 = ", a3);
    printLnStrInt("x << 2 = ", a4);
    printLnStrInt("x >> 2 = ", a5);
    
    return 0;
end

