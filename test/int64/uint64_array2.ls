
#OUTPUT
#X1: 5
#X2: 6
#X3: 1
#X4: 1
#END

#RET 0

use std.text_io;

func main -> int
    numbers : uint64[10] = array;
    base : uint64 = 2;
    x1 : uint64 = 0;
    x2 : uint64 = 0;
    x3 : uint64 = 0;
    x4 : uint64 = 0;
begin
    
    numbers[1] = 3 + base;
    numbers[2] = 3 * base;
    numbers[3] = 3 / base;
    numbers[4] = 3 % base;
    
    x1 = numbers[1];
    x2 = numbers[2];
    x3 = numbers[3];
    x4 = numbers[4];
    
    printLnStrInt("X1: ", x1);
    printLnStrInt("X2: ", x2);
    printLnStrInt("X3: ", x3);
    printLnStrInt("X4: ", x4);
    
    return 0;
end
