
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
    x : uint64 = 4;
    a1 : uint64 = 0;
    a2 : uint64 = 0;
    a3 : uint64 = 0;
    a4 : uint64 = 0;
    a5 : uint64 = 0;
begin
    
    a1 = x & 5;
    a2 = x | 5;
    a3 = x ^ 5;
    a4 = x << 2;
    a5 = x >> 2;
    
    printLnStrInt("X = ", x);
    printLnStrInt("x & 5 = ", a1);
    printLnStrInt("x | 5 = ", a2);
    printLnStrInt("x ^ 5 = ", a3);
    printLnStrInt("x << 2 = ", a4);
    printLnStrInt("x >> 2 = ", a5);
    
    return 0;
end

